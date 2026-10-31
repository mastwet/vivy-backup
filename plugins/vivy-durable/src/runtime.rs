//! One persistent task lifecycle around Rig's execution path.

use rig_harness::prelude::*;
use rig_harness::rig_core::{completion::Section, tool::ToolResult};
use rig_harness::rig_ecs::{
    SaveSystems, WriteLogs,
    session_log::{SessionLog, SessionStore},
    stage::{CommitBarrier, FinalOutcome, PersistOutcomes},
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, io};
mod state;
mod storage;
use state::workspace;
pub use state::{Admission, Control, DurableAgentState, Operation, PendingControl, Status, Task};
use storage::{committed, healthy};

const KEY: &str = "vivy.durable.v1";

#[derive(Default)]
pub struct DurablePlugin;

#[derive(Component, Clone)]
struct InputTask(String);
#[derive(Component, Clone)]
struct Bound {
    task: String,
    operation: String,
    generation: u64,
}
#[derive(Component)]
struct Held(Entity);
#[derive(Component)]
struct InitializedTurn;

#[derive(Deserialize, schemars::JsonSchema)]
struct Resolution {
    operation: String,
    generation: u64,
    evidence: String,
}

fn uncertain(state: &DurableAgentState, task: &str) -> bool {
    state
        .operations
        .iter()
        .any(|o| o.task == task && o.phase == "unknown" && o.resolution.is_none())
}
fn inspection_tool(name: &str) -> bool {
    [
        "read",
        "list",
        "search",
        "task_control",
        "resolve_operation",
    ]
    .contains(&name)
}

impl Plugin for DurablePlugin {
    fn build(&self, app: &mut App) {
        storage::install(app);
        app.register_type::<DurableAgentState>()
            .init_resource::<SavedKeys>();
        app.world_mut()
            .resource_mut::<SavedKeys>()
            .require(std::any::type_name::<DurableAgentState>(), KEY, 1)
            .expect("unique durable schema");
        app.add_observer(|added:On<Add<Agent>>, mut commands:Commands| {
            commands.entity(added.entity).insert(PersistOutcomes).insert_if_new(DurableAgentState::default());
        })
        .add_systems(First,(storage::initialize,recover_inputs,withdraw_cancelled,acknowledge,continue_pending,requeue,admit_inputs,gate).chain())
        .add_systems(AgentPass,(recover_turns,recover_inputs,withdraw_cancelled,admit_inputs,replay).chain().in_set(GateSystems::FromRecord))
        .add_systems(AgentPass,gate.in_set(GateSystems::After))
        .add_systems(AgentPass,context.in_set(PrepareSystems::AdaptOutermost))
        .add_systems(AgentPass,capture.in_set(Stage::Finalize))
        .add_systems(Last,boundary.in_set(SaveSystems))
        .add_systems(Last,storage::receipts.after(WriteLogs))
        .add_observer(cancelled)
        .add_observer(|event:On<Exit>,agents:Query<Entity,With<DurableAgentState>>,mut commands:Commands| {
            if event.0==ExitReason::Signal {
                for agent in &agents {commands.trigger(StopTurn::new(agent));}
            }
        })
        .add_observer(|event:On<Add<Started>>,calls:Query<(&ForAgent,&Bound)>,mut agents:Query<&mut DurableAgentState>| {
            if let Ok((of,binding))=calls.get(event.entity) {
                if let Ok(mut state)=agents.get_mut(of.0) {
                    if let Some(op)=state.operations.iter_mut().find(|o|o.id==binding.operation) {
                        if op.phase=="prepared" {op.phase="dispatched".into();state.bump();}
                    }
                }
            }
        })
        .add_observer(|event:On<Add<Outcome>>, inputs:Query<(&ForAgent,&InputTask,&Outcome),With<Incoming>>, mut agents:Query<&mut DurableAgentState>| {
            if let Ok((of,input,_))=inputs.get(event.entity) {
                if let Ok(mut state)=agents.get_mut(of.0) {
                    if let Some(task)=state.task_mut(&input.0) {if !task.delivered {task.delivered=true;state.bump();}}
                }
            }
        })
        .add_open_tool("task_control","Choose continue, switch, defer or finish_silently for an admitted task. Use its exact generation from durable context. This changes task state after this call and its sibling batch are saved.",ToolOptions::default(), control_called)
        .add_open_tool("resolve_operation","Resolve an unknown external effect using evidence from non-mutating inspection or an explicit user decision. Never guess success or resend the original operation.",ToolOptions::default(),|event:On<ToolCalled<Resolution>>,mut commands:Commands| {
            let (call,agent,op,generation,evidence)=(event.call,event.agent,event.args.operation.clone(),event.args.generation,event.args.evidence.clone());
            commands.queue(move |world:&mut World| {
                let outcome=match resolve(world,agent,&op,generation,&evidence) {
                    Ok(_)=>ToolResult::success(rig_harness::rig_core::tool::ToolOutput::text("resolution evidence recorded")).into(),
                    Err(error)=>Outcome::refused(error.to_string()),
                };
                world.entity_mut(call).insert(outcome);
            });
        });
    }
}

fn invalid(message: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, message.into())
}
fn digest(value: &impl Serialize) -> String {
    format!(
        "{:x}",
        Sha256::digest(serde_json::to_vec(value).expect("serializable durable data"))
    )
}
fn now_ms() -> u64 {
    rig_harness::rig_core::now_ms()
}

/// Inspect pending local state. `admission.committed` or a matching committed
/// revision is required before acknowledging persistence to a caller.
pub fn inspect(world: &World, agent: Entity) -> Option<&DurableAgentState> {
    world.get::<DurableAgentState>(agent)
}

