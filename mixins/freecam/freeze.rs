// The player's simulation tick: skipped entirely while the freecam freezes
// the player — no gravity, no momentum, no input drift (the gate is set by
// the game's tick, a moment before this runs).
fn freecam_freeze() -> core::ops::ControlFlow<Result<()>> {
    if crate::motion::freecam_gate::frozen() {
        core::ops::ControlFlow::Break(Ok(()))
    } else {
        core::ops::ControlFlow::Continue(())
    }
}
