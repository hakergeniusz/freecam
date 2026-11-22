// A mapped key went down or up: the mod takes the fly keys while the camera
// is detached and the hotbar digits while a combo key is held — presses
// only, since a release must reach the game's own bookkeeping. The same
// keys feed the brain's held state through the tick's polling; this break
// keeps the game from walking the player with them.
// Copyright (c) 2026 hakergeniusz. MIT.
fn freecam_mapped(
    game: &mut crate::game::Game<'_>,
    key: winit::keyboard::KeyCode,
    pressed: bool,
    repeat: bool,
) -> core::ops::ControlFlow<Result<()>> {
    fn on(game: &crate::game::Game<'_>) -> bool {
        game.stored_setting(freecam::ID, "enabled")
            .is_none_or(|value| value.trim() != "false")
    }
    if !pressed || repeat || !on(game) || !game.playing() {
        return core::ops::ControlFlow::Continue(());
    }
    let Some(name) = horizon_client::keybinds::key_name(key) else {
        return core::ops::ControlFlow::Continue(());
    };
    if freecam::swallows(name) {
        core::ops::ControlFlow::Break(Ok(()))
    } else {
        core::ops::ControlFlow::Continue(())
    }
}
