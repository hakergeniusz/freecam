// The block the interaction targets. While the camera's ray stands for the
// player's clicks (the aim helper's case), the game's own traversal runs
// along it — but out to the camera's hand, far past the player's reach, so
// the whole interaction pipeline (mining's breaking progress, placing,
// picking a block) acts on whatever the camera's ray hits, however far it
// flies. The integrated server may reject what is past the player's own
// reach; the attempt is the point. The tick's own cached target (set once
// a tick before the clicks run, so a creative break does not move the use
// that follows it) and every other pick stay the game's own.
fn freecam_pick(game: &crate::game::Game<'_>) -> core::ops::ControlFlow<Option<Hit>> {
    use crate::game::freecam as freecam;
    if !game.native_on(freecam::ID) || game.tick_hit.is_some() {
        return core::ops::ControlFlow::Continue(());
    }
    match freecam::aim() {
        Some((origin, dir)) => {
            core::ops::ControlFlow::Break(game.clip_ray(origin, dir, freecam::CAMERA_REACH))
        }
        None => core::ops::ControlFlow::Continue(()),
    }
}