/// Idempotent local admission; a returned pending receipt is not yet durable.
pub fn submit(
    world: &mut World,
    agent: Entity,
    source: &str,
    key: &str,
    text: &str,
) -> io::Result<Admission> {
    let input = Incoming::new(text, DeliveryMode::Steer);
    let receipt = register(world, agent, source, key, input.clone())?;
    let exists = world
        .query::<(&ForAgent, &InputTask)>()
        .iter(world)
        .any(|(of, t)| of.0 == agent && t.0 == receipt.task);
    if !exists
        && world
            .get::<DurableAgentState>(agent)
            .and_then(|s| s.task(&receipt.task))
            .is_some_and(|t| !t.delivered)
    {
        world.spawn((input, ForAgent(agent), InputTask(receipt.task.clone())));
    }
    world.resource::<Wake>().wake();
    Ok(receipt)
}

fn register(
    world: &mut World,
    agent: Entity,
    source: &str,
    key: &str,
    input: Incoming,
) -> io::Result<Admission> {
    if source.is_empty() || key.is_empty() {
        return Err(invalid("input source and request key must be nonempty"));
    }
    if !healthy(world) {
        return Err(io::Error::other(
            "durable session is not writable; reopen and reconcile",
        ));
    }
    let hash = digest(&input);
    let state = world
        .get::<DurableAgentState>(agent)
        .ok_or_else(|| invalid("not a durable agent"))?;
    if let Some(task) = state
        .tasks
        .iter()
        .find(|t| t.source == source && t.request_key == key)
    {
        if task.digest != hash {
            return Err(invalid(
                "request key already admitted with different content",
            ));
        }
        return Ok(Admission {
            task: task.id.clone(),
            revision: task.admission_revision,
            committed: committed(world, agent)
                .is_some_and(|s| s.task(&task.id).is_some_and(|t| t.digest == hash)),
        });
    }
    let mut state = world.get_mut::<DurableAgentState>(agent).unwrap();
    state.bump();
    let id = format!("task:{}", state.tasks.len() + 1);
    let revision = state.revision;
    state.tasks.push(Task {
        id: id.clone(),
        source: source.into(),
        request_key: key.into(),
        input,
        digest: hash,
        status: Status::Queued,
        generation: 0,
        next_step: 0,
        delivered: false,
        delivery_ready: false,
        admission_revision: revision,
        outcome: None,
    });
    Ok(Admission {
        task: id,
        revision,
        committed: false,
    })
}

/// Immediate physical stop; the cancellation is acknowledged only after a later
/// checkpoint contains the incremented generation and Cancelled status.
pub fn cancel(world: &mut World, agent: Entity, task: &str, generation: u64) -> io::Result<u64> {
    let mut state = world
        .get_mut::<DurableAgentState>(agent)
        .ok_or_else(|| invalid("not a durable agent"))?;
    let active = state.active.as_deref() == Some(task);
    let record = state
        .task_mut(task)
        .ok_or_else(|| invalid("unknown task"))?;
    if record.generation != generation {
        return Err(invalid("stale task generation"));
    }
    record.generation += 1;
    record.status = Status::Cancelled;
    if state
        .control
        .as_ref()
        .is_some_and(|c| c.origin == task || c.request.task == task)
    {
        state.control = None;
    }
    if active {
        state.decision_pending = false;
    }
    state.bump();
    let revision = state.revision;
    drop(state);
    withdraw_cancelled(world);
    if active {
        world.trigger(StopTurn::new(agent));
    }
    world.resource::<Wake>().wake();
    Ok(revision)
}

/// Record external evidence for an uncertain operation. This never replays the
/// original tool. Normal execution continues with a fresh model decision.
pub fn resolve(
    world: &mut World,
    agent: Entity,
    operation: &str,
    generation: u64,
    evidence: &str,
) -> io::Result<u64> {
    if evidence.trim().is_empty() {
        return Err(invalid("resolution requires evidence"));
    }
    let mut state = world
        .get_mut::<DurableAgentState>(agent)
        .ok_or_else(|| invalid("not a durable agent"))?;
    let current = state
        .operations
        .iter()
        .find(|o| o.id == operation)
        .ok_or_else(|| invalid("unknown operation"))?;
    if !state.task(&current.task).is_some_and(|t| {
        t.generation == generation
            && !matches!(
                t.status,
                Status::Cancelled | Status::Completed | Status::Failed
            )
    }) {
        return Err(invalid(
            "resolution belongs to a cancelled or ended task generation",
        ));
    }
    let op = state
        .operations
        .iter_mut()
        .find(|o| o.id == operation)
        .unwrap();
    if op.generation != generation || op.phase != "unknown" {
        return Err(invalid(
            "operation is not an unresolved operation of this generation",
        ));
    }
    op.resolution = Some(evidence.into());
    let task = op.task.clone();
    if !uncertain(&state, &task) {
        if let Some(task) = state.task_mut(&task) {
            task.status = Status::Running;
        }
    }
    state.bump();
    let revision = state.revision;
    drop(state);
    world.resource::<Wake>().wake();
    Ok(revision)
}

fn hold(world: &mut World, call: Entity) {
    if world.get::<Held>(call).is_none() {
        let hold = world.spawn(HoldOf(call)).id();
        world.entity_mut(call).insert(Held(hold));
    }
}
fn release(world: &mut World, call: Entity) {
    if let Some(hold) = world.get::<Held>(call).map(|h| h.0) {
        world.despawn(hold);
        world.entity_mut(call).remove::<Held>();
    }
}
fn decide(world: &mut World, call: Entity, outcome: Outcome) {
    if let Some(mut decision) = world.get_mut::<Decision>(call) {
        decision.decide(Decider::plugin("durable"), outcome);
    }
}

