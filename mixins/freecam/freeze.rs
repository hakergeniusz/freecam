// The player's simulation tick on this thread: skipped while the freecam
// freezes the player — no gravity, no momentum, no input drift (the gate
// is this thread's, set by its own game tick a moment before this runs;
// the integrated server's Game on the Server thread keeps its own gate,
// its freecam never flew, and its physics are nobody's freeze to stop).
// The tick's own keys decide the rest: whoever drives the player drives
// it — a bot's held keys, a stick's push. Mining and building in place
// live outside this tick (the clicks and the dig state machine run on the
// game's tick), so a driven player's job goes on while the body holds.
fn freecam_freeze(keys: crate::sprint::Keys) -> core::ops::ControlFlow<Result<()>> {
    if crate::motion::freecam_gate::skip_keys(keys) {
        core::ops::ControlFlow::Break(Ok(()))
    } else {
        core::ops::ControlFlow::Continue(())
    }
}
