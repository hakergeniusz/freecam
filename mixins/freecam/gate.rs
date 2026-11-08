// The freecam's freeze gate: the freecam's own thread asks, and every
// thread answers only for its own sims. Pure standard library, so it
// compiles wherever motion does — the game's own state is a world away
// from this crate.
//
// The flag lives per thread, and that is the whole point. A single-player
// world runs two Games in one process: the window's on its thread, and the
// integrated server's own Game on the "Server thread", whose tick drives
// this same motion method for the same player and whose freecam never
// flew. A process-wide flag would make the freeze a coin flip on thread
// scheduling — the server's tick clearing what the camera's thread set,
// and any other simulation that ever runs through the method freezing
// with it. Per thread, the freeze holds exactly the freecam's own
// thread's sim — the client's — and no other player's or thread's motion
// ever reads it.
thread_local! {
    /// The player stands still while the freecam asks (the game's tick on
    /// this thread writes this every tick, so a lost exit clears itself
    /// within a tick).
    static FREEZE: core::cell::Cell<bool> = const { core::cell::Cell::new(false) };
}

/// Whether this thread's motion tick should skip the player's simulation
/// entirely: no gravity, no momentum, no input drift. Movement input this
/// tick — a bot's held keys, a stick's push — runs the tick anyway, so
/// whoever drives the player drives it, and the freeze only holds an idle
/// player still.
pub fn skip(movement: bool) -> bool {
    frozen() && !movement
}

/// Whether the freecam asks this thread's sim to hold the player.
pub fn frozen() -> bool {
    FREEZE.with(|freeze| freeze.get())
}

/// The freecam's tick says whether this thread's sim should hold the
/// player. Another thread's Game — the integrated server's own, whose
/// freecam never flew — writes its own gate and can neither raise nor
/// clear this one.
pub fn set_freeze(on: bool) {
    FREEZE.with(|freeze| freeze.set(on));
}

#[cfg(test)]
mod tests {
    use super::*;
    // The staged crate's sim types: the injected module sees motion's own
    // items through its paths, not through the parent's imports.
    use crate::motion::{MotionResources, MotionWorld, PlayerMotion};
    use crate::world::World;

    /// A freeze set by the freecam's thread must survive the integrated
    /// server's thread: its Game's tick writes its own gate every tick
    /// (its freecam never flew, so it writes false), and that write is
    /// its own — the camera's thread stays frozen, and nothing on the
    /// server's thread freezes with it.
    #[test]
    fn the_server_threads_tick_leaves_the_cameras_freeze_alone() {
        set_freeze(true);
        assert!(skip(false), "the freecam's thread holds an idle sim");
        assert!(!skip(true), "a driven sim runs");
        std::thread::scope(|s| {
            // The Server thread's Game: its own tick, its own gate.
            s.spawn(|| {
                assert!(!skip(false), "no other thread's sim freezes");
                set_freeze(false);
                assert!(!skip(false));
            })
            .join()
            .unwrap();
        });
        assert!(
            skip(false),
            "the server thread's write must not clobber the camera's freeze"
        );
        assert!(!skip(true));
        set_freeze(false);
        assert!(!skip(false));
    }

    /// The reviewer's case, through the game's own motion code: the
    /// freecam's thread's freeze holds the camera's player in the air
    /// (no gravity), while a second player — simmed on a thread whose
    /// freecam never ran, as the integrated server's guests are — falls
    /// and lands, and the server thread's every-tick gate writes leave
    /// the camera's freeze standing.
    #[test]
    fn the_freeze_holds_the_camera_thread_and_not_the_second_player() {
        // The prepared resources carry the movement properties the sim
        // loads; the lan bot tests skip without them, and so does this.
        let root = std::env::var_os("HORIZON_TEST_RESOURCES")
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|| {
                std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("../../.local/horizon-client")
                    .join(horizon_core::VANILLA_VERSION)
            });
        if !root.join("movement-properties.nbt").exists() {
            eprintln!("skipped the freeze scope test: prepared resources unavailable");
            return;
        }
        let assets = crate::assets::Assets::load(&root).unwrap();
        let resources = MotionResources::load(&root, &assets).unwrap();
        let binding = crate::collisions::load(&root, assets.registry).unwrap();
        // A stone floor a few blocks down, like the lan bot tests' world.
        const STONE: u32 = 1;
        let floor = |scene: &mut World| {
            for cz in -1..=1 {
                for cx in -1..=1 {
                    let mut chunk = crate::chunk::Chunk::empty(cx, cz);
                    for z in 0..16 {
                        for x in 0..16 {
                            chunk.set(x, crate::chunk::MIN_Y + 4, z, STONE);
                        }
                    }
                    chunk.lit = true;
                    scene.chunks.insert((cx, cz), std::sync::Arc::new(chunk));
                }
            }
        };
        let mut scene = World::new(horizon_world::registry::BlockStateId(0), 0);
        floor(&mut scene);
        // Ten blocks above the floor: gravity has room to show itself.
        let air = horizon_core::geometry::Vec3::new(0.5, f64::from(crate::chunk::MIN_Y + 14), 0.5);
        let tick_idle = |player: &mut PlayerMotion,
                         scene: &mut World,
                         assets: &crate::assets::Assets,
                         binding: &_,
                         resources: &_| {
            MotionWorld {
                entities: &[],
                player,
                scene,
                assets,
                collision_binding: binding,
                resources,
            }
            .controlled_simulation_tick(crate::sprint::Keys::default())
            .unwrap();
        };

        // Sanity: the sim's own gravity drops an unfrozen player onto the
        // floor (the same world, the same idle tick).
        let mut falling = PlayerMotion::new(&resources, air).unwrap();
        for _ in 0..60 {
            tick_idle(&mut falling, &mut scene, &assets, &binding, &resources);
        }
        assert!(
            falling.ground.on_ground,
            "the harness's gravity must land an idle player"
        );

        // The camera's player, frozen in the same air: the sim is skipped
        // whole, so nothing moves them.
        let mut frozen_player = PlayerMotion::new(&resources, air).unwrap();
        set_freeze(true);
        for _ in 0..60 {
            tick_idle(&mut frozen_player, &mut scene, &assets, &binding, &resources);
        }
        assert_eq!(
            frozen_player.position.y, air.y,
            "the freeze holds them in the air"
        );

        // The second player sims on a thread of its own — the integrated
        // server's — whose freecam never ran. Its ticks run, its player
        // falls, and its Game's every-tick gate write is its own.
        std::thread::scope(|s| {
            s.spawn(move || {
                let assets = crate::assets::Assets::load(&root).unwrap();
                let resources = MotionResources::load(&root, &assets).unwrap();
                let binding = crate::collisions::load(&root, assets.registry).unwrap();
                let mut scene = World::new(horizon_world::registry::BlockStateId(0), 0);
                floor(&mut scene);
                let mut second = PlayerMotion::new(&resources, air).unwrap();
                for _ in 0..60 {
                    tick_idle(
                        &mut second,
                        &mut scene,
                        &assets,
                        &binding,
                        &resources,
                    );
                    // The server's Game writes its own gate every tick.
                    set_freeze(false);
                }
                assert!(
                    second.ground.on_ground,
                    "the second player's gravity is nobody's freeze to stop"
                );
            })
            .join()
            .unwrap();
        });

        // And the camera's freeze outlived all of it.
        for _ in 0..10 {
            tick_idle(&mut frozen_player, &mut scene, &assets, &binding, &resources);
        }
        assert_eq!(
            frozen_player.position.y, air.y,
            "the server thread's ticks never touched the camera's gate"
        );
        set_freeze(false);
    }
}