fn withdraw_cancelled(world: &mut World) {
    let inputs:Vec<_>=world.query_filtered::<(Entity,&ForAgent,&InputTask),(With<Incoming>,Without<Outcome>,Without<Settled>)>()
        .iter(world).filter(|(_,of,t)|world.get::<DurableAgentState>(of.0).and_then(|s|s.task(&t.0)).is_some_and(|t|t.status==Status::Cancelled))
        .map(|(e,_,_)|e).collect();
    for input in inputs {
        // Inboxes excludes a decided input immediately, including after Started.
        decide(
            world,
            input,
            Outcome::refused("this admitted task was cancelled before delivery"),
        );
        release(world, input);
    }
}

fn continue_pending(world: &mut World) {
    if !healthy(world) {
        return;
    }
    let agents: Vec<_> = world
        .query_filtered::<Entity, (With<DurableAgentState>, Without<ActiveTurn>)>()
        .iter(world)
        .collect();
    for agent in agents {
        let next = world
            .get::<DurableAgentState>(agent)
            .filter(|s| s.decision_pending && s.active.is_none())
            .and_then(|s| {
                s.tasks
                    .iter()
                    .find(|t| t.status == Status::Queued && t.delivered)
            })
            .map(|t| t.id.clone());
        let Some(next) = next else {
            continue;
        };
        if !committed(world, agent).is_some_and(|s| {
            s.decision_pending
                && s.task(&next)
                    .is_some_and(|t| t.status == Status::Queued && t.delivered)
        }) {
            continue;
        }
        let mut state = world.get_mut::<DurableAgentState>(agent).unwrap();
        state.active = Some(next.clone());
        state.task_mut(&next).unwrap().status = Status::Running;
        state.decision_pending = false;
        state.bump();
        drop(state);
        // Its input is already in the conversation. Start only a foreground
        // decision turn through Rig; do not deliver the same message again.
        let turn = world.spawn(TurnOf(agent)).id();
        world.spawn((TurnCall, CallOf(turn), ForAgent(agent)));
    }
}

fn requeue(world: &mut World) {
    let missing: Vec<_> = world
        .query::<(Entity, &DurableAgentState)>()
        .iter(world)
        .flat_map(|(agent, state)| {
            state
                .tasks
                .iter()
                .filter(|t| !t.delivered && matches!(t.status, Status::Queued | Status::Running))
                .map(move |t| (agent, t.id.clone(), t.input.clone()))
        })
        .collect();
    for (agent, task, input) in missing {
        let exists = world
            .query::<(&ForAgent, &InputTask)>()
            .iter(world)
            .any(|(of, t)| of.0 == agent && t.0 == task);
        if !exists {
            world.spawn((input, ForAgent(agent), InputTask(task)));
        }
    }
}

fn recover_inputs(world: &mut World) {
    let inputs: Vec<_> = world
        .query_filtered::<(Entity, &ForAgent, &Incoming), (With<Restored>, Without<InputTask>)>()
        .iter(world)
        .map(|(e, of, i)| (e, of.0, digest(i)))
        .collect();
    for (call, agent, hash) in inputs {
        let bound: Vec<_> = world
            .query::<(&ForAgent, &InputTask)>()
            .iter(world)
            .filter(|(of, _)| of.0 == agent)
            .map(|(_, t)| t.0.clone())
            .collect();
        let task = world
            .get::<DurableAgentState>(agent)
            .and_then(|s| {
                s.tasks
                    .iter()
                    .find(|t| !t.delivered && t.digest == hash && !bound.contains(&t.id))
            })
            .map(|t| t.id.clone());
        if let Some(task) = task {
            world.entity_mut(call).insert(InputTask(task));
        }
    }
}

fn recover_turns(world: &mut World) {
    let turns: Vec<_> = world
        .query_filtered::<(Entity, &TurnOf), Without<InitializedTurn>>()
        .iter(world)
        .map(|(e, of)| (e, of.0))
        .collect();
    for (turn, agent) in turns {
        if let Some(state) = world.get::<DurableAgentState>(agent) {
            let cancelled = state
                .active
                .as_ref()
                .and_then(|id| state.task(id))
                .is_some_and(|t| t.status == Status::Cancelled);
            world.entity_mut(turn).insert((CarryOn, InitializedTurn));
            if cancelled {
                world.trigger(StopTurn::new(agent));
                world.flush();
            }
        }
    }
    // Rig removes CarryOn when it accepts restored work; keep it for host exit.
    let turns: Vec<_> = world
        .query_filtered::<Entity, (With<InitializedTurn>, Without<CarryOn>)>()
        .iter(world)
        .collect();
    for turn in turns {
        world.entity_mut(turn).insert(CarryOn);
    }
}

