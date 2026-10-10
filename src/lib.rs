//! # padmap
//!
//! Which controller is this, what are its buttons called, and how do you
//! remap it. No dependencies, no input reading: pair it with whatever reads
//! the pad (gilrs, SDL, the browser's Gamepad API, evdev) and it answers the
//! questions every launcher, emulator front-end and remapper has to answer
//! again:
//!
//! - [`identify`]: a pad's family and model from its USB ids and name.
//! - [`Layout`]: what each control is called on that family ("Cross", "ZR",
//!   "View"), and which button confirms.
//! - [`Remap`]: "this button acts as that one", kept a valid permutation.
//! - [`SdlMapping`]: an SDL controller mapping rewritten under a remap, ready
//!   for `SDL_GAMECONTROLLERCONFIG`, which SDL games, Wine and Proton read.
//!
//! ```
//! use padmap::{identify, Control, Family, Layout, Remap, SdlMapping};
//!
//! let pad = identify(Some(0x054c), Some(0x0ce6), "Wireless Controller");
//! assert_eq!(pad.family, Family::PlayStation);
//! assert_eq!(Layout(pad.family).label(Control::South).short, "Cross");
//!
//! let mut remap = Remap::new();
//! remap.assign(Control::South, Control::East); // Cross acts as Circle, and back
//! let line = "030000004c050000e60c000000010000,DualSense,a:b1,b:b2,x:b0,y:b3,platform:Linux,";
//! let out = SdlMapping::parse(line).unwrap().remapped(&remap);
//! assert_eq!(out.get("a"), Some("b2"));
//! ```

mod control;
mod identify;
mod layout;
mod remap;
mod sdl;

pub use control::Control;
pub use identify::{identify, is_generic_name, vendor, Family, Features, Matched, Model};
pub use layout::{Label, Layout};
pub use remap::Remap;
pub use sdl::{config_env, lookup_in, SdlError, SdlMapping};
#[cfg(feature = "sdl-db")]
pub use sdl::lookup;
