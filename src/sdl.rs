//! SDL game controller mappings: reading them and rewriting them under a
//! [`Remap`].
//!
//! A mapping is one line: `GUID,name,a:b0,b:b1,...,platform:Linux,`. SDL
//! reads extra lines from the `SDL_GAMECONTROLLERCONFIG` environment variable
//! (one per line) before its own database, which is how Steam hands its
//! remaps to games: SDL games read it directly, and so do Wine and Proton,
//! whose controller driver (winebus) is built on SDL.

use crate::{Control, Remap};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SdlMapping {
    pub guid: String,
    pub name: String,
    /// `key:source` pairs in their original order (`a` -> `b0`), including
    /// axes (`leftx`), `platform` and anything this crate does not know.
    pub fields: Vec<(String, String)>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SdlError {
    /// Fewer than the GUID and name, or a GUID that is not 32 hex digits.
    Malformed,
}

impl SdlMapping {
    pub fn parse(line: &str) -> Result<SdlMapping, SdlError> {
        let mut parts = line.trim().split(',');
        let guid = parts.next().ok_or(SdlError::Malformed)?.trim().to_string();
        let name = parts.next().ok_or(SdlError::Malformed)?.trim().to_string();
        if guid.len() != 32 || !guid.bytes().all(|b| b.is_ascii_hexdigit()) {
            return Err(SdlError::Malformed);
        }
        let fields = parts
            .filter(|p| !p.trim().is_empty())
            .filter_map(|p| p.split_once(':'))
            .map(|(k, v)| (k.trim().to_string(), v.trim().to_string()))
            .collect();
        Ok(SdlMapping { guid, name, fields })
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.fields.iter().find(|(k, _)| k == key).map(|(_, v)| v.as_str())
    }

    /// This mapping with each control's input moved to the job it now does:
    /// if South acts as East, the input that was `a` becomes `b`. Controls the
    /// pad does not have stay absent; a job whose control is absent is left
    /// unbound rather than invented.
    pub fn remapped(&self, remap: &Remap) -> SdlMapping {
        let source_of = |c: Control| self.get(c.sdl_name()).map(str::to_string);
        let mut fields: Vec<(String, String)> = self
            .fields
            .iter()
            .filter(|(k, _)| Control::from_sdl_name(k).is_none())
            .cloned()
            .collect();
        // Control fields first in SDL's own order, then the rest as they were.
        let mut controls: Vec<(String, String)> = Vec::new();
        for job in Control::ALL {
            if let Some(src) = source_of(remap.done_by(job)) {
                controls.push((job.sdl_name().to_string(), src));
            }
        }
        controls.append(&mut fields);
        SdlMapping { guid: self.guid.clone(), name: self.name.clone(), fields: controls }
    }

    pub fn to_line(&self) -> String {
        let mut s = format!("{},{},", self.guid, self.name.replace(',', " "));
        for (k, v) in &self.fields {
            s.push_str(k);
            s.push(':');
            s.push_str(v);
            s.push(',');
        }
        s
    }
}

/// The Linux half of the SDL_GameControllerDB (data/, zlib licence).
#[cfg(feature = "sdl-db")]
const LINUX_DB: &str = include_str!("../data/gamecontrollerdb-linux.txt");

/// The SDL_GameControllerDB's mapping for a Linux pad, by its SDL GUID (32 hex
/// digits: bus, name CRC, vendor, product, version, driver). Matched the way
/// SDL matches: exactly, then ignoring the name CRC, then ignoring the
/// version too. `None` for pads the database does not list (SDL maps those
/// by itself, from the standard button codes, and so leaves nothing to
/// rewrite).
#[cfg(feature = "sdl-db")]
pub fn lookup(guid: &str) -> Option<SdlMapping> {
    lookup_in(LINUX_DB, guid)
}

/// [`lookup`] in a database of your own (one mapping per line, `#` comments).
pub fn lookup_in(db: &str, guid: &str) -> Option<SdlMapping> {
    let want = guid.trim().to_ascii_lowercase();
    if want.len() != 32 {
        return None;
    }
    // Hex digit ranges: bytes 2-3 are the CRC, bytes 12-13 the version.
    let blank = |g: &str, version: bool| {
        let mut b = g.to_string();
        b.replace_range(4..8, "0000");
        if version {
            b.replace_range(24..28, "0000");
        }
        b
    };
    let lines = || {
        db.lines()
            .map(str::trim)
            .filter(|l| !l.is_empty() && !l.starts_with('#'))
            .filter_map(|l| SdlMapping::parse(l).ok())
    };
    let tries: [&dyn Fn(&str) -> bool; 3] = [
        &|g| g == want,
        &|g| blank(g, false) == blank(&want, false),
        &|g| blank(g, true) == blank(&want, true),
    ];
    for matches in tries {
        if let Some(m) = lines().find(|m| matches(&m.guid.to_ascii_lowercase())) {
            return Some(SdlMapping { guid: want.clone(), ..m });
        }
    }
    None
}

/// The value for `SDL_GAMECONTROLLERCONFIG`: one mapping per line.
pub fn config_env<'a>(lines: impl IntoIterator<Item = &'a str>) -> String {
    lines.into_iter().map(str::trim).filter(|l| !l.is_empty()).collect::<Vec<_>>().join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use Control::*;

    const XBOX: &str = "030000005e0400008e02000014010000,Xbox 360 Controller,a:b0,b:b1,back:b6,dpdown:h0.4,dpleft:h0.8,dpright:h0.2,dpup:h0.1,guide:b8,leftshoulder:b4,leftstick:b9,lefttrigger:a2,leftx:a0,lefty:a1,rightshoulder:b5,rightstick:b10,righttrigger:a5,rightx:a3,righty:a4,start:b7,x:b2,y:b3,platform:Linux,";

    #[test]
    fn round_trips() {
        let m = SdlMapping::parse(XBOX).unwrap();
        assert_eq!(m.get("lefttrigger"), Some("a2"));
        assert_eq!(SdlMapping::parse(&m.to_line()).unwrap(), m);
    }

    #[test]
    fn swap_moves_the_inputs() {
        let m = SdlMapping::parse(XBOX).unwrap();
        let mut r = Remap::new();
        r.assign(South, East);
        r.assign(LeftTrigger, LeftBumper);
        let out = m.remapped(&r);
        assert_eq!(out.get("a"), Some("b1"));
        assert_eq!(out.get("b"), Some("b0"));
        assert_eq!(out.get("leftshoulder"), Some("a2"));
        assert_eq!(out.get("lefttrigger"), Some("b4"));
        // Axes and the platform are kept.
        assert_eq!(out.get("leftx"), Some("a0"));
        assert_eq!(out.get("platform"), Some("Linux"));
        assert_eq!(out.guid, m.guid);
    }

    #[test]
    fn identity_keeps_every_binding() {
        let m = SdlMapping::parse(XBOX).unwrap();
        let out = m.remapped(&Remap::new());
        for (k, v) in &m.fields {
            assert_eq!(out.get(k), Some(v.as_str()), "{k}");
        }
    }

    #[test]
    fn rejects_garbage() {
        assert_eq!(SdlMapping::parse("nope"), Err(SdlError::Malformed));
        assert_eq!(SdlMapping::parse("xyz,name,a:b0"), Err(SdlError::Malformed));
    }

    #[test]
    fn lookup_ignores_crc_then_version() {
        let db = format!("# comment
{XBOX}
");
        // Exact.
        assert!(lookup_in(&db, "030000005e0400008e02000014010000").is_some());
        // A name CRC in bytes 2-3, and the answer carries the pad's own GUID.
        let m = lookup_in(&db, "0300abcd5e0400008e02000014010000").unwrap();
        assert_eq!(m.guid, "0300abcd5e0400008e02000014010000");
        // Another firmware version.
        assert!(lookup_in(&db, "030000005e0400008e02000099990000").is_some());
        // Another product.
        assert!(lookup_in(&db, "030000005e040000ffff000014010000").is_none());
    }

    #[cfg(feature = "sdl-db")]
    #[test]
    fn the_bundled_db_knows_a_dualsense() {
        let m = lookup("030000004c050000e60c000011810000").unwrap();
        assert_eq!(m.get("platform"), Some("Linux"));
        assert!(m.get("a").is_some());
    }

    #[test]
    fn env_joins_lines() {
        assert_eq!(config_env(["a ", "", " b"]), "a\nb");
    }
}
