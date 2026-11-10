// A click in the world, where the game still knows which device sent it:
// the mapping's source says whose hand is on the button. The keyboard's
// and the pad's are the person at the machine, and their presses and
// releases count toward the camera's aim (and are eaten while the freecam
// holds the camera and the interaction is Off, so the player neither digs
// nor places from a distance). The agent source's clicks — horizon-ctl, a
// script, a driving bot's — count for nothing and are never eaten, so
// whoever drives the player clicks with the player's own targeting.
fn freecam_mouse(
    game: &mut crate::game::Game<'_>,
    button: winit::event::MouseButton,
    pressed: bool,
    source: horizon_client::held::Source,
) -> core::ops::ControlFlow<Result<()>> {
    use crate::game::freecam as freecam;
    let _ = button;
    let human = !matches!(source, horizon_client::held::Source::Agent);
    freecam::mouse_click(pressed, human);
    if human
        && game.native_on(freecam::ID)
        && game.native_playing()
        && freecam::blocks_interaction()
    {
        core::ops::ControlFlow::Break(Ok(()))
    } else {
        core::ops::ControlFlow::Continue(())
    }
}
