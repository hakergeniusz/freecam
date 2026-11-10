// Where targeting starts and points: the freecam's ray while the camera is
// detached and the interaction asks for it, the game's own otherwise. The
// pick helper carries the same ray's reach past the player's own, so the
// whole interaction pipeline acts on what this ray hits.
fn freecam_aim(game: &crate::game::Game<'_>) -> core::ops::ControlFlow<Option<([f64; 3], [f64; 3])>> {
    use crate::game::freecam as freecam;
    if !game.native_on(freecam::ID) {
        return core::ops::ControlFlow::Continue(());
    }
    match freecam::aim() {
        Some(aim) => core::ops::ControlFlow::Break(Some(aim)),
        None => core::ops::ControlFlow::Continue(()),
    }
}
