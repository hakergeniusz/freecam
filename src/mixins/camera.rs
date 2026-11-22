// The view the frame renders from: the freecam's while it holds the camera,
// the game's own otherwise.
// Copyright (c) 2026 hakergeniusz. MIT.
fn freecam_camera(
    game: &crate::game::Game<'_>,
) -> core::ops::ControlFlow<Option<(glam::DVec3, f32, f32)>> {
    fn on(game: &crate::game::Game<'_>) -> bool {
        game.stored_setting(freecam::ID, "enabled")
            .is_none_or(|value| value.trim() != "false")
    }
    if !on(game) {
        return core::ops::ControlFlow::Continue(());
    }
    match freecam::view() {
        Some(([x, y, z], yaw, pitch)) => {
            core::ops::ControlFlow::Break(Some((glam::DVec3::new(x, y, z), yaw, pitch)))
        }
        None => core::ops::ControlFlow::Continue(()),
    }
}
