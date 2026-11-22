// Once a tick, on every Game: the mod's two keys poll through their
// registered mappings' held state and the window's physical keys stand
// where native_key's events stood — each edge becomes one synthetic key
// event the brain answers as it always did (the toggle enters and exits,
// the combos with the digits open and reset tripods, the fly keys are
// tracked while the camera flies). The stored settings refresh into the
// brain every tick, so a change on the mod's page lands without a restart.
// Then the tick's own work: the flight runs here too, one tick's worth —
// the per-frame hook's dispatch never reached its helper on the staged
// build (the toggle and the view's turn both worked on live seams while
// the frame hook's movement moved no pixel), so the camera flies from the
// tick, at the brain's per-tick speeds — and the exits on damage or a
// closed world (the player's view kind comes back), the clean turn-off
// when the mod is disabled, and the freeze gate the motion tick reads.
//
// The gate is this thread's, so the integrated server's own Game — whose
// freecam never flew — writes its own and cannot clear this one.
// Copyright (c) 2026 hakergeniusz. MIT.
fn freecam_tick(game: &mut crate::game::Game<'_>) {
    // The player's view kind comes back where it was.
    fn restore_camera(game: &mut crate::game::Game<'_>) {
        game.camera_type = match freecam::camera_kind() {
            1 => horizon_client::camera::CameraType::ThirdPersonBack,
            2 => horizon_client::camera::CameraType::ThirdPersonFront,
            _ => horizon_client::camera::CameraType::FirstPerson,
        };
    }
    // A stored choice index becomes the brain's canonical stored value;
    // anything else passes through (the values the old config kept).
    fn indexed(raw: &str, names: &[&str]) -> String {
        raw.trim()
            .parse::<usize>()
            .ok()
            .and_then(|i| names.get(i).map(|name| (*name).to_string()))
            .unwrap_or_else(|| raw.to_string())
    }
    // The stored values, read through the game's settings store with the
    // choices' indices translated into the names the brain's
    // `Settings::read` parses.
    fn stored<'a>(
        game: &'a crate::game::Game<'a>,
    ) -> impl Fn(&str) -> Option<String> + 'a {
        move |key| {
            let raw = game.stored_setting(freecam::ID, key)?;
            Some(match key {
                "flight_mode" => indexed(&raw, &["default", "creative"]),
                "perspective" => indexed(
                    &raw,
                    &["inside", "first_person", "third_person", "third_person_mirror"],
                ),
                "interaction" => indexed(&raw, &["off", "camera", "player"]),
                "horizontal_speed" | "vertical_speed" => freecam::SPEEDS
                    .get(raw.trim().parse::<usize>().ok()?)
                    .map(|speed| format!("{speed}"))
                    .unwrap_or(raw),
                _ => raw,
            })
        }
    }
    // Whether the mod is on: a missing setting means on; only `false` turns
    // it off (fly's rule).
    fn on(game: &crate::game::Game<'_>) -> bool {
        game.stored_setting(freecam::ID, "enabled")
            .is_none_or(|value| value.trim() != "false")
    }
    // The player snapshot the brain's key() starts a camera from.
    fn player(game: &crate::game::Game<'_>) -> Option<freecam::Player> {
        game.player.as_ref().map(|p| freecam::Player {
            pos: [p.position.x, p.position.y, p.position.z],
            eye_height: p.dimensions.eye_height,
            yaw: game.yaw,
            pitch: game.pitch,
        })
    }
    // The key edges since the last tick, as (key name, pressed) events the
    // brain's key() answers: the mod's two keys read their registered
    // mappings' held state (rebinding lives there), the digits and the fly
    // keys read the window's physical set — the fixed keys, as the original
    // kept them.
    fn events(game: &crate::game::Game<'_>) -> Vec<(&'static str, bool)> {
        use winit::keyboard::KeyCode;
        thread_local! {
            /// The keys' state as the last tick saw it.
            static PREV: core::cell::RefCell<std::collections::HashMap<&'static str, bool>> =
                core::cell::RefCell::new(std::collections::HashMap::new());
            /// Whether a world was open, as the last tick saw it: a close or
            /// a fresh open resets the baseline below.
            static WORLD: core::cell::Cell<Option<bool>> = const { core::cell::Cell::new(None) };
            /// How long each key has stood down, in ticks.
            static HOLD: core::cell::RefCell<std::collections::HashMap<&'static str, u32>> =
                core::cell::RefCell::new(std::collections::HashMap::new());
            /// The registered mappings a lost release left down, retired
            /// until the mapping really lets go.
            static RETIRED: core::cell::RefCell<std::collections::HashSet<&'static str>> =
                core::cell::RefCell::new(std::collections::HashSet::new());
        }
        let open = game.player.is_some();
        let fresh = WORLD.with(|world| world.replace(Some(open)) != Some(open));
        let down = [
            (
                freecam::DEFAULT_TOGGLE_KEY,
                game.is_down(freecam::ID, freecam::TOGGLE),
            ),
            (
                freecam::DEFAULT_RESET_KEY,
                game.is_down(freecam::ID, freecam::TRIPOD_RESET),
            ),
            (freecam::DIGITS[0], game.physical_keys.contains(&KeyCode::Digit1)),
            (freecam::DIGITS[1], game.physical_keys.contains(&KeyCode::Digit2)),
            (freecam::DIGITS[2], game.physical_keys.contains(&KeyCode::Digit3)),
            (freecam::DIGITS[3], game.physical_keys.contains(&KeyCode::Digit4)),
            (freecam::DIGITS[4], game.physical_keys.contains(&KeyCode::Digit5)),
            (freecam::DIGITS[5], game.physical_keys.contains(&KeyCode::Digit6)),
            (freecam::DIGITS[6], game.physical_keys.contains(&KeyCode::Digit7)),
            (freecam::DIGITS[7], game.physical_keys.contains(&KeyCode::Digit8)),
            (freecam::DIGITS[8], game.physical_keys.contains(&KeyCode::Digit9)),
            (freecam::KEY_FORWARD, game.physical_keys.contains(&KeyCode::KeyW)),
            (freecam::KEY_BACK, game.physical_keys.contains(&KeyCode::KeyS)),
            (freecam::KEY_LEFT, game.physical_keys.contains(&KeyCode::KeyA)),
            (freecam::KEY_RIGHT, game.physical_keys.contains(&KeyCode::KeyD)),
            (freecam::KEY_UP, game.physical_keys.contains(&KeyCode::Space)),
            (freecam::KEY_DOWN, game.physical_keys.contains(&KeyCode::ShiftLeft)),
            (
                freecam::KEY_SPRINT,
                game.physical_keys.contains(&KeyCode::ControlLeft),
            ),
        ];
        PREV.with(|prev_cell| {
            // A world's close or a fresh open resets the baseline: the game
            // drops its key state along the way, and a release lost while
            // the window was unfocused (one winit never sends) leaves what
            // stood a lie. The first tick of a world starts from what is
            // down now, not from what was.
            let mut hold = HOLD.with(|hold| std::mem::take(&mut *hold.borrow_mut()));
            let mut retired = RETIRED.with(|retired| std::mem::take(&mut *retired.borrow_mut()));
            let mut prev = prev_cell.borrow_mut();
            if fresh {
                prev.clear();
                hold.clear();
                retired.clear();
            }
            let mut out = Vec::new();
            for (name, now) in down {
                let mapping =
                    matches!(name, freecam::DEFAULT_TOGGLE_KEY | freecam::DEFAULT_RESET_KEY);
                if now {
                    *hold.entry(name).or_insert(0) += 1;
                } else {
                    hold.remove(name);
                    retired.remove(name);
                }
                // The registered mappings' held state is bookkeeping a lost
                // release can corrupt: the mapping then stands down for good,
                // and every later press is no edge — the key stops working. A
                // down that has outlasted any press is retired: one synthetic
                // release, and presses count again from the next real one.
                if mapping
                    && now
                    && hold.get(name).copied().unwrap_or(0) > freecam::STALE_TICKS
                {
                    if retired.insert(name) {
                        prev.insert(name, false);
                        out.push((name, false));
                    }
                    continue;
                }
                let was = prev.insert(name, now).unwrap_or(false);
                if now != was {
                    out.push((name, now));
                }
            }
            HOLD.with(|cell| *cell.borrow_mut() = hold);
            RETIRED.with(|cell| *cell.borrow_mut() = retired);
            out
        })
    }

    // The settings refresh first: the page's stored values into the brain.
    let settings = freecam::Settings::read(stored(game));
    freecam::set_settings(settings);
    // Turned off: the camera returns, silently, and the freeze gate clears
    // (this runs even while the world is gone).
    if !on(game) {
        if freecam::disable().is_some() {
            restore_camera(game);
        }
        freecam::gate::set_freeze(false);
        return;
    }
    // The key events: the toggle enters and exits (Enter's side effects
    // here — the remembered view kind, the third-person body, the fly keys
    // dropped so the player stands still), the combos with the digits open
    // and reset tripods, the fly keys are tracked. The command acts where
    // the &mut game is; the getter's borrow ends with the call.
    let playing = game.playing();
    let snapshot = player(game);
    for (name, pressed) in events(game) {
        let command = freecam::key(
            playing,
            name,
            pressed,
            freecam::DEFAULT_TOGGLE_KEY,
            freecam::DEFAULT_RESET_KEY,
            snapshot,
            stored(game),
        );
        match command {
            Some(freecam::Command::Enter) => {
                use winit::keyboard::KeyCode;
                freecam::remember_camera(match game.camera_type {
                    horizon_client::camera::CameraType::ThirdPersonBack => 1,
                    horizon_client::camera::CameraType::ThirdPersonFront => 2,
                    horizon_client::camera::CameraType::FirstPerson => 0,
                });
                game.camera_type = horizon_client::camera::CameraType::ThirdPersonBack;
                for key in [
                    KeyCode::KeyW,
                    KeyCode::KeyS,
                    KeyCode::KeyA,
                    KeyCode::KeyD,
                    KeyCode::Space,
                ] {
                    game.keys.release(&key, horizon_client::held::Source::Keyboard);
                }
            }
            Some(freecam::Command::Exit) => restore_camera(game),
            None => {}
        }
    }
    // The tick's exits: on damage or a closed world the view kind comes
    // back (the world's close resets silently).
    let world_open = game.player.is_some();
    let health = game.player.as_ref().map(|p| p.health.health());
    if let Some(freecam::Command::Exit) = freecam::tick(world_open, health) {
        restore_camera(game);
    }
    // The flight itself: one tick's worth, here, where the key edges land —
    // the per-frame hook's dispatch never reached its helper on the staged
    // build, so the movement runs on the tick, at the brain's per-tick
    // speeds. A closed world's tick exits first, and the brain moves
    // nothing once the camera is back.
    freecam::frame(freecam::TICK_DURATION, playing);
    // The player freezes inside this thread's motion tick, which runs
    // after this; the gate written here is this thread's.
    freecam::gate::set_freeze(freecam::freezes_player());
}
