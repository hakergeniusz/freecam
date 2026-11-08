// The player's simulation tick on this thread: skipped while the freecam
// freezes the player — no gravity, no momentum, no input drift (the gate
// is this thread's, set by its own game tick a moment before this runs;
// the integrated server's Game on the Server thread keeps its own gate,
// its freecam never flew, and its physics are nobody's freeze to stop).
// Movement input this tick — a bot's held keys, a stick's push — runs the
// tick anyway: whoever drives the player drives it, and the freeze only
// holds an idle player.
fn freecam_freeze(keys: crate::sprint::Keys) -> core::ops::ControlFlow<Result<()>> {
    let movement = keys.forward || keys.backward || keys.left || keys.right || keys.jump;
    if crate::motion::freecam_gate::skip(movement) {
        core::ops::ControlFlow::Break(Ok(()))
    } else {
        core::ops::ControlFlow::Continue(())
    }
}
