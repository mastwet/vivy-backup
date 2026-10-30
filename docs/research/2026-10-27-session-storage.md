# Session storage and durable state: reference comparison

Date: 2026-10-27. This is source research and a provisional direction, not an
approved implementation plan or a measured performance claim. The owner asked
how Prime and Pi store sessions and what the two Vivy harnesses should reuse.
The current project is a Windows coding experiment; the durable harness and
shared memory integration are not yet implemented.

## Reference scope

Historical comparison below: source checkouts inspected read-only. The
2026-10-31 refresh follows this list; it does not redate earlier test evidence.

- Prime `c4c238ca025e382d96fb5f37e6328ca77fb82ef3`:
  `crates/pa-core/src/session/{manager,window.rs,window_cache.rs}` and
  `crates/pa-daemon/src/session_store/`.
- Pi `f321e5bdd26b3624bfb2ab182a334d0a5edb698a`:
  `packages/coding-agent/{docs/session-format.md,src/core/session-manager.ts}`
  and `packages/durable/{README.md,src/storage,src/session}`.
- Rig `55cd481a2421fd5da468b5bd6afc6ac876883599`:
  `crates/{rig-memory,rig-ecs,rig-harness,rig-sqlite}/src`.
- vivy-test: `recipes/coding-agent.yaml`, `plugins/vivy-basics/src/prompt.rs`,
  `docs/PI-BASELINE.md`, and the Python workbench documentation.

These are pinned local source findings, not new runtime tests. No user sessions,
databases, credentials, or production data were inspected.

## Durable reference refresh (2026-10-31)

