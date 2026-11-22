// Once a frame, while a world renders: the camera flies while the world is
// played, and holds when a screen opens (its key clearings are honored
// through the brain). The frame's elapsed time lives here now — the hook
// this replaces (native_frame) carried it, the frame-visuals hook does not.
// Copyright (c) 2026 hakergeniusz. MIT.
fn freecam_frame(game: &crate::game::Game<'_>) {
    thread_local! {
        /// The instant the last frame rendered.
        static LAST: core::cell::Cell<Option<std::time::Instant>> =
            const { core::cell::Cell::new(None) };
    }
    let now = std::time::Instant::now();
    let elapsed = LAST
        .with(|last| last.replace(Some(now)))
        .map_or(std::time::Duration::ZERO, |at| now - at);
    freecam::frame(elapsed, game.playing());
}
