// The local player's body tick: skipped while the freecam freezes the
// player — no gravity, no momentum, no input drift. (The sim the old gate
// held — controlled_simulation_tick_with_sprint_window — moved into a
// crate the launcher's staged build cannot patch; this caller is the same
// freeze one step up, in the game crate. It also holds the cooldown and
// sleep clocks while it holds the body, and a driven tick — the keys, a
// stick's push — runs everything, so whoever drives the player drives it.)
// The keys are the ones the tick itself builds: the held actions behind
// its playing gate, and the pad's stick. Mining and building in place
// live outside this tick (the clicks and the dig state machine run on the
// game's tick), so a driven player's job goes on while the body holds.
// The gate is this thread's, set by its own game tick a moment before
// this runs; the integrated server's Game keeps its own, its freecam
// never flew, and its physics are nobody's freeze to stop.
// Copyright (c) 2026 hakergeniusz. MIT.
fn freecam_freeze(game: &mut crate::game::Game<'_>) -> core::ops::ControlFlow<Result<()>> {
    // The playing gate and the keys the body tick builds its sim's input
    // from, kept in step with that tick's own expressions.
    let loading = game
        .remote
        .as_ref()
        .is_some_and(|r| r.level_load.is_active());
    let playing = game.screen == Screen::Playing && !loading && !game.dialog_open();
    let held = |action| playing && game.keys.held(action);
    let driven = held(Action::Forward)
        || held(Action::Back)
        || held(Action::Left)
        || held(Action::Right)
        || held(Action::Jump)
        || (playing && game.pad_move().is_some());
    if freecam::gate::skip(driven) {
        core::ops::ControlFlow::Break(Ok(()))
    } else {
        core::ops::ControlFlow::Continue(())
    }
}