The owner starts persistent-assistant design preparation and reaffirms pi-durable
as its anchor. The local Pi reference now matches upstream `main` at
`a14cdf90f0a76532f3e20bc3c01872837cef24ba` (2026-10-30). Its durable package still
declares itself experimental. Reviewed material includes its
[README](https://github.com/earendil-works/pi/blob/a14cdf90f0a76532f3e20bc3c01872837cef24ba/packages/durable/README.md),
changelog and the relevant tool, generation, live-progress and submission code.
This is a reference refresh, not a TypeScript runtime dependency or a completed
Rust implementation. No upstream or Vivy runtime tests were run for this review.

- **Keep the lifecycle anchor.** Admission deduplication, committed phases and
  sleep deadlines, explicit abort and recovery remain relevant. Cancelling a
  wait does not abort work. A sleeping task still needs a running host or a host
  alarm to resume; a stored deadline cannot start a stopped desktop process.
- **Recovery is conditional.** The tool implementation commits intent before
  execution, defaults to unsafe replay, and reruns an interrupted tool only when
  both its saved and current replay policies are safe. Otherwise it reports
  interruption and possible partial execution. Arbitrary Python cells, MCP
  actions and remote messages need effect-specific treatment.
- **Separate progress from durable decisions.** Streamed partials/output now use
  deferred document commits: views update immediately, while persistence occurs
  with dependent work or after the default approximately 1,000 ms deadline.
  Task states, entries and tool intent still force persistence. The old blanket
  description that nothing is shown before storage must not be applied to live
  progress at this revision. Adopting this optimization in Vivy is undecided.
- **Do not copy the steering boundary.** Pi places steering after the current
  tool round; follow-ups wait for an answer. Vivy's accepted ordinary handoff is
  after a single tool finishes, with the agent deciding what comes next and the
  human retaining immediate interruption. Multiple calls in one model response
  therefore need explicit scheduling and result handling in the later design.
- **Keep evolution scheduling explicit.** New `afterRun` hooks create durable
  child tasks after an answer; the answer settles first, but the conversation
  stays busy until those tasks finish and new input waits. This is useful
  lifecycle evidence, not a selected AutoDream trigger or a reason to block
  conversation on evolution. The separately deferred official inbox is also
  distinct from Pi's internal submission queue.
- **Keep the storage decision bounded.** Storage now accepts grouped atomic
  commits with per-commit receipts; conversation views use bounded transcript
  windows. JSONL still keeps full history in memory, and storage still needs a
  single process owner. A bounded UI view does not fix backend history growth.
  The owner's JSON-file preference stands; format, sync, locking and recovery
  remain open. No grouping or cache machinery is required without a measured
  need in the selected Rust implementation.

The owner confirmed "myllm" means mysoul. The Desktop mysoul checkout is now at
`3a7cbde7`, newer than the previously referenced worktree. Its
`src/mysoul/service.py`, `laputa.py`, integration tests, runbook and validation
records support persona plus committed Recap, current input/activity state,
explicit bounded recall, and subject-scheduled wakeups as design inputs. This
inspection did not rerun those integrations. Existing real-provider wake-tool
validation records include gateway failures; the owner's basic-loop evidence
does not establish every integration or the new Rust recovery contract.

Laputa's new persona authority and AutoDream's automatic adoption remain the
accepted direction. Exact durable context policy, task orchestration and the
first acceptance scenario are still for discussion. Entering design preparation
does not mark the deferred Vivy workbench provider gate passed or advance the
official inbox implementation schedule.

## Historical implementation comparison (2026-10-27)

The owner's follow-up correctly notes that **Go Vivy uses SQLite**. The Rig
file-store row below describes the current **vivy-test experiment**, not the
older Go product. The earlier source-only inventory also identifies an optional
PostgreSQL backend. At Go revision `9cd2801f8090723a652d3e835f72d83715b82121`,
the SQLite backend uses a single database connection, applies migrations on
open, and has session, message, run, journal, snapshot, lease and other product
stores. Its journal appends an event batch and applicable user messages in one
transaction; tool-operation admission stores the operation and its journal event
in one transaction. See the pinned Go source for
[SQLite setup](https://github.com/ProjectViVy/agent-vivy/blob/9cd2801f8090723a652d3e835f72d83715b82121/internal/storage/sqlite/sqlite.go),
[sessions](https://github.com/ProjectViVy/agent-vivy/blob/9cd2801f8090723a652d3e835f72d83715b82121/internal/storage/sqlite/sessions.go),
[journal](https://github.com/ProjectViVy/agent-vivy/blob/9cd2801f8090723a652d3e835f72d83715b82121/internal/storage/sqlite/journal.go)
and [tool operations](https://github.com/ProjectViVy/agent-vivy/blob/9cd2801f8090723a652d3e835f72d83715b82121/internal/storage/sqlite/tool_operations.go).
These are useful design references, not a reason to import the Go backend,
reproduce every product store, or migrate legacy data.

The Go [V0 architecture](https://github.com/ProjectViVy/agent-vivy/blob/9cd2801f8090723a652d3e835f72d83715b82121/docs/AGENT-VIVY-ARCHITECTURE-V0.md)
records that the pure-Go SQLite driver supported a single Windows executable,
while the desired storage semantics included atomic journal commits, snapshots,
leases and restart recovery. D-026 explicitly says the database engine was not
the architectural invariant; D-031 made SQLite the V0 reference and deferred a
filesystem journal probe. This explains the recorded choice, not a proven
performance advantage over files.

## What the references actually keep

| System | Authoritative representation | Lifecycle and recovery | Long history and concurrency |
| --- | --- | --- | --- |
| Pi coding | One JSONL file per session; header plus entries with `id` and `parentId`. A branch is a path through the tree; compaction appends a summary and retains old entries. | Setup entries stay in memory until a user or assistant message exists; subsequent records append synchronously. The loader skips malformed lines and handles an unterminated last line. This is conversation history, not a durable task scheduler. | The ordinary session loader reads the file into entries. No fsync-per-message or multi-process task ownership guarantee was established. |
| Prime | The session JSONL is authoritative; an active leaf and parent links describe the branch. The manager appends records and can hydrate the full history when required. | `append_cached` flushes and calls the platform fsync helper. Suspected torn or zero-filled damage triggers a repair scan and replacement; full rewrites use a temporary file and sync before replacement. Incremental cache certification requires the existing session lease. | A disposable generation-certified window sidecar lets a warm open read the header and retained suffix; metadata sidecars support session listing. Cold or invalid cases fall back to source reads. The optimized window explicitly returns no result on non-Unix systems because the file-identity certificate is insufficient; Windows behavior must not be inferred from the Unix path. |
| pi-durable | A durable session contains conversations, immutable entries, submissions, task states and scoped documents. It offers SQLite, JSONL and other backends. | Submissions and phase transitions commit before being exposed; a sleeping task stores deadline and checkpoint and must be woken by a live scheduler or host alarm. An interrupted phase restarts from its beginning. A tool marked replay-safe can rerun; an unsafe interrupted tool reports interruption. | SQLite stores indexed records in one database; the documented Node adapter uses WAL and `synchronous=NORMAL`, so the latest commits may be lost on power/host failure. The JSONL backend optionally fsyncs before commit markers but keeps its full history in memory; the README recommends SQLite for long-running agents. One process owns the storage; there is no cross-process ownership lock. |
| Current Rig coding path | `DirStore` keeps `logs/<agent>.jsonl`, a session log, `files/` and image blobs in a session directory; `rig-session/4` records conversation changes and saved state. | The session log queues and flushes changes during the agent loop, including before tools that may change things. A torn final line is set aside when opening for writing; unreadable or incompatible content is refused. Ordinary append uses `write_all`, with no per-append sync in that path; durable replacement syncs a temporary file. | A directory lock serializes cooperating changes across processes and an expected-length check refuses a stale log write. This protects storage writes; it does not provide exclusive task ownership or durable external effects. `read_log` reads a complete log. |

Pi's `/tree` branches within one file, while current Rig agents have separate
logs within a session. Existing resume/fork/clone behavior was exercised in the
project's scripted comparison; this is not proof of identical branching or
crash semantics. A chat snapshot, an ACTMEM summary, a filesystem rollback,
and a scheduled task checkpoint are different records and must not be conflated.

## Existing infrastructure worth reusing

The selected coding recipe already includes Rig sessions and the prompt plugin
installs `SessionLogPlugin`; the default `StoreOpener` opens `DirStore`.
Rig supports resume, saved state, session listing metadata, image blobs,
compaction and stale-write detection. Its session writer records changes
incrementally. Start with those working paths and verify observed limitations
before building a second coding storage layer.

Rig also has `rig-sqlite`'s `SqliteSessionStore` behind its `memory` feature:
it stores logs as chunks and checks each log append in an SQLite transaction.
Its `AsyncStore` read assembles the complete log. This is useful reference or a
possible future backend, **not** an already-selected backend in this recipe:
the default host `StoreOpener` expects a synchronous `Store`. Its per-log
transaction alone does not atomically commit a durable task phase and all
conversation/effect records; integration and transaction boundaries need proof.
Nor does switching to it automatically provide indexed message-level queries.

Prime's generalizable ideas are source-of-truth plus disposable, validated
indexes, append-before-effect where appropriate, and recovery that does not
silently treat a broken file as a complete session. Its exact cache certificate
is not a ready Windows optimization. Pi-durable supplies the stronger conceptual
model for task admission, checkpoint, sleep, resume, replay and cancellation;
the TypeScript implementation is a reference, not a Rust dependency.

## Proposed direction for Vivy

The owner's subsequent preference is **JSON-based files rather than SQLite**.
Start with Rig's existing file store for coding and evaluate a Rust-owned
file-backed durable store using pi-durable's JSONL backend as a reference.
The exact format, transaction protocol and recovery semantics remain to be
designed and tested; this is a direction, not a claim that they are implemented.
The owner further clarified that an additional Vivy-owned SQLite database is
not desired alongside MemPalace: the intended memory investment is an ingestion
and recall flow into MemPalace's existing storage. This is not a ban on upstream
storage internals: the inspected MemPalace source includes a Chroma backend
with SQLite files. Single-binary deployment is not an owner goal.
Keep the two harnesses' session and
execution state distinct even when they use the same workspace. The Python
workbench supplies capabilities; it must not be the authoritative owner of the
Rust runtime's session or task lifecycle.

For a durable session, publish a task transition and the conversation records
it depends on as one authoritative commit. A minimal model would identify
the session, task, phase/status, revision, checkpoint and wake deadline, plus
request/effect identifiers where needed. A durable submission is admitted once;
resume loads committed state; a sleep commits its checkpoint and deadline before
the runtime relinquishes control. An actual running scheduler or host wakeup is
still required. A persisted deadline alone cannot start a stopped application.
Implement only the task kinds and records needed by the first durable use case.

This proposal does not require a generic workflow platform. Keep one
authoritative representation per session; UI projections, search indexes and
caches are derived data. Do not make separate files independently authoritative
for the same commit without a recovery protocol: a crash between writes would
leave ambiguous state. One candidate is a single JSONL batch per commit; another
is pi-durable's multiple sidecars with a main commit marker. The latter is more
involved and should not be implemented preemptively. Define and test commit
publication, truncated writes, sync policy and single-owner behavior on Windows.
Python/MCP effects still require their own recovery rules.

The owner has expressed a preference, not approved a concrete on-disk format.
Readable JSON is suitable for small snapshots; append-only JSONL avoids rewriting
a growing conversation on every event. Neither is automatically faster for
every query. Pi-durable's present JSONL backend holds full history in memory;
do not adopt that limitation as a Vivy requirement. Large-history loading,
pagination, indexes, and crash guarantees need measurement and a bounded design.
No compatibility with old Vivy data or its deployment architecture is required.

## Effects, memory and the shared workspace

- A transaction cannot encompass an arbitrary Python/MCP call, remote service,
  workspace file write, or MemPalace operation. Record intent and completion
  where needed; use stable operation IDs and idempotency only where the receiver
  supports it. If a crash leaves an external effect uncertain, expose or
  reconcile that state instead of claiming exactly-once execution or blindly
  retrying a non-replay-safe operation.
- The shared workspace provides shared files, not shared task ownership or
  automatic conflict-free file edits. Concurrent coding and durable work needs
  an explicit policy for overlapping writes. Session branching and restoring
  task state do not rewind user files.
- Laputa remains the persona source. ACTMEM/Recap is durable working context,
  with its source and commit boundary retained; MemPalace is domain-partitioned
  long-term memory accessed through the Python workbench. A palace/wing/room/
  drawer is an organization and retrieval boundary, not an access-control
  guarantee. Do not automatically copy every transcript into long-term memory.
- MemPalace, Obsidian and their synchronization remain a later design. They
  should not become a second authoritative task log. Any future asynchronous
  memory write needs an identifiable source, retry/failure policy and explicit
  visibility semantics; no cross-store atomicity is assumed.
- A derived search index or Prime-style sidecar must be rebuildable, versioned,
  validated against its source and allowed to fail without losing the source.
  Include caches, exports and backups in privacy/deletion decisions; redaction
  is not a substitute for a defined retention policy.

## What to measure before optimizing

Use isolated synthetic sessions and the actual selected implementation. Compare
short and long histories (for example 1,000 and 100,000 entries) and record cold
and warm open, active-context assembly, paged history/listing, append/commit
latency, disk growth and peak memory on Windows. Capture the sync policy,
hardware, data shape and p50/p95; benchmark before adding an index or cache.
Test interruption during append and checkpoint, corrupted tail, disk-full/error
propagation, two competing owners, restart during a replay-safe and unsafe tool,
overdue wakeup, and memory-write failure. Separate process-kill tests from
power-loss claims. These are future acceptance experiments, not tests run here.

The Python worker currently loses its namespace on worker restart/session
switch and cancels cell-spawned asynchronous tasks; its state must not be
mistaken for a durable checkpoint. Real provider, MCP, multi-process recovery,
SQLite integration and performance have not been validated by this research.
