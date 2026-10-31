# D0 implementation: Rig upgrade and persistence seam probes

Status: implemented; final review pending. D0 persistence gate: blocked by
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
