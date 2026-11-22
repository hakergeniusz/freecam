// The player's view turned: the freecam's turn while it holds the camera,
// the player's own otherwise.
// Copyright (c) 2026 hakergeniusz. MIT.
fn freecam_turn(game: &crate::game::Game<'_>, xo: f64, yo: f64) -> core::ops::ControlFlow<()> {
    fn on(game: &crate::game::Game<'_>) -> bool {
        game.stored_setting(freecam::ID, "enabled")
            .is_none_or(|value| value.trim() != "false")
    }
    if on(game) && freecam::turn(xo, yo) {
        core::ops::ControlFlow::Break(())
    } else {
        core::ops::ControlFlow::Continue(())
    }
}
