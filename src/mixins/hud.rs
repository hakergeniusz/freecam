// The toast on screen: the last message, centered above the hotbar in a
// box, as the original's notification overlay did. The HUD hook runs the
// vanilla layers' post-draw; the crosshair's is the one above the hotbar.
// Copyright (c) 2026 hakergeniusz. MIT.
fn freecam_hud(game: &crate::game::Game<'_>, layer: &str, batch: &mut HudBatch) {
    fn on(game: &crate::game::Game<'_>) -> bool {
        game.stored_setting(freecam::ID, "enabled")
            .is_none_or(|value| value.trim() != "false")
    }
    if layer != "minecraft:crosshair" || !on(game) {
        return;
    }
    let Some(text) = freecam::toast() else {
        return;
    };
    let width = game.native_text_width(&*batch, &text);
    let x = (batch.width - width) / 2.0;
    let y = batch.height - 59.0;
    game.native_fill(batch, [x - 4.0, y - 2.0, width + 8.0, 12.0], 0x78000000);
    game.native_text(batch, &text, [x, y], 0xFFFFFF, true);
}