fn admit_inputs(world: &mut World) {
    let inputs:Vec<_>=world.query_filtered::<(Entity,&ForAgent,&Effect,&Incoming,Option<&InputTask>),(Without<Started>,Without<Outcome>,Without<Settled>)>()
        .iter(world).map(|(e,of,fx,i,t)|(e,of.0,fx.id().to_string(),i.clone(),t.map(|t|t.0.clone()))).collect();
    for (call, agent, effect, input, task) in inputs {
        if world.get::<DurableAgentState>(agent).is_none() {
            continue;
        }
        hold(world, call);
        let task = match task {
            Some(task) => task,
            None => {
                if input.command().is_some_and(|s| {
                    ["fork", "rewind", "clone", "import", "compact"]
                        .contains(&s.split_whitespace().next().unwrap_or(""))
                }) {
                    decide(
                        world,
                        call,
                        Outcome::refused("history mutation is unsupported in a durable session"),
                    );
                    release(world, call);
                    continue;
                }
                // Face deliveries have no remote dedup key; assign a persisted
                // local sequence. Typed callers use submit's source/request key.
                let key = format!(
                    "{}:{}",
                    world.get::<DurableAgentState>(agent).unwrap().tasks.len() + 1,
                    effect
                );
                let Ok(receipt) = register(world, agent, "face", &key, input) else {
                    continue;
                };
                world
                    .entity_mut(call)
                    .insert(InputTask(receipt.task.clone()));
                receipt.task
            }
        };
        if world
            .get::<DurableAgentState>(agent)
            .and_then(|s| s.task(&task))
            .is_some_and(|t| {
                matches!(
                    t.status,
                    Status::Cancelled | Status::Completed | Status::Failed
                )
            })
        {
            decide(
                world,
                call,
                Outcome::refused("this admitted task has already ended"),
            );
            release(world, call);
            continue;
        }
        if !healthy(world) || !committed(world, agent).is_some_and(|s| s.task(&task).is_some()) {
            continue;
        }
        // Rig accepts it into the persisted inbox while work runs. It delivers
        // only at the next model boundary; gate closes unstarted siblings first.
        let mut state = world.get_mut::<DurableAgentState>(agent).unwrap();
        if state.active.is_none() {
            state.active = Some(task.clone());
            state.task_mut(&task).unwrap().status = Status::Running;
        }
        if !state.task(&task).unwrap().delivery_ready {
            state.task_mut(&task).unwrap().delivery_ready = true;
            state.bump();
        }
        drop(state);
        // Delivery itself is held until its disposition and active task are saved.
        if committed(world, agent).is_some_and(|s| s.task(&task).is_some_and(|t| t.delivery_ready))
        {
            release(world, call);
        }
    }
}

fn route(world: &World, model: &Adapted) -> String {
    digest(&(
        model.0.spec.provider,
        &model.0.spec.id,
        world
            .get_resource::<vivy_gateway::ProvidersState>()
            .map(|s| &s.config),
    ))
}

fn replay(world: &mut World) {
    let restored: Vec<_> = world
        .query_filtered::<(
            Entity,
            &ForAgent,
            Option<&ToolRequest>,
            Option<&ModelRequest>,
            Option<&Adapted>,
        ), (
            With<Restored>,
            Without<Bound>,
            Without<Outcome>,
            Or<(With<ToolRequest>, With<ModelRequest>)>,
        )>()
        .iter(world)
        .map(|(e, of, t, m, a)| (e, of.0, t.cloned(), m.cloned(), a.cloned()))
        .collect();
    for (call, agent, tool, model, adapted) in restored {
        let Some(state) = world.get::<DurableAgentState>(agent).cloned() else {
            continue;
        };
        let found = if let Some(tool) = &tool {
            state.operations.iter().rev().find(|o| {
                o.call_id.as_deref() == Some(serde_json::to_string(&tool.call.id).unwrap().as_str())
            })
        } else {
            state
                .operations
                .iter()
                .rev()
                .find(|o| o.kind == "model" && !o.published)
        };
        let Some(op) = found.cloned() else {
            continue;
        };
        hold(world, call);
        world.entity_mut(call).insert(Bound {
            task: op.task.clone(),
            operation: op.id.clone(),
            generation: op.generation,
        });
        if let Some(result) = &op.outcome {
            if let Ok(Some(outcome)) = FinalOutcome::from_value(result.clone())
                .and_then(|s| s.effect_result().map_err(io::Error::other))
            {
                if model.is_some() && outcome.0.is_err() && op.retry_at_ms > 0 {
                    let delay =
                        std::time::Duration::from_millis(op.retry_at_ms.saturating_sub(now_ms()));
                    world.entity_mut(call).insert(RetryVerdict(
                        rig_harness::rig_ecs::retry::Verdict::Retry(delay),
                    ));
                    if let Some(turn) = world.get::<CallOf>(call).map(|of| of.0) {
                        world
                            .entity_mut(turn)
                            .insert(rig_harness::rig_ecs::turn::Recovery {
                                retries: op.attempt,
                            });
                    }
                }
                decide(world, call, outcome);
                world.entity_mut(call).remove::<Interrupted>();
            }
        } else if tool.is_some() {
            let mut state = world.get_mut::<DurableAgentState>(agent).unwrap();
            state
                .operations
                .iter_mut()
                .find(|o| o.id == op.id)
                .unwrap()
                .phase = "unknown".into();
            state.task_mut(&op.task).unwrap().status = Status::Suspended;
            state.bump();
            drop(state);
            decide(world,call,ToolResult::skipped("Unknown external outcome after restart; inspect actual effects and resolve this operation. It was not replayed.").into());
            world.entity_mut(call).remove::<Interrupted>();
        } else if model.is_some() {
            let compatible = adapted
                .as_ref()
                .is_some_and(|a| route(world, a) == op.route)
                && state.workspace == workspace();
            if !compatible {
                let mut state = world.get_mut::<DurableAgentState>(agent).unwrap();
                state.task_mut(&op.task).unwrap().status = Status::Suspended;
                state.bump();
                drop(state);
                if let Some(turn) = world.get::<CallOf>(call).map(|of| of.0) {
                    world.entity_mut(turn).insert(TurnOutcome::Stopped(Some("durable recovery suspended: workspace or provider route changed; submit a new decision instead of replaying the old request".into())));
                }
                continue;
            }
            if let Ok(request) = serde_json::from_value(op.request.clone()) {
                if let Some(mut request_now) = world.get_mut::<ModelRequest>(call) {
                    request_now.request = std::sync::Arc::new(request);
                    request_now.prompt = None;
                    request_now.history = None;
                }
            }
            let policy = rig_harness::rig_ecs::turn::RETRY_POLICY;
            if op.attempt >= policy.max_retries {
                decide(
                    world,
                    call,
                    Outcome::refused("interrupted model attempts exhausted Rig's retry policy"),
                );
            } else {
                let mut state = world.get_mut::<DurableAgentState>(agent).unwrap();
                let old = state.operations.iter_mut().find(|o| o.id == op.id).unwrap();
                old.published = true;
                let mut next = op.clone();
                next.attempt += 1;
                next.id = format!("{}/{}/{}", op.task, op.step, next.attempt);
                next.retry_at_ms = now_ms() + policy.backoff(op.attempt).as_millis() as u64;
                next.phase = "prepared".into();
                let binding = Bound {
                    task: next.task.clone(),
                    operation: next.id.clone(),
                    generation: next.generation,
                };
                state.operations.push(next);
                state.bump();
                drop(state);
                world
                    .entity_mut(call)
                    .insert(binding)
                    .remove::<Interrupted>();
                if let Some(turn) = world.get::<CallOf>(call).map(|of| of.0) {
                    world
                        .entity_mut(turn)
                        .insert(rig_harness::rig_ecs::turn::Recovery {
                            retries: op.attempt + 1,
                        });
                }
                world
                    .entity_mut(call)
                    .insert(rig_harness::clock::WakeAt::after(
                        policy.backoff(op.attempt),
                    ));
            }
        }
    }
}

