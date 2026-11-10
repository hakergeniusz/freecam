// The F3 overlay's position group: while a camera flies, the lines describe
// the camera — its place in the world and its view — instead of the
// player's, who stands where the freeze holds them.
fn freecam_debug_lines(
    game: &crate::game::Game<'_>,
    lines: Vec<String>,
) -> Vec<String> {
    use crate::game::freecam as freecam;
    let Some((pos, yaw, pitch)) = freecam::view() else {
        return lines;
    };
    horizon_client::debug_overlay::position(
        pos,
        yaw,
        pitch,
        game.dim.id(),
        game.streaming.forced.len(),
    )
}
