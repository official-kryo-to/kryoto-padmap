# padmap

Which controller is this, what are its buttons called, and how do you remap it? A small,
dependency-free Rust crate; pair it with whatever reads the controller (gilrs, SDL, evdev, the
Gamepad API).

**Kryoto** · Rust · MIT · [All repos](https://github.com/Kryoto-Dev)

![The four layouts padmap knows, drawn as pixel art](art/all.png)

## Quick start

```rust
use padmap::{identify, Control, Family, Layout, Remap, SdlMapping};

// A DualSense over Bluetooth calls itself "Wireless Controller".
let pad = identify(Some(0x054c), Some(0x0ce6), "Wireless Controller");
assert_eq!(pad.family, Family::PlayStation);
assert_eq!(pad.name, "DualSense");
assert_eq!(Layout(pad.family).label(Control::South).short, "Cross");

// Swap the bottom and right buttons, for games.
let mut remap = Remap::new();
remap.assign(Control::South, Control::East);

let base = padmap::lookup("030000004c050000e60c000011810000").unwrap(); // feature "sdl-db"
let line = base.remapped(&remap).to_line();
std::env::set_var("SDL_GAMECONTROLLERCONFIG", padmap::config_env([line.as_str()]));
```

For an SDL remap to reach a game, SDL has to see the pad through the same driver the mapping was written for. On Linux, also set `SDL_JOYSTICK_HIDAPI=0` (SDL's HIDAPI drivers address PlayStation and Switch pads by another GUID) and, for Proton, `PROTON_PREFER_SDL=1`.

Controls are named by position, not by what is printed on them: `Control::South` is the bottom
face button on every pad. Full API: [docs/reference.md](docs/reference.md).

## Features

- `serde`: `Serialize` / `Deserialize` for the public types. A `Remap` stores as `{"a":"b","b":"a"}`.
- `sdl-db`: the Linux half of SDL_GameControllerDB (about 200 KB) and `lookup`.

## Layout

| Path | What's there |
|---|---|
| `src/` | identify, layouts, remaps, SDL mappings |
| `art/` | each controller family as pixel-art SVG |
| `data/` | the bundled Linux SDL_GameControllerDB |
| `scripts/update-sdl-db.sh` | refresh that database |

## Checks

```bash
cargo test --all-features
```

## License

MIT, see [LICENSE](LICENSE). `data/gamecontrollerdb-linux.txt` is from SDL_GameControllerDB, under
the zlib licence ([data/LICENSE-SDL_GameControllerDB](data/LICENSE-SDL_GameControllerDB)).