fn gate(world: &mut World) {
    let mut calls: Vec<_> = world
        .query_filtered::<(
            Entity,
            &ForAgent,
            &Effect,
            Option<&ToolRequest>,
            Option<&ModelRequest>,
            Option<&Adapted>,
        ), (
            With<Admitted>,
            Without<Started>,
            Without<Outcome>,
            Without<Settled>,
            Without<SupersededBy>,
        )>()
        .iter(world)
        .filter(|(_, _, _, t, m, _)| t.is_some() || m.is_some())
        .map(|(e, of, fx, t, m, a)| {
            (
                e,
                of.0,
                fx.id().as_u64(),
                t.cloned(),
                m.cloned(),
                a.cloned(),
            )
        })
        .collect();
    calls.sort_by_key(|c| c.2);
    let mut first_tool = BTreeMap::new();
    for (call, agent, _, tool, model, adapted) in calls {
        let Some(mut state) = world.get::<DurableAgentState>(agent).cloned() else {
            continue;
        };
        hold(world, call);
        if !healthy(world) {
            continue;
        }
        let Some(task_id) = state.active.clone() else {
            continue;
        };
        let task = state.task(&task_id).unwrap().clone();
        let reconciling = task.status == Status::Suspended && uncertain(&state, &task_id);
        let pending = state
            .tasks
            .iter()
            .any(|t| !t.delivered && t.status == Status::Queued);
        let changing = state
            .control
            .as_ref()
            .is_some_and(|c| c.request.action != "continue");
        let previously_bound = world.get::<Bound>(call).cloned();
        if previously_bound
            .as_ref()
            .is_some_and(|b| b.task != task_id || b.generation != task.generation)
        {
            decide(
                world,
                call,
                Outcome::refused("not_run: stale task generation"),
            );
            continue;
        }
        if tool.is_some() {
            let other=world.query_filtered::<(Entity,&ForAgent),(With<ToolRequest>,With<Started>,Without<Settled>)>().iter(world).any(|(e,of)|of.0==agent && e!=call);
            if other || first_tool.insert(agent, call).is_some() {
                continue;
            }
        }
        let blocked = matches!(
            task.status,
            Status::Suspended | Status::Cancelled | Status::Completed | Status::Failed
        ) || state.workspace != workspace();
        let ending = world
            .get::<CallOf>(call)
            .is_some_and(|of| world.get::<TurnOutcome>(of.0).is_some());
        if let Some(tool) = &tool {
            if pending
                || changing
                || (blocked && !(reconciling && inspection_tool(tool.call.function.name.as_str())))
            {
                decide(world,call,ToolResult::skipped("not_run: yielding at a tool boundary; issue a fresh call after deciding what to do next").into());
            }
        } else if ending {
            decide(world, call, Outcome::refused("not_run: the task has ended"));
        } else if (blocked && !reconciling) || pending || changing {
            continue;
        }
        if world
            .get::<Decision>(call)
            .is_some_and(Decision::is_decided)
            && previously_bound.is_some()
        {
            continue;
        }
        let (kind, request, route, call_id) = if let Some(tool) = &tool {
            (
                "tool",
                serde_json::to_value(&tool.call).unwrap(),
                "unsafe/v1".to_owned(),
                Some(serde_json::to_string(&tool.call.id).unwrap()),
            )
        } else if let (Some(model), Some(adapted)) = (&model, &adapted) {
            (
                "model",
                serde_json::to_value(adapted.sent(model)).unwrap(),
                route(world, adapted),
                None,
            )
        } else {
            continue;
        };
        let hash = digest(&(&request, &route));
        let index = previously_bound
            .as_ref()
            .and_then(|b| state.operations.iter().position(|o| o.id == b.operation));
        let index = if let Some(index) = index {
            if state.operations[index].digest != hash && state.operations[index].outcome.is_none() {
                state.operations[index].request = request.clone();
                state.operations[index].digest = hash.clone();
                state.operations[index].route = route.clone();
                state.bump();
            }
            index
        } else {
            let attempt = if kind == "model" {
                world
                    .get::<CallOf>(call)
                    .and_then(|of| world.get::<rig_harness::rig_ecs::turn::Recovery>(of.0))
                    .map_or(0, |r| r.retries)
            } else {
                0
            };
            let previous = state
                .operations
                .iter()
                .rev()
                .find(|o| o.task == task_id && o.kind == "model");
            let step = if attempt > 0 {
                previous.map_or(task.next_step, |o| o.step)
            } else {
                task.next_step
            };
            if step == task.next_step {
                state.task_mut(&task_id).unwrap().next_step += 1;
            }
            let id = format!("{task_id}/{step}/{attempt}");
            state.operations.push(Operation {
                id: id.clone(),
                task: task_id.clone(),
                generation: task.generation,
                kind: kind.into(),
                call_id,
                request,
                digest: hash.clone(),
                route,
                step,
                attempt,
                retry_at_ms: 0,
                phase: "prepared".into(),
                outcome: None,
                published: false,
                resolution: None,
            });
            state.bump();
            world.entity_mut(call).insert(Bound {
                task: task_id.clone(),
                operation: id,
                generation: task.generation,
            });
            state.operations.len() - 1
        };
        let operation = state.operations[index].clone();
        world.entity_mut(agent).insert(state.clone());
        probe("prepared", &state);
        if world
            .get::<Decision>(call)
            .is_some_and(Decision::is_decided)
        {
            continue;
        }
        let ready = committed(world, agent).is_some_and(|c| {
            c.revision >= state.revision
                && c.operations.iter().any(|o| {
                    o.id == operation.id && o.digest == hash && o.generation == task.generation
                })
        }) && operation.retry_at_ms <= now_ms();
        if ready && (!blocked || reconciling) {
            probe("dispatch", &state);
            release(world, call);
        }
    }
}

