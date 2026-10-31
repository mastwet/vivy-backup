# D0 implementation: Rig upgrade and persistence seam probes

Status: implemented and independently reviewed; findings fixed with regressions.
D0 persistence gate: blocked by
reproduced gaps. Spec: [architecture baseline](../DESIGN.md#preliminary-detailed-architecture-baseline-2026-10-31).

The owner chose to upgrade first to the inspected Rig commit
`fc556f982d501eea9d7167398a19386e4884e776`. The shorter, mistyped hash in the
conversation plan is not a second candidate. Keep Rust 1.97.1 and Bevy 0.20.0.

## Deliverables

1. Upgrade every active Rig dependency and the coding recipe together. Verify
   normal coding, provider wires, sessions, subagents, Python lifecycle and the
   Windows terminal. Ordinary restored turns follow Rig's stop default; never
   globally opt them into CarryOn.
2. Add an experimental `vivy-durable` crate with an explicit `d0-probe` feature,
   a minimal saved probe component and an observing Store. Reuse Rig's loop and
   test support. Ordinary append acknowledgement is not a durable receipt.
3. Exercise holds across multiple passes, effective-request binding, matching
   saved revisions, final redaction, restoration, cancellation, incompatible
   saved state and writer ownership. Report supported, gap or unverified with
   evidence, including negative reproductions. Do not ignore failing contracts.
4. Run a synthetic loopback coding task through the actual assembled executable.
   Derive its temporary recipe from the baseline. Use isolated homes and files,
   observe event ordering, and never substitute mock sync for disk evidence.
5. Record exact upstream-sized gaps and proposed changes in the existing
   session-storage research, and update DESIGN/HANDOFF with verified limits.

## Boundaries and acceptance

No reference checkout edits, upstream patch implementation, fork, private Store,
second runtime, D1 state machine, persona/memory/wake/evolution work or publishing.
Real-provider acceptance remains deferred. New tests stay offline; a scripted
gateway is not a real provider. Process interruption does not prove power-loss
durability. Keep ordinary recipes free of the experimental plugin.

Delivery is complete when upgrade checks and the reproducible probe/report set
are complete. The D0 persistence gate passes only if all required contracts have
real-path evidence. A reproduced blocker is a successful investigation but a
failed contract: stop at the bounded upstream proposal before D1.

Implementation uses native execution in an isolated worktree, with a final
independent review. Run focused Cargo tests, existing smoke scripts, formatting
and diff checks. Commit the upgrade separately from probes and conclusions.

## Follow-up: session ownership at every entry (2026-10-31)

Status: implemented and verified, following the owner's request to continue after D0.
This closes the observed Vivy ownership gap; synced storage and required saved
state remain separate upstream work. The production dependency pin stays fixed.
Upstream comparison on this date found only catalog updates and the 0.46 release
after our pin, with no ECS/memory/launcher source fix for the reproduced gaps.

Use the existing `vivy-process` host support and Rig's `SessionDir::hold`.
Every generated main acquires a guard before constructing Rig or any plugin.
Direct runs hold the session lease themselves. A child of our actual launcher
checks the parent's run identity and held run/session leases, then acquires an
additional execution lock; this avoids relocking the parent's lease and prevents
duplicate children. Both guards last until execution exits. The parent retains
its existing session lease across startup, execution, switching and cleanup.
Parent identity and OS locks coordinate trusted processes; they are not a sandbox.
Description-only invocations open no session. Fork holds its source and target
leases and copies no process lock files.

Review refinements: an empty description variable does not mean describe mode.
Keep the execution lock outside the session tree and retain its file identity.
Ephemeral cleanup renames the directory while holding that external lock before
recursive deletion; a newly created session at the original path remains safe.
Upstream maintenance writers still use only Rig's lease and are outside the added
launcher-loss protection. No universal Store-writer guarantee is claimed.

Moving the only lease into the child would expose launcher preparation and
post-exit switching to races. Inheriting a platform-specific lock handle would
add more platform plumbing. A checked parent handoff plus a small execution lock
preserves the existing launcher protocol and keeps the change local.

- [x] Reproduce the direct-entry failure in the actual binary and test live-source fork refusal.
- [x] Add the host guard in `crates/vivy-process/src/session.rs`, offline tests
  for lock conflicts, invalid/stale handoffs, duplicate children and release.
- [x] Wire it into generated mains, reserve the host dependency, sanitize
  inherited launcher protocol state, and lock source/target during CLI fork.
- [x] Run root tests, builds, assembled ownership/cancellation/optional-recipe
  checks and the D0 probe; update its result from ownership gap to supported.
- [x] Review and commit the focused change with evidence and remaining limits.
