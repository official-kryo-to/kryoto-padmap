# padmap reference

## What it does

| | |
|---|---|
| `identify(vendor, product, name)` | Tells a pad's **family** (Xbox, PlayStation, Nintendo, or generic) and **model** (DualSense Edge, Xbox Elite Series 2, Switch Pro Controller, Steam Deck…) from its USB ids. When there are none, as with XInput, it falls back to the pad's name, then to its maker. It also reports how it knew. |
| `is_generic_name(name)` | Tells the system's placeholder for an unknown pad (Windows' "HID-compliant game controller" in any language, DirectInput's "6 axis 16 button gamepad with hat switch") from a real name, so a screen can show the maker's name instead (`identify` already does: an unknown SteelSeries pad is "SteelSeries controller"). |
| `Layout(family).label(control)` | What each control is **called** on that pad: `A` / `Cross` / `B` for the bottom button, `RT` / `R2` / `ZR`, `View` / `Create` / `-`. Also which button **confirms** in menus (A on the right on Nintendo pads). |
| `Remap` | "This button acts as that one", always kept a **valid permutation**: giving a button a new job hands its old job to the button that had it. No job is ever lost or doubled. |
| `SdlMapping` | Parses an SDL game controller mapping and **rewrites it under a `Remap`**. The result goes in `SDL_GAMECONTROLLERCONFIG`, which SDL games read, and so do Wine and Proton, whose controller driver is built on SDL. Steam passes its remaps to games on Linux the same way. |
| `lookup(guid)` (feature `sdl-db`) | Finds the mapping for a Linux pad's SDL GUID in the bundled [SDL_GameControllerDB](https://github.com/mdqinc/SDL_GameControllerDB) (zlib licence). It matches the way SDL does: exactly, then ignoring the name CRC, then ignoring the version. |

Controls are named **by where they are, not by what is printed on them**: `Control::South` is the bottom face button on every pad. SDL names them the same way (its `a`), and it is the only naming under which "swap A and B" means the same thing on every controller.

## The drawings

`art/` holds each family as a pixel-art SVG, 112 × 64 art pixels, each in its own colours: a carbon Xbox pad, a white and black DualSense with its light bars, a black Switch Pro, a grey universal pad. Light comes from the top left, the grips fade into a dithered shade, and every key casts a one-pixel shadow. Every control is its own `<g id="…">` named by its SDL name (`a`, `leftshoulder`, `dpup`…), so a page can light up what is pressed. Kryoto Desktop draws them live from `src/lib/pad-art.ts` in kryoto-desktop. The home button wears Kryoto's K// mark.