fn capture(world: &mut World) {
    // Calls refused/cancelled before the intent gate still need a saved terminal
    // record. Their representation is never permission to invoke a handler.
    let unbound: Vec<_> = world
        .query_filtered::<(Entity, &ForAgent, Option<&ToolRequest>), (
            With<Effect>,
            With<FinalOutcome>,
            Without<Bound>,
            Without<Incoming>,
            Without<Settled>,
        )>()
        .iter(world)
        .map(|(e, of, t)| (e, of.0, t.cloned()))
        .collect();
    for (call, agent, tool) in unbound {
        let Some(mut state) = world.get_mut::<DurableAgentState>(agent) else {
            continue;
        };
        let Some(task) = state.active.clone() else {
            continue;
        };
        let record = state.task_mut(&task).unwrap();
        let step = record.next_step;
        record.next_step += 1;
        let generation = record.generation;
        let id = format!("{task}/{step}/0");
        state.operations.push(Operation {
            id: id.clone(),
            task: task.clone(),
            generation,
            kind: if tool.is_some() { "tool" } else { "model" }.into(),
            call_id: tool
                .as_ref()
                .map(|t| serde_json::to_string(&t.call.id).unwrap()),
            request: json!(null),
            digest: digest(&Value::Null),
            route: "not_run".into(),
            step,
            attempt: 0,
            retry_at_ms: 0,
            phase: "not_run".into(),
            outcome: None,
            published: false,
            resolution: None,
        });
        state.bump();
        drop(state);
        world.entity_mut(call).insert(Bound {
            task,
            operation: id,
            generation,
        });
    }
    let results: Vec<_> = world
        .query_filtered::<(
            Entity,
            &ForAgent,
            &FinalOutcome,
            Option<&Bound>,
            Option<&TurnOutcome>,
        ), Without<Settled>>()
        .iter(world)
        .map(|(e, of, v, b, t)| (e, of.0, v.value().clone(), b.cloned(), t.cloned()))
        .collect();
    for (call, agent, value, bound, turn) in results {
        let retry_at = world.get::<RetryVerdict>(call).and_then(|r| match &r.0 {
            rig_harness::rig_ecs::retry::Verdict::Retry(delay) => {
                Some(now_ms() + delay.as_millis() as u64)
            }
            _ => None,
        });
        let Some(mut state) = world.get_mut::<DurableAgentState>(agent) else {
            continue;
        };
        if let Some(bound) = bound {
            if let Some(op) = state
                .operations
                .iter_mut()
                .find(|o| o.id == bound.operation)
            {
                if op.outcome.as_ref() != Some(&value) {
                    if op.outcome.is_none() {
                        op.retry_at_ms = retry_at.unwrap_or(0);
                    }
                    op.outcome = Some(value);
                    if op.phase != "unknown" {
                        op.phase = "settled".into();
                    }
                    state.bump();
                }
            }
        } else if let Some(turn) = turn {
            if let Some(active) = state.active.clone() {
                let task = state.task_mut(&active).unwrap();
                if task.outcome.as_ref() != Some(&value) {
                    task.outcome = Some(value);
                    if task.status != Status::Cancelled && task.status != Status::Suspended {
                        task.status = match turn {
                            TurnOutcome::Answered(_) => Status::Completed,
                            TurnOutcome::Failed(_) => Status::Failed,
                            TurnOutcome::Stopped(_) => Status::Suspended,
                        };
                    }
                    state.bump();
                }
            }
        }
        probe("received", &state);
    }
}

fn acknowledge(world: &mut World) {
    if !healthy(world) {
        return;
    }
    let results: Vec<_> = world
        .query_filtered::<(Entity, &ForAgent, &FinalOutcome, Option<&Bound>), Without<Settled>>()
        .iter(world)
        .map(|(e, of, v, b)| (e, of.0, v.clone(), b.cloned()))
        .collect();
    for (call, agent, value, bound) in results {
        if bound.is_some() {
            let pending: Vec<_> = world
                .get::<DurableAgentState>(agent)
                .into_iter()
                .flat_map(|s| s.tasks.iter())
                .filter(|t| !t.delivered && t.status == Status::Queued)
                .map(|t| t.id.clone())
                .collect();
            let accepted: Vec<_> = world
                .query_filtered::<(&ForAgent, &InputTask), With<Started>>()
                .iter(world)
                .filter(|(of, _)| of.0 == agent)
                .map(|(_, t)| t.0.clone())
                .collect();
            if pending.iter().any(|id| !accepted.contains(id)) {
                continue;
            }
        }
        let Some(state) = committed(world, agent) else {
            continue;
        };
        let ready = if let Some(bound) = &bound {
            state.operations.iter().any(|o| {
                o.id == bound.operation
                    && o.generation == bound.generation
                    && o.outcome.as_ref() == Some(value.value())
            })
        } else {
            state
                .active
                .as_ref()
                .and_then(|id| state.task(id))
                .is_some_and(|t| t.outcome.as_ref() == Some(value.value()))
        };
        if ready {
            world
                .entity_mut(call)
                .insert(CommitBarrier::acknowledged(&value));
            release(world, call);
        }
    }
    apply_control(world);
}

