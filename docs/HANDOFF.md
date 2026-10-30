# Handoff: continue the independent assembly experiment

## Detailed Plan B architecture proposed (2026-10-31)

The owner requested detailed architecture after selecting Plan B. Read the
[detailed proposal](DESIGN.md#detailed-architecture-proposal-2026-10-31) within
the existing decision document. Plan B itself is accepted; the concrete state,
storage and integration contracts are proposals for review, not implemented or
new owner-approved guarantees.

The proposed minimum adds a `vivy-durable` plugin to the existing Rig execution
path. Reuse Rig's logical conversation logs and saved components; a framed
JSONL `DurableStore` synchronizes each append and exposes precise receipts.
Holds gate admission, dispatch, result settlement and continuation. Task state
and recovery data share an agent-log commit. The first path serializes tools
per agent to meet single-tool handoff and keeps unknown external outcomes
explicit. A saved one-shot wake needs a running host for on-time execution.

Later slices connect native Laputa, agent-owned Recap, existing MemPalace and
AutoDream. Persona/context updates become effective at a new model operation;
an interrupted operation retains its pinned context. The first AutoDream slice
validates and automatically adopts persona/textual candidates. Product UI,
official inbox, external adapters and background-service deployment stay outside
the first infrastructure loop.

Source inspection used the recipe's Rig pin
`e1cd47d57f0bccb50ddffd6f45de2ac3365b271c`, not reference checkout HEAD. Required
proof remains: holds surviving multi-pass execution until `Last/WriteLogs`,
receipt/state matching, final redacted outcome capture, model-result recovery,
cancel ordering and fail-closed durable-format detection on resume. The private
flush/drain APIs mean a Store wrapper alone is insufficient. If the pinned public
seams cannot support the contract, identify the smallest required Rig API change
before expanding implementation; do not mutate the reference or silently change
architecture. D0 verifies these seams through one coding task; D1-D3 close
execution, context/memory and wake, then D4 evolution. See the proposal's
acceptance table. No product code or new runtime/provider verification is
delivered by this document update.

## Plan B selected: extend the existing execution chain (2026-10-31)

The owner selects DESIGN's
[Plan B](DESIGN.md#plan-b-durable-execution-within-the-existing-rig-chain-owner-2026-10-31).
Keep one Rust Vivy core and the existing Rig model/tool execution path; add
persistent phases, commit boundaries, recovery, cancellation and sleep/wake
semantics, using the current coding face for validation. Pi durable is a full
agent execution runtime designed for persistence and is the semantic anchor,
not a selected TypeScript dependency or a CLI mode to enable.

Mysoul informs Recap, explicit recall, autonomous wake and silence. Preserve the
accepted Laputa/persona, agent-owned task context, native MemPalace/workbench and
AutoDream boundaries. JSON/JSONL remains the host storage direction. Product
navigation and the official inbox schedule remain parked as recorded below.

Next detail the minimal state/commit contract and verify the Rig seams for tool
start, result publication, restart and cancellation. Existing Rig log flushes
do not alone prove durable transactions or disk sync. Begin with one recoverable
model/tool task, then context/memory and wake integration. Record remaining seam
limitations explicitly; do not silently replace the selected route with coarse
task continuation or a new execution loop. No implementation or new runtime
verification is delivered by this decision commit.

## Current priority: one Vivy core, validate through Coding (2026-10-31)

Read DESIGN's
[single-core decision](DESIGN.md#single-vivy-core-and-infrastructure-first-decision-owner-2026-10-31)
first. The owner settles one Vivy technical foundation and prioritizes completing
agent infrastructure using the existing coding face for actual validation.
Reuse the working Rust components, with Pi durable as the persistent-lifecycle
reference and the accepted Laputa, Recap, MemPalace and AutoDream boundaries.
Keep workload-specific state and recovery contracts explicit in this core.

The role entry, session-menu layout, independent-session navigation, assistant-
first delegation and Diva desktop/pet/bundle form remain **undecided and parked**.
The owner explicitly withdrew those topics from the immediate decision scope.
Do not treat the earlier conversational confirmation as an approved UI design,
inspect the old Vivy/Diva projects, or make a new interface a prerequisite.

Next identify the infrastructure gaps and define the smallest behavior to build
and verify through the current coding face. This decision delivers no runtime
code or new test evidence; outstanding provider acceptance remains recorded,
and the official inbox implementation remains deferred.

## Durable design preparation and Pi refresh (2026-10-31)

The owner now explicitly starts persistent-assistant design preparation, with
pi-durable as the anchor and the mysoul basic loop, Laputa governance and
AutoDream decisions as inputs. The owner confirmed that "myllm" in the request
means mysoul. This supersedes the older instruction to defer durable research;
the current request is inspection followed by discussion, not implementation
approval or a new real-provider acceptance result.

The clean `.workspace/pi` reference was fetched and moved in detached HEAD from
`f321e5bdd26b3624bfb2ab182a334d0a5edb698a` to upstream `main` at
`a14cdf90f0a76532f3e20bc3c01872837cef24ba` (commit date 2026-10-30).
No dependencies were installed and no Pi code was modified. Earlier coding
comparisons remain evidence for their original revision. Read the refresh in
[session storage research](research/2026-10-27-session-storage.md#durable-reference-refresh-2026-10-31)
before relying on the old durable storage or notification descriptions.

Key differences for discussion: Pi steering follows a whole tool round, whereas
the owner selected a single-tool handoff boundary; Pi's new `afterRun` work keeps
the conversation busy and cannot be assumed to be the AutoDream scheduling
policy; streamed progress can now be visible before persistence, unlike task
admission and tool intent. The official inbox remains deferred. Rust lifecycle,
storage, context refresh and recovery details still need a bounded design.

Source and existing verification records were inspected in mysoul's current
Desktop checkout (`3a7cbde7`), including Recap-only context, explicit recall and
autonomous wake scheduling. No application was run or live data accessed. The
previous Vivy workbench provider gate is not retrospectively marked passed.

## Laputa protocol and native governance checkpoint (2026-10-31)

Read DESIGN's
[Laputa protocol and Garden governance decision](DESIGN.md#laputa-protocol-and-garden-governance-decision-owner-2026-10-31)
before continuing persona, memory or evolution design. The owner repositions
Laputa as a specification/protocol with host-specific implementations. Vivy's
preferred direction is a small native Rust persona component with read/update
operations and top-level persona-context injection. A complete Garden/Go runtime
or external service is not required. Exact APIs and persona format remain open.

The agent/harness owns Recap/ACTMEM. Full Python-native MemPalace remains the
current long-term engine; later backend replacement, such as Mem0, is an
architecture goal, not an implemented adapter or migration. Do not duplicate
persona in Prime harness entries or inject MemPalace L0.

Garden remains an optional reusable governance workflow for other agents: collect
corpus, run a selected agent, apply updates to the host's existing authority,
verify effective changes and retain outcomes. Respect native files/evolution
instead of establishing parallel persona or Recap stores. Vivy AutoDream retains
automatic candidate adoption; other hosts keep their own write policies. Hermes
and OpenClaw are intended adapter examples, not verified integrations. A Web
console and Garden control of Vivy's internal Laputa remain later discussion.

This direction is synchronized to the local Garden/Laputa project's ADR-0016.
Documentation only: no new runtime, backend, adapter, durable work or inbox work
is delivered; the owner-deferred real-provider workbench gate remains unchanged.

## Inbox scheduling checkpoint (2026-10-31)

The owner defers the official inbox plugin and its notification/hook and durable-
agent integration to a later phase, outside the immediate implementation stage.
Keep the accepted behavior in DESIGN's
[Inbox and event handoff decision](DESIGN.md#inbox-and-event-handoff-decision-owner-2026-10-31),
but do not start it or make it a nearest-stage gate. Pending messages survive
restarts; the agent chooses reading/deletion/reply and controls reminder hooks.
Ordinary hook/chat/new-task handoff waits for one tool execution to finish, then
the agent decides what to do; humans retain explicit immediate interruption.
No implementation date, exact later-phase order or full swarm runtime is selected.

## PEN, NeuroLink and Minecraft decision checkpoint (2026-10-30)

The owner locks the positioning in DESIGN's
[PEN and NeuroLink decision](DESIGN.md#pen-and-neurolink-decision-owner-2026-10-30).
PEN packages deliver complete external capabilities on import and may combine
tools, MCP connections, Skills, connector code, resources and configuration.
PEN is broader than an MCP configuration bundle. NeuroLink is a separate
A2A-like protocol/standard for external conversations and events; channels are
one integration class. Execution and actual lifecycles remain in Vivy or the
responsible PEN module, not in a protocol-level attention/scheduling system.

Minecraft is the first selected game. Independent-player multiplayer is the
primary route; computer-controlled client play must remain possible in the
design but is not a second first-delivery gate. Edition/version, Mineflayer or
another connector, auxiliary-mod policy, manifest/runners, hub distribution and
the exact A2A/ACP/AG-UI relationship are not finalized. Mineflayer remains a game
connector candidate, not a verified Vivy integration.

Final channel ruling, 2026-10-30: port/adapt channel packages from
[Octos](https://github.com/octos-org/octos) and load them through NeuroLink.
Octos replaces the earlier ZeroClaw candidate. The inspected channel boundary is
`crates/octos-bus` at `7486f3df54b5803ec4095c60b244617c9d254750`; that crate
also contains session/cron facilities beyond the selected port. Read DESIGN's
**Channel packages: port Octos and load through NeuroLink** for the source links
and scope. This selects channel reuse, not Octos's agent runtime or OUP. Package
extraction/loading details remain open; no port, build or runtime acceptance is
delivered. Owner refinement, 2026-10-31: only the official QQ Bot channel is
selected from Octos. NapCat QQ is a separate actively browsable PEN capability,
using NapCat/NTQQ's existing data rather than a second mirrored QQ database.
Read DESIGN's NapCat section before treating it as another passive channel.

This is a documentation-only decision checkpoint: no PEN/NeuroLink/Minecraft
runtime is delivered. Recipe-selected compilation, existing setup/trust rules,
durable sequencing and the deferred workbench provider gate are unchanged. Read
the locked decision before using older PEN-undefined or channel-only scope notes.

## AutoDream decision checkpoint (2026-10-29)

The owner selects AutoDream for the new Vivy's evolution chain: a collection
workflow supplies a dedicated evolution workspace; a dedicated Dream prompt
runs without Vivy's conversational persona, organizes the workspace and produces
Prime-style artifact candidates plus persona data. Candidate generation remains;
adoption is automatic, without routine human approval of each candidate. Do not
reduce the implementation to proposal generation awaiting manual activation.

Read DESIGN's **AutoDream evolution decision** before continuing evolution work.
It supersedes the older outside-Vivy/research-only direction and does not select
Prime's refine-trigger route or port the old AutoDream implementation. Exact
artifacts, triggers, validation, adoption/version boundaries and recovery remain
design work. Existing recipe compilation and initialization-consent boundaries
remain in force. This checkpoint records a decision only: AutoDream is not yet
implemented, and the deferred workbench provider gate and durable sequencing
are unchanged.

## Initialization closure (2026-10-28)

The approved plan is `docs/plans/2026-10-28-initialization.md`. Setup now registers
from compiled plugins, follows trust/provider/Python/optional memory, remembers
project skips and scoped installation consent in user state, and offers `/setup`.
Python startup detection, consented repair, managed Windows x64 uv/Python,
structured stages, fixed native memory validation and safe next-cell environment
adoption extend the existing workbench. Provider configuration has save-only and
explicit streaming/tool verification with cancellation and a deadline. MemPalace
remains opt-in. Do not restart durable or the deferred StepFun acceptance.

Verified during implementation: root workspace tests (18+1+2+1), basics 82,
gateway 13+1+6+14, Python Rust 20 and all 43 Python/native tests (44.403s).
The new Windows
`scripts/initialization_tui.py` passes
ordered prompts, remembered skips, manual preparation, approved repair after a
missing interpreter, no automatic provider requests, explicit streaming/tools
and pinned provider failure, including cancellation that preserves the earlier
error. A real isolated managed-toolchain download verified
the pinned uv archive and installed Python 3.13.12 without system registration.
The complete scripted Coding/Skill/MCP/native-memory workflow passes in 85.774s
(33.006s candidate preparation); it is not real-provider verification.

One independent whole-branch review found three Important issues, reproduced
and fixed: interpreter startup before scope validation, descendants retaining
installer pipes after parent exit, and an extra package index omitted from the
consent scope. Startup now checks both scope and a locally recorded publication
hash in Rust before executing a prepared interpreter. Legacy/copied environments
need `/setup python` or `/python prepare` once; automatic detection does not run
them. This local validation record is separate from future installation consent.

Windows initialization, workbench rotation/lifecycle, branding, Plan and provider
terminal regressions pass, including 120x40 and 50x20 provider/branding screens.
The headless workbench and recipe-omitted Python dependency checks pass. Twenty
real workbench/child/MCP lifecycle cycles pass with zero orphans; cold/warm median
calls were 0.835/0.066 seconds and maximum close plus confirmed exit was 0.164s.
Queued-call cancellation, compaction retention, crash/timeout recovery and Skill
removal also pass. The lifecycle harness now waits for compaction's own end event
before submitting its warm-state check.
Formatting, PowerShell parsing and diff checks pass. One preceding Python run
hit a Windows temporary-venv cleanup sharing error after its assertions; no
fixture process remained, and the complete suite passed using a dedicated temp
directory. Keep that environmental failure distinct from functional acceptance.

After fast-forward integration, the main-checkout launcher and Coding binary were
rebuilt. Root tests (22), the original launcher Coding smoke and the complete
initialization terminal scenario pass on main, including automatic repair,
explicit provider streaming/tools and cancellation preserving a prior failure.
For another machine: pull main, run `build.cmd` then `start.cmd`, and follow the
startup guide or `/setup`. MemPalace remains an explicit optional setup item.

## Footer rotation (2026-10-28)

The terminal alternates right-side telemetry and Python/memory status every five
seconds, pinning preparation/errors and holding recovery for five seconds.
The existing runtime publishes per-agent observations; the display uses Rig's
clock and fitting without launching Python, installing packages or querying the
database. `/python status` refreshes the cached interpreter check. See DESIGN's
footer section and `scripts/workbench_tui.py` for behavior and verification.

## Active workbench checkpoint (2026-10-28)

Implementation is integrated into `main`, including the remote one-click build
and terminal scroll/resize changes. The ignored `.workspace/python-workbench`
worktree retains local acceptance environments and evidence. The earlier
2026-10-27 sections below are historical: the new
owned Python runtime, Rust dispatcher, explicit environment preparation, trusted
Skill loading, generic MCP, complete artifacts and native MemPalace facade now
exist. Read DESIGN's latest checkpoint, README's workbench usage and the plan
before continuing. Durable has not started.

Verification commands belong to this repository: root `cargo test --workspace`,
focused Cargo tests for `plugins/vivy-python` and `plugins/vivy-basics`, and
`python -m unittest discover -s plugins/vivy-python/tests` using the prepared
interpreter (with locked MCP/native dependencies installed). Native tests require
an explicitly prepared isolated root in `VIVY_MEMORY_TEST_ROOT`; without it they
are reported as skipped. The real memory acceptance venv and assets are ignored
under the implementation worktree's `.workspace/memory-acceptance`.

The new `scripts/workbench_smoke.py`, `workbench_lifecycle.py`,
`workbench_optional.py` and `workbench_tui.py` provide Coding, 20-cycle ownership,
recipe omission/headless and Windows terminal checks. The joint script's default
gateway is scripted; `--real` uses an explicitly configured endpoint and credential
and must never be reported as passing based on its default mode. Keep test data
synthetic and separate from any existing user's palace or session directories.

The independent review found five Important issues; all were reproduced and fixed.
Final verification passes 13 Rust workbench tests and 36 Python tests including
9 native tests, actual queued-call close/reset, compaction retention, 20-cycle
ownership, Windows Python terminal and original Coding smoke. Maximum close command
plus process-exit confirmation is 0.256 seconds. Complete scripted joint acceptance
now independently inspects artifacts and reopens memory; it passes in 84.28 seconds.

Merged-main verification also passes root workspace tests, 33 gateway tests,
78 basics tests, 13 Rust workbench tests, and all 36 Python/native tests.
The rebuilt main-checkout launcher and Coding binary pass the Windows branding
terminal at 120x40 and 50x20 (including resize), Python terminal lifecycle and
original Coding smoke. Formatting, PowerShell parsing and diff checks pass.
The optional Desktop dialect enters the same first-run provider bootstrap and
is covered for persistence and preservation of existing managed settings.

For another machine, pull `main`, build with `build.cmd`, launch with `start.cmd`,
then run `/python prepare` (including native memory preparation as described in
README). Local virtual environments, model caches and credentials are not shipped.

Outstanding: the owner deferred actual-provider verification to a later manual
test while changing provider. Stop StepFun retries. Earlier attempts failed TLS
before receiving a model response. The authorized credential was not committed or
printed. Configure the replacement through the existing provider workflow, use an
isolated synthetic test root, then run the joint scenario. `--real` takes an
OpenAI-compatible endpoint via `OPENAI_BASE_URL`, an explicit `VIVY_TEST_MODEL`
and `RIG_SECRET_GATEWAY`. For another compiled protocol, perform the same recorded
workflow through that provider's existing configuration. Do not
claim M3 complete or start durable before that system gate is accepted.

## Goal and current state

The owner wants a fast Rust experiment preserving Vivy's useful idea of a species
assembled from a YAML recipe at compile time. Rig supplies the current coding
implementation. Model access is gateway first. The latest 2026-10-31 decision
settles one Vivy core: complete agent infrastructure and validate it through the
existing coding face. Classic Pi remains the coding reference and pi-durable
anchors persistent lifecycle behavior within that shared technical foundation.
Product entry points and Diva packaging are deferred. Coding has passed initial
real-task acceptance; the new durable behavior still needs its own execution
evidence. The delivered Python workbench and native MemPalace integration remain
available, with their deferred real-provider acceptance recorded above.
Both paths use the same workspace and may recall the same domain-partitioned
MemPalace memory. The host's Laputa implementation owns persona; do not inject
MemPalace's L0 identity. The agent/harness owns Recap/ACTMEM; durable's layered
context remains a later memory slice. Do not apply mysoul's companionship context
policy to coding by default. Domain wings,
topic rooms and source drawers are distinct from the optional L0-L3 context
stack. Preserve per-task state/namespace boundaries within the shared workspace.
Recursion, factory, a complete Prime daemon and automatic evolution are not
prerequisites. Read DESIGN's **Single Vivy core and infrastructure-first decision**
before the historical two-harness discussion. Durable infrastructure is not yet
delivered; this decision does not change that implementation status.

### Saved discussion checkpoint (2026-10-27)

Latest owner acceptance (2026-10-27): the agent completed a real coding task by
fixing one of its own bugs. The owner confirmed the chain was stable and
explicitly accepted it. The initial real-task coding gate is closed. The owner's
subsequent sequencing decision makes the Python workbench the next deliverable,
with durable starting after workbench system validation. This is owner-run
acceptance, separate from the historical automated
checks. Do not require another coding demonstration before that next slice.

The owner asked to save today's decisions before further work. Resume from
DESIGN's current two-harness section and **Python-native memory candidate and
research checkpoint**, not the older full Prime/evolution direction. The order
is now: accepted coding foundation, new Python workbench design and system
validation, then the separate durable foundation. Shared workspace/domain memory does
not merge their task state or replay semantics. Keep the implementation small.

The latest investigation is a native MemPalace library facade inside Python.
Programmatic search and collection access exist, but native write/chunk/read
equivalence is unverified. remember/search/get are proposed facade names, not
shipped APIs. Laputa supplies persona; MemPalace L0 is not injected. No MemPalace
fork, native-memory implementation or dependency change was made. Record costs
before treating optional-feature omission as a smaller installation/runtime.

Prime runtime `c4c238ca025e382d96fb5f37e6328ca77fb82ef3` passed seven isolated
Python protocol/lifecycle probe checks; that does not validate its Rust adapter,
MCP, snapshots or full background operation. Existing vivy worker tests passed
six cases; its asyncio tasks are still cancelled at cell end. DESIGN preserves
the exact inspected paths, pins, probe limits and mysoul reference locations.
mysoul remains read-only reference material; no sessions or memory data are
copied. Next work requires a bounded implementation scope, not another broad
capability inventory. Today's save contains documentation only.

Repository: `https://github.com/mastwet/vivy-test`.
The owner's subsequent Go/remake migration assessment is recorded in
[research section 18](research/2026-10-25-hybrid-agent-workbench-decisions.md#18-go-vivy-migration-assessment).
It checks pinned source sizes and donor overlap, proposes a minimum behavior
selection, and gives a conditional engineering allowance. It does not authorize
implementation or change the two-harness scope. Original source was inspected
read-only; no app data, source port or legacy data migration was involved.

The owner's next refinement reduces migration scope further: retire BML, use
the full upstream MemPalace engine in Python (still no L0 persona injection),
and rewrite Laputa. Small bundles and hooks remain intended capabilities.
Plan is implemented; external hook and saved session-goal examples exist in Rig
but are not selected. Autonomous Goal remains unwired. See DESIGN's native-memory
section and research 18.7; do not reuse the earlier broad estimate as a quote
for copying old features.

The owner requested a broad feature-by-feature reconciliation, including A2A.
[The Vivy feature inventory](research/2026-10-27-vivy-feature-inventory.md)
separates owner scope decisions from actual target implementation, covers all
16 remake lanes plus A2A/ACP/SSH and lists unresolved product choices. A2A has
accepted server source/evidence on its Go feature branch but is not selected in
vivy-test; its server and an outbound A2A client are separate capabilities.
Use that inventory rather than treating older suggestions to defer as decisions.

Latest owner decisions narrow the inventory: no legacy data or deployment
compatibility; no old Notebook/Reports, BML, Skill-evolution or AutoDream
migration. A new MemPalace + Obsidian loop is intended for the Python workbench
and awaits design; new Prime-style evolution is outside Vivy. INOFY-style
workflows, if selected, belong in Python. Cron/webhook/background are redesigned;
LSP is optional; QQ channel is the only first-phase channel candidate and remains
undecided. ACP/SSH use references rather than the old Go architecture. The
inventory also records the Prime and Rig history/context/file-rewind source
findings; no such additional implementation is implied.

The later [session storage comparison](research/2026-10-27-session-storage.md)
records Prime, Pi coding, pi-durable and the current Rig storage boundaries. It
records the owner's later preference for JSON-based files, retaining Rig's
coding store and evaluating a Rust-owned file-backed durable store. The durable
format and recovery protocol remain open and are not implemented.
Prime's fast window path falls back on Windows; pi-durable's Node SQLite defaults
and single-owner assumption are explicitly qualified in that research.

The owner further clarified: do not add a Vivy-owned SQLite database alongside
MemPalace; focus on a Python-workbench ingestion and recall flow into MemPalace's
own storage. JSON-based files remain the proposed session/task substrate.
Single-binary deployment is not important. This is not a ban on the storage
technologies internal to MemPalace. Ingestion policy and durable file commit
semantics are not yet designed or implemented; see DESIGN.

Current state: the builder, gateway, native Windows launcher, Git Bash process
adapter and auxiliary Python workbench are implemented. DESIGN's "Windows coding
species and Python workbench" section records implementation evidence. The owner
accepted the real-task coding path as recorded above. Gateway model facts
(delivery step 1) are implemented and checked against
the scripted gateway. DESIGN's 2026-10-27 two-path decision supersedes conflicting
scope and ordering in its older "Delivery slices and stopping points" record.
The later owner-approved collaborative Plan addition is implemented in
`vivy-basics::plan::PlanPlugin` and selected in the baseline recipe. It includes
terminal mode markers, explicit plan submission and user review/feedback;
see DESIGN's "Collaborative Plan mode" and README usage. Goal and automatic
evolution remain separate work.
The owner-approved 2026-10-26 terminal refresh selects a compact vivy-test screen
on Rig's public widgets, a persistent footer and a branded default identity.
See DESIGN's "Compact vivy-test terminal" and the verification appendix below.
Read [DESIGN.md](DESIGN.md) for decisions and [PI-BASELINE.md](PI-BASELINE.md) for
coverage. Those documents replace the need to reconstruct the chat history.

Latest delivery (2026-10-25, `feat/subagents`): the recipe selects a Vivy wrapper
around Rig's native delegation. It provides roles, task/message/wait, bounded
parallel batches, chains, saved child follow-up and terminal progress/usage.
Children share the host process and project context; cwd stays within the project.
Default depth is one, with four active children per session. No Python bridge,
apixis, sandbox or automatic worktree was added. See DESIGN and
[delivery evidence](logs/2026-10-25-subagents.md); mirrored research section 17.7
supersedes the previous single-child scope. Commit c94f87f was fast-forward merged
into local main on 2026-10-25. Collaborative Plan commit b43d448 is integrated
alongside it on 2026-10-26. This integration has not been pushed.

## Reference setup

Clone **Pi and Rig themselves**, including source, into this checkout's ignored
`.workspace/`. Installed packages alone are insufficient for the investigation.
Do not copy old reference folders from the original Vivy checkout. Do not clone
Vivy: it is undergoing separate repair, and this repository must not inherit its
build, runtime, SDK or working-process dependencies.

Run from the `vivy-test` root, only for reference directories that do not already
exist. If a checkout already exists, inspect its status and remote before making
changes; do not overwrite somebody's work.

```sh
git clone --depth 1 https://github.com/earendil-works/pi.git .workspace/pi
git -C .workspace/pi fetch --depth 1 origin a14cdf90f0a76532f3e20bc3c01872837cef24ba
git -C .workspace/pi checkout --detach a14cdf90f0a76532f3e20bc3c01872837cef24ba

git clone --depth 1 https://github.com/0xPlaygrounds/rig.git .workspace/rig
git -C .workspace/rig fetch --depth 1 origin e1cd47d57f0bccb50ddffd6f45de2ac3365b271c
git -C .workspace/rig checkout --detach e1cd47d57f0bccb50ddffd6f45de2ac3365b271c
```

These ordinary source pins identify what was inspected. They do not introduce a
species hash system. If a revision cannot be fetched, report it; do not silently
substitute current HEAD and repeat the old findings. Updating the baseline is a
normal dependency decision after reviewing the relevant changes.

The commands above are setup instructions when references are missing. No upstream dependencies need
installation just to read source. Read each reference repository's own
instructions before running its tooling. Keep product changes in `vivy-test`.

## Where to look first

| Question | Source in the reference checkouts |
| --- | --- |
| How does Rig already generate an agent? | `rig/src/launcher/project.rs`, `build.rs`, `rig/crates/rig-harness/README.md` |
| What can the existing product reuse? | `rig/plugins/rig-basics/`, `rig/plugins/rig-tui/`, `rig/crates/rig-harness/PLUGINS.md` |
| What is the smaller library route? | `rig/crates/rig-agent/README.md`, its `src/run/` implementation, `rig/crates/rig-core/` |
| What do coding tools and session storage actually guarantee? | `rig/crates/rig-tools/`, `rig/crates/rig-memory/`, `rig/crates/rig-ecs/src/stage.rs` |
| What does Pi's loop do? | `pi/packages/agent/src/agent-loop.ts`, `pi/packages/coding-agent/src/core/agent-session.ts` |
| What must session and extension behavior preserve? | Pi coding-agent `docs/sessions.md`, `session-format.md`, `compaction.md`, `extensions.md` |
| What are the current MCP and Code Mode capabilities? | Pi coding-agent `docs/mcp.md`, `codemode.md`; Rig `crates/rig-rmcp/`, `plugins/rig-steel/` |

## Suggested next implementation task

Owner decision, 2026-10-27: the workbench closure scope, exclusions and transplant/
local implementation ownership in [research section 19.9](research/2026-10-25-hybrid-agent-workbench-decisions.md#199-closure-scope-and-implementation-ownership-2026-10-27)
are accepted and recorded in DESIGN. Do not reopen the broad scope decision.
Next turn the accepted boundary into concrete adapter contracts and an
implementation plan. The sequence is core W1-W5 validation through Coding,
then the bounded native MemPalace consumer gate, then durable. Scope acceptance
is not runtime acceptance: sixteen upstream REPL tests passed on Windows, but
MCP transport and the proposed Vivy adapter were not run.

The 2026-10-28 deeper Prime donor audit is in research section 19.10. It adds
protocol/output attribution and loss boundaries, startup/cancellation races,
environment locking and package metadata, MCP pagination/credential/lifecycle
concerns, a component-by-component extraction map and focused verification
cases. In particular, upstream `repr` and current Vivy collection can truncate
before a spill, so the accepted complete-artifact contract needs producer-side
design. This is source inspection only; no new runtime result or scope change.

Owner focus, 2026-10-28: research and close the workbench plus bounded native
MemPalace consumer; do not start durable research now. Research section 19.11
maps the pinned MemPalace library seam, source-identity collision in the MCP
content-derived ID, chunk/get and backend cleanup boundaries, Windows writer
contention, embedding preparation and blocking-call cancellation. It proposes
synthetic acceptance cases and records the installed/pinned version mismatch.
No MemPalace runtime test or product implementation was performed.

The subsequent component audit is in research section 19.7. It identifies the
REPL/protocol/bridge, executable-Skill loading and generic MCP lifecycle as donor
candidates, and maps all nine bundled Python Skills to their actual host or
library dependencies. The current Vivy worker protocol is incompatible with a
drop-in Prime worker; host routing must change with it. Keep `vivy-process` and
existing direct tools. No transplant has been implemented or validated yet.

The owner supports this selective-reuse direction and requested investigation of
Python-side tool loading. Section 19.8 records the source findings and a bounded
loading contract. Resolve that contract in the design: installed packages,
bound Python names and model-visible capabilities are distinct; Skill code
updates and MCP connection reload have different lifecycles. Extend W2 with
namespace conflicts, unavailable capabilities and update/removal behavior.
The accepted restart policy is fresh Python state plus preserved explicit
artifacts and a reset notice, without automatic cell replay. Skill code updates
use a worker restart boundary; MCP connection reload is separate. The native
memory consumer remains a separate gate from W1-W5 and the durable lifecycle.

The 2026-10-27 scope clarification takes precedence over older sequencing below.
The classic-Pi coding path has passed owner-run real-task acceptance. Prioritize
the new Python workbench: close its design, implement the selected runtime and
adapters, and validate the complete capability lifecycle independently and
through coding before starting durable. Preserve the working coding path and
its existing regression checks. Current worker code preserves variables but
cancels async tasks at cell end; timeout/cancellation resets its process. The
earlier Prime probes do not establish a delivered replacement.

Define bounded acceptance scenarios for discovery/instructions, persistent
resources, Python/MCP calls, correlated bounded results, failure, cancellation,
reset and cleanup. Exact extraction boundaries, adapter contracts and test
fixtures remain design work within the accepted scope. Keep future host
integration explicit without adding a universal harness or moving authoritative
task state into Python.

After workbench validation, establish the pi-durable-referenced Rust harness
and test its workbench integration under its own recovery/replay rules.
`pi-durable` is a TypeScript reference, not a Rust integration already present.
mysoul's memory behavior remains a durable-path reference, not a dependency on
its application, private data or runtime.

The owner also requested provider-management research on 2026-10-26. See
[provider management and plugin boundaries](research/2026-10-26-provider-management.md)
for Pi/DSH/Rig evidence and a proposed extension of the gateway-first surface.
The owner fixes provider scope to OpenAI-compatible and Anthropic-compatible
protocols only, with built-in presets and user-defined instances derived from
them. Subscription login/OAuth/token refresh and other native protocols are
excluded. DESIGN records this scope; the research records Rig's registration
boundary and an offline API-construction probe, with no live-provider claim.
Provider management commit `de10ee3` was fast-forward merged into local `main`
in `vivy-test` on 2026-10-26. The baseline selects ProvidersPlugin, two protocol
features and the full TUI editor. README documents commands and configuration;
the existing research document records verification and review corrections.
The native launcher and baseline agent were rebuilt from this main checkout.
The integration remains local. No new live-provider or non-Windows platform
claim is made.

The older coding-only delivery sequence and its verification gaps remain in
DESIGN and PI-BASELINE. Preserve the Windows launcher and Rig harness/TUI while
checking that path; keep actual gateway evidence distinct from mock checks.
Those coding records do not establish or select the durable implementation.
The validated Python workbench now precedes the durable implementation.
Automatic evolution and PEN/NeuroLink have later dated decisions above;
their implementation and persona growth are not delivered by this checkpoint.

Use existing authorized gateway configuration without printing credentials. If
essential configuration is unavailable, request only that missing information.
Do not guess secrets or present a scripted endpoint as a real gateway smoke.

## Research directions and traps

- **Measure iteration cost.** More upstream reuse can mean more dependencies but
  less owned product code. Check cold/warm/plugin-edit builds and debugging cost
  before deciding that a smaller-looking crate is the faster foundation.
- **Prove real optionality.** Inspect the resolved dependency graph for each
  generated agent. Shared upstream crates can contain several tools/providers;
  omitting a wrapper does not prove all their source was excluded.
- **Keep feedback useful.** Preserve bounded task evidence for useful errors and
  future memory/refinement work. The Prime workbench loop does not require its
  incident CLI, daemon, service catalog, subagent platform or automatic evolution.
- **Preserve semantics as coverage expands.** Branch-aware state, cancellation,
  extension interception, and tool-result pairing matter more than matching
  command names. Use representative behavior checks.
- **Keep source assertions modest.** Rig library availability does not prove
  integration with this gateway, terminal, platform, or recipe. New APIs at this
  pre-release revision may differ from older online examples.

## Evidence at handoff

DESIGN's dated implementation sections retain build, scripted-gateway and Windows
smoke evidence and limitations. They do not verify the new hybrid design. For the
2026-10-25 documentation delivery, review content/links and run `git diff --check`;
runtime CI is unnecessary because no executable behavior changed. Later code
deliveries must use the repository's focused checks and a representative real-path
smoke. Source availability and a successful manual run do not prove full parity.

## Collaborative Plan verification (2026-10-25, Windows)

The collaborative Plan delivery supersedes the documentation-only scope above.
Checks used Rust 1.97.1, `CARGO_TARGET_DIR=C:\vt` and
`CARGO_PROFILE_DEV_DEBUG=0`:

```powershell
cargo test --workspace --locked
cargo test --manifest-path plugins/vivy-basics/Cargo.toml --features tui --locked
cargo check --manifest-path plugins/vivy-basics/Cargo.toml --lib --no-default-features --locked
cargo run -p vivy -- build recipes/coding-agent.yaml
python scripts/smoke.py C:/vt/debug/vivy.exe
python scripts/plan_tui.py C:/vt/debug/vivy.exe
cargo fmt --manifest-path plugins/vivy-basics/Cargo.toml -- --check
git diff --check
```

Results: 20 workspace tests and 37 basics tests passed, including 14 Plan cases.
Plan cases cover unchanged tools and writable behavior, exact review/feedback,
cancellation, missing reviewer, stale approval, modal `/plan off`, running-turn
transitions, session restore and fork isolation. An unfinished review was also
reopened to verify it never replays approval. Core feature tests failed before
implementation; stale-approval and modal-command regressions failed before their
corrections, then passed.

The real Windows ConPTY terminal passed 16 scripted checkpoints: Plan entry and
ordinary clarification, mode/review markers, scrolling an 80-step plan, approval
followed by a real file write, feedback, Esc, manual exit inside the modal,
re-entry and process/session resume. Captured gateway requests confirmed guidance
changes and a stable tool catalog. The existing gateway/shell/session/Python/JSON/
error/secret-filtering/image/reasoning/compaction smoke passed. No live-model or
new macOS/Linux acceptance is claimed.

The terminal driver uses test-only `pyte==0.8.2`, `wcwidth==0.2.13` and, on
Windows, `pywinpty==3.1.0`; these are not agent/runtime dependencies. They were
installed in ignored scratch and exposed through `PYTHONPATH`. The Windows
driver selects UTF-8 before launch. Its optional evidence output contains only
synthetic test screens and requests and remains outside Git.

Two disposable recipes were built and exercised: without Plan, the model receives
no Plan tool/guidance; without the terminal, Plan submission stops with a clear
missing-reviewer message and `vivy-basics` has no normal `rig-tui` dependency.
Temporary recipe variants are not maintained configuration sources.

Fresh independent review found the modal command interception; the typed command
now routes through the same backend command and is covered in both unit and
terminal tests. Re-review reported no remaining substantiated findings. Debug
terminal startup also confirmed the explicit phase ordering removed the Plan/
compaction ambiguity. Existing Windows fixture assumptions in context/trust
tests and a smoke assertion assuming every JSON `text` field was a string were
corrected in test code; related product behavior was unchanged.
Formatting and Git whitespace checks passed. Clippy completed with one existing
`collapsible_if` suggestion in the Windows shell implementation, outside this change.

## Combined Plan and subagent integration (2026-10-26)

The owner approved merging collaborative Plan commit b43d448 into local main,
which already contains subagent implementation c94f87f. Conflicts were limited
to README composition, plugin documentation and a trust-fixture comment. Both
plugin exports and recipe components are retained; the later empty-discovery
trust correction remains intact. Plan retains its advisory per-agent semantics
and does not introduce another permission or agent execution path.

Integration verification used CARGO_TARGET_DIR=C:/vt and
CARGO_PROFILE_DEV_DEBUG=0. The root workspace's 20 tests and the basics plugin's
61 tui-enabled tests passed, including a new combined native-stage-order check.
The no-default-features library check, root/basics formatting and Git whitespace
checks passed. The baseline recipe rebuilt successfully for the existing start.cmd.

Both scripts/plan_tui.py and scripts/subagents_smoke.py --tui passed on the actual
Windows ConPTY terminal. Checks included Plan approval followed by a real write,
feedback, modal exit, restart/resume, parallel/chain delegation, role trust,
usage, and cancellation of four shell descendants without starting queued tasks.
The existing scripts/smoke.py passed gateway, shell, session, Python, JSON,
failure, credentials and model-facts checks. Test dependencies were reused from
ignored scratch; none were added to the product.

The Plan terminal script initially hit stale slash-command completion at different
steps. Outside a review, it now waits for the exact editor text, dismisses
completion and then submits; the full scenario passed. Modal command behavior
and product keybindings were unchanged. This scripted gateway verification does
not establish real-provider compatibility. No push was performed.

## Compact terminal verification (2026-10-26, Windows)

The recipe now selects `VivyPromptPlugin` and the feature-gated `VivyTuiPlugin`.
The local Codex source at `970111f6cb` was inspected for presentation; neither
reference checkout was edited or made a new build/runtime dependency. Rig's
native execution, session, question, editor and transcript paths are retained.

Checks used the pinned Rust 1.97.1 toolchain, `CARGO_TARGET_DIR=C:/vt` and
`CARGO_PROFILE_DEV_DEBUG=0`:

```powershell
cargo test --workspace --locked
cargo test --manifest-path plugins/vivy-basics/Cargo.toml --features tui --locked
cargo check --manifest-path plugins/vivy-basics/Cargo.toml --lib --no-default-features --locked
cargo run -p vivy -- build recipes/coding-agent.yaml
cargo fmt --all -- --check
cargo fmt --manifest-path plugins/vivy-basics/Cargo.toml -- --check
git diff --check
python scripts/branding_tui.py C:/vt/debug/vivy.exe
python scripts/plan_tui.py C:/vt/debug/vivy.exe
python scripts/subagents_smoke.py C:/vt/debug/vivy.exe --tui
python scripts/smoke.py C:/vt/debug/vivy.exe
```

The 20 root tests and 73 tui-enabled basics tests passed. The new tests cover
model-visible default identity and preservation of an explicit role, changing
welcome metadata, Unicode and resize fitting, persistent draft-time status,
NO_COLOR, dark/light input surfaces with preserved diff/failure colors, native
multiline Paste events, title restoration and unrelated control sequences,
and complete/live tool output through actual registered renderers. Brand tests
first failed against upstream defaults. A stale input-hint redraw and missing
shell renderer were reproduced before correction, then passed.
Final visual QA retained the project name at the end of shortened directories
and fitted whole shortcut hints, with an additional failing-then-passing test.

The baseline rebuilt successfully for the existing start.cmd. A disposable
recipe with no terminal component also built, and its normal Cargo dependency
graph contained neither rig-tui nor ratatui. The no-default-features library
check, formatting and Git whitespace checks passed. Native linking emitted
MSVC's library/export-file informational warning; no new compiler warning was
introduced by the implementation.

The compact-screen script passed on real Windows ConPTY at 120x40 and 50x20,
both with the inherited NO_COLOR setting and with truecolor explicitly enabled:
branded startup/title, Plan-visible drafts, Unicode multiline continuation,
actual shell output, exit-7 failure cause, running output and Esc cancellation.
Synthetic terminal streams were rasterized and visually inspected at both sizes;
the images are ignored test artifacts, not screenshots of a real-provider task.
The scripted gateway checked that requests use the vivy-test default identity.
The existing Plan terminal script passed approval/write, feedback, scrolling,
modal exit and restart/resume. Subagent acceptance passed child views, usage,
parallel/chain workflows and cancellation of four shell descendants. The
existing general smoke passed model, file, shell, session, Python, JSON,
failure, credential-filtering, image, effort and compaction scenarios.

Test-only pyte 0.8.2, wcwidth 0.2.13 and pywinpty 3.1.0 were reused/installed in
ignored scratch and exposed through PYTHONPATH; no product dependency was added.
The new script closes its terminal before deleting its scratch directory and
uses process liveness on Windows, since pywinpty's output reader may wait after
the process exits. ConPTY multiline checks use the native backslash+Enter
fallback; the offline Paste test separately covers newlines and Unicode.

A fresh read-only source review found no substantiated issue. This review is
distinct from the builds and terminal runs above. All network acceptance here
used a synthetic loopback gateway; no new real-provider or Linux/macOS result
is claimed.
