# Freecam for Horizon

Control your camera separately from your player. A [Horizon](https://github.com/horizon-rust/horizon)
native mod ported from the [Freecam](https://modrinth.com/mod/freecam) mod
([MIT](https://github.com/MinecraftFreecam/Freecam)).

While the freecam is active your view flies freely — through walls, at any
speed — while your player stays where they were. Tripod cameras let you keep
up to nine saved spots and jump between them.

## Keys

| Key | Action |
| --- | --- |
| `F4` | Toggle the freecam (rebindable in Options > Mods) |
| `F4` + `1`–`9` | Open tripod camera n (keeps its spot when closed) |
| Reset key + `1`–`9` | Reset tripod camera n to the player (rebindable; unbound by default) |

The same movement keys fly the camera (the player's are taken while
detached), `Left Ctrl` sprints, the mouse turns the camera, and `F5` is
ignored while detached so it cannot disturb the view.

## Settings (Options > Mods > Freecam)

- **Flight mode** — `Default` flies at exactly the set speed; `Creative`
  eases into its target with momentum, like vanilla creative flight.
- **Horizontal / vertical speed** — 0.25× to 8×.
- **Initial perspective** — where the camera starts: first person (just in
  front of the eyes), third person (4 blocks behind), or mirrored.
- **Show player** — draw your own body while detached.
- **Disable on damage** — return the camera when the player is hurt.
- **Freeze player** — the player's whole simulation tick is skipped while
  detached: no gravity, no momentum, no drift.
- **Interaction** — `Off` (the player neither digs nor places), `Camera`
  (target and interact from the camera), or `Player` (the game's own ray).
- **Notify / Notify tripods** — the on-screen toasts, per camera kind.

Settings are stored per install through Horizon's mod settings.

## Install

The `.hnmod` is source-only: Horizon's launcher compiles it against the
pinned Horizon revision on install.

```sh
python3 tools/package.py   # writes target/freecam-0.1.0.hnmod
```

## Development

The mod's logic lives in `mixins/freecam/runtime.rs` — pure standard
library, no game types — which doubles as this repository's tested library
and as the module Horizon injects into the game. The other files in
`mixins/freecam/` are small helpers that bridge the game to it; they are
injected into the game's sources by [horizon-mixins](https://github.com/horizon-rust/horizon-mixins)
and compile only there.

```sh
python3 tools/test.py                                # fmt, clippy, the runtime's tests
HORIZON_MIXINS=…/horizon-mixins python3 tools/horizon.py stage --horizon …/horizon
HORIZON_MIXINS=…/horizon-mixins python3 tools/horizon.py check --horizon …/horizon
HORIZON_MIXINS=…/horizon-mixins python3 tools/horizon.py test  --horizon …/horizon
```

`compatibility.json` pins the Horizon revision and the hashes of the four
patched sources (`game.rs`, `game/native_hooks.rs`, `game/perspective.rs`,
`motion.rs`); a Horizon change there needs the pins re-checked.

## Ported from the original

- The static-velocity motion (diagonal spread, sprint's 1.5×, speeds as
  blocks per tick), and the creative flight's easing as the port's take on
  vanilla's momentum.
- Tripods: spots kept per slot, the active one moved to the player on reset,
  idle ones forgotten, the flown freecam spot discarded when a tripod opens,
  the `Opening/Closing/Reset camera %s` toasts.
- Damage and world-close exits; the close is silent, the tripods forget.
- The dummy input on entry (the player's held fly keys are dropped so they
  do not walk on), and the view-kind remember/restore around the detach.

## Not ported (yet)

- Collision for the camera itself (the original's `ignoreAll` lower bound —
  this port always ignores blocks).
- The original's `showHand`, `showSubmersion` and `outlinePlayer` overlays.
- `controlPlayer` (moving the player from the freecam) and full-brightness.
- Server-side restriction hooks and the chunk-load check before a tripod
  restores far from the player (spot restores trust the chunks are there).
- Controller sticks are not intercepted; pad movement still moves the player.