fn boundary(world: &mut World) {
    let delivered: Vec<_> = world
        .query_filtered::<(&ForAgent, &InputTask), With<Settled>>()
        .iter(world)
        .map(|(of, t)| (of.0, t.0.clone()))
        .collect();
    for (agent, id) in delivered {
        if let Some(mut state) = world.get_mut::<DurableAgentState>(agent) {
            if let Some(task) = state.task_mut(&id) {
                if !task.delivered {
                    task.delivered = true;
                    state.bump();
                }
            }
        }
    }
    let published: Vec<_> = world
        .query_filtered::<(&ForAgent, &Bound), With<Settled>>()
        .iter(world)
        .map(|(of, b)| (of.0, b.operation.clone()))
        .collect();
    for (agent, id) in published {
        if let Some(mut state) = world.get_mut::<DurableAgentState>(agent) {
            if let Some(op) = state.operations.iter_mut().find(|o| o.id == id) {
                if !op.published {
                    op.published = true;
                    state.bump();
                }
            }
        }
    }
    let mut agents = world.query::<(&Conversation, &mut DurableAgentState, Option<&ActiveTurn>)>();
    for (conversation, mut state, turn) in agents.iter_mut(world) {
        if state.conversation_revision != conversation.revision() {
            state.conversation_revision = conversation.revision();
            state.bump();
        }
        if turn.is_none()
            && state
                .active
                .as_ref()
                .and_then(|id| state.task(id))
                .is_some_and(|t| {
                    matches!(
                        t.status,
                        Status::Completed | Status::Failed | Status::Cancelled | Status::Suspended
                    )
                })
        {
            state.decision_pending = state
                .active
                .as_ref()
                .and_then(|id| state.task(id))
                .is_some_and(|t| t.status != Status::Cancelled)
                && state
                    .tasks
                    .iter()
                    .any(|t| t.status == Status::Queued && t.delivered);
            state.active = None;
            state.bump();
        }
    }
}

fn context(
    mut calls: Query<(&ForAgent, &mut ModelRequest), Added<Admitted>>,
    agents: Query<&DurableAgentState>,
) {
    for (of, mut call) in &mut calls {
        let Ok(state) = agents.get(of.0) else {
            continue;
        };
        let tasks: Vec<_> = state
            .tasks
            .iter()
            .map(|t| json!({"id":t.id,"generation":t.generation,"status":t.status}))
            .collect();
        let unknown: Vec<_> = state
            .operations
            .iter()
            .filter(|o| o.phase == "unknown")
            .map(|o| json!({"operation":o.id,"generation":o.generation,"resolution":o.resolution}))
            .collect();
        let reconciling = state.active.as_ref().is_some_and(|id| uncertain(state, id));
        if reconciling {
            call.request_mut()
                .tools
                .retain(|tool| inspection_tool(tool.name.as_str()));
        }
        if let Some(prompt) = &mut call.prompt {
            prompt.sections.push(Section::new("Durable tasks",format!("Active task: {:?}. Tasks: {}. Unknown effects: {}. You choose how to handle pending inputs using task_control. Tools are sequential; not_run calls require a new decision. Unknown effects require non-mutating inspection and resolve_operation with evidence; do not resend them. No coding-session or subagent delegation is available.",state.active,json!(tasks),json!(unknown))));
        }
    }
}

fn cancelled(event: On<StopTurn>, mut agents: Query<&mut DurableAgentState>) {
    let Ok(mut state) = agents.get_mut(event.entity) else {
        return;
    };
    if let Some(active) = state.active.clone() {
        let task = state.task_mut(&active).unwrap();
        if task.status != Status::Cancelled {
            task.status = Status::Cancelled;
            task.generation += 1;
            state.bump();
        }
        if state
            .control
            .as_ref()
            .is_some_and(|c| c.origin == active || c.request.task == active)
        {
            state.control = None;
            state.bump();
        }
    }
    if state.decision_pending {
        state.decision_pending = false;
        state.bump();
    }
}

fn control_called(
    event: On<ToolCalled<Control>>,
    mut commands: Commands,
    mut agents: Query<&mut DurableAgentState>,
) {
    let Ok(mut state) = agents.get_mut(event.agent) else {
        return;
    };
    let valid = ["continue", "switch", "defer", "finish_silently"]
        .contains(&event.args.action.as_str())
        && (event.args.action == "switch"
            || state.active.as_deref() == Some(event.args.task.as_str()))
        && (!matches!(event.args.action.as_str(), "continue" | "switch")
            || !uncertain(&state, &event.args.task))
        && state.task(&event.args.task).is_some_and(|t| {
            t.generation == event.args.generation
                && !matches!(
                    t.status,
                    Status::Completed | Status::Failed | Status::Cancelled
                )
        });
    let origin = state
        .active
        .as_ref()
        .and_then(|id| state.task(id))
        .filter(|t| {
            !matches!(
                t.status,
                Status::Cancelled | Status::Completed | Status::Failed
            )
        })
        .map(|t| (t.id.clone(), t.generation));
    if valid && state.control.is_none() && origin.is_some() {
        let (origin, origin_generation) = origin.unwrap();
        state.control = Some(PendingControl {
            request: event.args.clone(),
            origin,
            origin_generation,
        });
        state.bump();
        commands
            .entity(event.call)
            .insert(Outcome::from(ToolResult::success(
                rig_harness::rig_core::tool::ToolOutput::text(
                    "control accepted; applied after this batch is committed",
                ),
            )));
    } else {
        commands.entity(event.call).insert(Outcome::refused(
            "invalid control action, task, generation, or another control is pending",
        ));
    }
}

