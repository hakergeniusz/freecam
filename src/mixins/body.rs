// The player's own body in view: hidden while the freecam holds the camera
// and the setting asks.
// Copyright (c) 2026 hakergeniusz. MIT.
fn freecam_body(game: &crate::game::Game<'_>) -> core::ops::ControlFlow<bool> {
    fn on(game: &crate::game::Game<'_>) -> bool {
        game.stored_setting(freecam::ID, "enabled")
            .is_none_or(|value| value.trim() != "false")
    }
    if on(game) && freecam::hides_player() {
        core::ops::ControlFlow::Break(false)
    } else {
        core::ops::ControlFlow::Continue(())
    }
}
