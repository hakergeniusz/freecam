// The perspective key: taken while the freecam holds the camera, so F5
// cannot disturb the detached view.
// Copyright (c) 2026 hakergeniusz. MIT.
fn freecam_cycle(game: &crate::game::Game<'_>) -> core::ops::ControlFlow<()> {
    fn on(game: &crate::game::Game<'_>) -> bool {
        game.stored_setting(freecam::ID, "enabled")
            .is_none_or(|value| value.trim() != "false")
    }
    if on(game) && freecam::is_active() {
        core::ops::ControlFlow::Break(())
    } else {
        core::ops::ControlFlow::Continue(())
    }
}
