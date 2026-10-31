// A click in the world: eaten while the freecam holds the camera and the
// interaction is Off, so the player neither digs nor places from a distance.
fn freecam_mouse(
    game: &crate::game::Game<'_>,
    button: winit::event::MouseButton,
) -> core::ops::ControlFlow<Result<()>> {
    use crate::game::freecam as freecam;
    if game.native_on(freecam::ID)
        && game.native_playing()
        && freecam::blocks_interaction()
    {
        let _ = button;
        core::ops::ControlFlow::Break(Ok(()))
    } else {
        core::ops::ControlFlow::Continue(())
    }
}