fn apply_control(world: &mut World) {
    let changes: Vec<_> = world
        .query::<(Entity, &DurableAgentState)>()
        .iter(world)
        .filter_map(|(agent, state)| state.control.clone().map(|c| (agent, c)))
        .collect();
    for (agent, pending) in changes {
        let control = &pending.request;
        let state = world.get::<DurableAgentState>(agent).unwrap();
        let valid = state.active.as_deref() == Some(pending.origin.as_str())
            && state.task(&pending.origin).is_some_and(|t| {
                t.generation == pending.origin_generation
                    && !matches!(
                        t.status,
                        Status::Cancelled | Status::Completed | Status::Failed
                    )
            })
            && state.task(&control.task).is_some_and(|t| {
                t.generation == control.generation
                    && !matches!(
                        t.status,
                        Status::Cancelled | Status::Completed | Status::Failed
                    )
            })
            && (!matches!(control.action.as_str(), "continue" | "switch")
                || !uncertain(state, &control.task));
        if !valid {
            let mut state = world.get_mut::<DurableAgentState>(agent).unwrap();
            state.control = None;
            state.bump();
            continue;
        }
        if !committed(world, agent).is_some_and(|s| s.control.as_ref() == Some(&pending)) {
            continue;
        }
        let open = world
            .query_filtered::<&ForAgent, (With<ToolRequest>, Without<Settled>)>()
            .iter(world)
            .any(|of| of.0 == agent);
        if open {
            continue;
        }
        let mut state = world.get_mut::<DurableAgentState>(agent).unwrap();
        match control.action.as_str() {
            "continue" | "switch" => {
                if let Some(old) = state.active.clone() {
                    if old != control.task {
                        state.task_mut(&old).unwrap().status = Status::Suspended;
                    }
                }
                state.active = Some(control.task.clone());
                state.task_mut(&control.task).unwrap().status = Status::Running;
            }
            "defer" => {
                state.task_mut(&control.task).unwrap().status = Status::Suspended;
            }
            "finish_silently" => {
                state.task_mut(&control.task).unwrap().status = Status::Completed;
            }
            _ => {}
        }
        state.control = None;
        state.bump();
        drop(state);
        if matches!(control.action.as_str(), "finish_silently" | "defer") {
            // An empty answer leaves a tool result at the transcript tail. Mark
            // that deliberate stop, so Rig does not infer an interrupted model
            // call from that tail on restart. Queued task continuation is saved
            // separately and starts a normal new foreground turn.
            if let Some(mut conversation) = world.get_mut::<Conversation>(agent) {
                conversation.halt(rig_harness::rig_core::conversation::Halt::Kept);
            }
            if let Some(turn) = world.get::<ActiveTurn>(agent).map(ActiveTurn::turn) {
                world.entity_mut(turn).insert(TurnOutcome::Answered(
                    rig_harness::rig_core::message::Message::assistant(""),
                ));
            }
        } else {
            // A call already composed while the control result was held must
            // pass composition again with the newly committed active task.
            let calls: Vec<_> = world
                .query_filtered::<(Entity, &ForAgent, &CallOf), (
                    With<TurnCall>,
                    Without<Started>,
                    Without<Outcome>,
                    Without<Settled>,
                    Without<SupersededBy>,
                )>()
                .iter(world)
                .filter(|(_, of, _)| of.0 == agent)
                .map(|(e, _, of)| (e, of.0))
                .collect();
            for (call, turn) in calls {
                let next = world.spawn((TurnCall, ForAgent(agent), CallOf(turn))).id();
                world.entity_mut(call).insert(SupersededBy(next));
            }
        }
    }
}

#[cfg(not(feature = "checkpoint-probe"))]
fn probe(_event: &str, _state: &DurableAgentState) {}

// Disposable acceptance builds only. The ordinary runtime feature contains no
// environment-controlled fault hooks, trace writer or deliberate blocking.
#[cfg(feature = "checkpoint-probe")]
fn probe(event: &str, state: &DurableAgentState) {
    use std::io::Write;
    let Some(path) = std::env::var_os("VIVY_DURABLE_TRACE") else {
        return;
    };
    let operation = state.operations.last();
    let stage = match (event, operation) {
        ("prepared", Some(op)) if op.outcome.is_none() => format!("before_intent:{}", op.kind),
        ("dispatch", Some(op)) => format!("after_intent:{}", op.kind),
        ("received", Some(op)) if op.outcome.is_some() => format!("after_effect:{}", op.kind),
        ("committed", Some(op)) if op.outcome.is_some() && !op.published => {
            format!("after_result:{}", op.kind)
        }
        ("committed", _) if state.tasks.iter().any(|t| t.status == Status::Completed) => {
            "after_completion".into()
        }
        _ => String::new(),
    };
    let pause = !stage.is_empty() && std::env::var("VIVY_DURABLE_PAUSE").is_ok_and(|s| s == stage);
    let result = (|| -> io::Result<()> {
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;
        serde_json::to_writer(
            &mut file,
            &json!({"event":event,"stage":stage,"paused":pause,"state":state}),
        )?;
        file.write_all(b"\n")?;
        file.flush()
    })();
    if result.is_err() {
        std::process::abort();
    }
    if pause {
        loop {
            std::thread::park_timeout(std::time::Duration::from_secs(30));
        }
    }
}

#[cfg(test)]
mod tests;
