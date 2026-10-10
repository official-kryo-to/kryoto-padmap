//! Which controller is this?
//!
//! A pad tells the system three things about itself: a USB vendor id, a
//! product id and a name. The ids are exact when they are there; the name is
//! what is left when they are not (XInput reports no ids at all, and Bluetooth
//! stacks and third-party pads are loose with them). [`identify`] tries the
//! ids first, then the name, then the maker, and otherwise says "a gamepad".
//!
//! The name a system gives an unknown pad is often no name at all: Windows
//! calls a DirectInput pad "HID-compliant game controller" in the user's own
//! language, or describes it ("6 axis 16 button gamepad with hat switch").
//! [`is_generic_name`] tells those apart, so a screen can show the maker's
//! name instead.

/// The four kinds of face-button printing there are, which is what decides
/// what a button is called and what the pad looks like on screen.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum Family {
    /// A B X Y, A at the bottom. Xbox pads, and most PC pads and handhelds
    /// (Steam Deck, Logitech, 8BitDo in X mode) copy them.
    Xbox,
    /// Cross, Circle, Square, Triangle.
    PlayStation,
    /// A B X Y with A on the right and B at the bottom.
    Nintendo,
    /// Anything else: named by position.
    Generic,
}

impl Family {
    pub const ALL: [Family; 4] = [Family::Xbox, Family::PlayStation, Family::Nintendo, Family::Generic];

    pub const fn id(self) -> &'static str {
        match self {
            Family::Xbox => "xbox",
            Family::PlayStation => "playstation",
            Family::Nintendo => "nintendo",
            Family::Generic => "generic",
        }
    }

    pub fn from_id(id: &str) -> Option<Family> {
        Family::ALL.into_iter().find(|f| f.id() == id)
    }
}

/// What a pad has beyond the standard layout.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Features {
    pub touchpad: bool,
    pub gyro: bool,
    /// Back paddles (and how many).
    pub paddles: u8,
    /// The extra middle button (Share, Mic, Capture).
    pub misc: bool,
    /// Analog triggers rather than buttons.
    pub analog_triggers: bool,
}

/// A controller, as precisely as it could be told.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Model {
    pub family: Family,
    /// "DualSense", "Xbox Series X|S Controller", "Gamepad".
    pub name: String,
    pub features: Features,
    /// How it was told: from the ids, from the name, or not at all.
    pub matched: Matched,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum Matched {
    Ids,
    Name,
    /// An unknown product from a known maker: named after the maker.
    Maker,
    Fallback,
}

pub mod vendor {
    pub const MICROSOFT: u16 = 0x045e;
    pub const SONY: u16 = 0x054c;
    pub const NINTENDO: u16 = 0x057e;
    pub const VALVE: u16 = 0x28de;
    pub const LOGITECH: u16 = 0x046d;
    pub const EIGHTBITDO: u16 = 0x2dc8;
    pub const STEELSERIES: u16 = 0x1038;
    /// SteelSeries' other vendor id, on its Bluetooth pads.
    pub const STEELSERIES_BT: u16 = 0x0111;
}

const XBOX_360: Features = Features { touchpad: false, gyro: false, paddles: 0, misc: false, analog_triggers: true };
const XBOX_ONE: Features = XBOX_360;
const XBOX_ELITE: Features = Features { paddles: 4, ..XBOX_360 };
const XBOX_SERIES: Features = Features { misc: true, ..XBOX_360 };
const DS3: Features = Features { touchpad: false, gyro: true, paddles: 0, misc: false, analog_triggers: true };
const DS4: Features = Features { touchpad: true, gyro: true, paddles: 0, misc: false, analog_triggers: true };
const DUALSENSE: Features = Features { misc: true, ..DS4 };
const DUALSENSE_EDGE: Features = Features { paddles: 2, ..DUALSENSE };
const SWITCH_PRO: Features = Features { touchpad: false, gyro: true, paddles: 0, misc: true, analog_triggers: false };
const JOYCON: Features = Features { touchpad: false, gyro: true, paddles: 0, misc: true, analog_triggers: false };
const STEAM_DECK: Features = Features { touchpad: true, gyro: true, paddles: 4, misc: true, analog_triggers: true };
const STEAM_CONTROLLER: Features = Features { touchpad: true, gyro: true, paddles: 2, misc: false, analog_triggers: true };
const PLAIN: Features = Features { touchpad: false, gyro: false, paddles: 0, misc: false, analog_triggers: true };

/// Known pads by (vendor, product). Product ids from the vendors' own
/// devices as Linux's hid drivers and SDL list them.
const KNOWN: &[(u16, u16, Family, &str, Features)] = &[
    // Microsoft
    (vendor::MICROSOFT, 0x028e, Family::Xbox, "Xbox 360 Controller", XBOX_360),
    (vendor::MICROSOFT, 0x028f, Family::Xbox, "Xbox 360 Wireless Controller", XBOX_360),
    (vendor::MICROSOFT, 0x0719, Family::Xbox, "Xbox 360 Wireless Controller", XBOX_360),
    (vendor::MICROSOFT, 0x02d1, Family::Xbox, "Xbox One Controller", XBOX_ONE),
    (vendor::MICROSOFT, 0x02dd, Family::Xbox, "Xbox One Controller", XBOX_ONE),
    (vendor::MICROSOFT, 0x02ea, Family::Xbox, "Xbox One S Controller", XBOX_ONE),
    (vendor::MICROSOFT, 0x02e0, Family::Xbox, "Xbox One S Controller", XBOX_ONE),
    (vendor::MICROSOFT, 0x02fd, Family::Xbox, "Xbox One S Controller", XBOX_ONE),
    (vendor::MICROSOFT, 0x0b20, Family::Xbox, "Xbox One S Controller", XBOX_ONE),
    (vendor::MICROSOFT, 0x02e3, Family::Xbox, "Xbox Elite Controller", XBOX_ELITE),
    (vendor::MICROSOFT, 0x0b00, Family::Xbox, "Xbox Elite Series 2", XBOX_ELITE),
    (vendor::MICROSOFT, 0x0b05, Family::Xbox, "Xbox Elite Series 2", XBOX_ELITE),
    (vendor::MICROSOFT, 0x0b22, Family::Xbox, "Xbox Elite Series 2", XBOX_ELITE),
    (vendor::MICROSOFT, 0x0b12, Family::Xbox, "Xbox Series X|S Controller", XBOX_SERIES),
    (vendor::MICROSOFT, 0x0b13, Family::Xbox, "Xbox Series X|S Controller", XBOX_SERIES),
    (vendor::MICROSOFT, 0x0b0a, Family::Xbox, "Xbox Adaptive Controller", XBOX_ONE),
    (vendor::MICROSOFT, 0x0b21, Family::Xbox, "Xbox Adaptive Controller", XBOX_ONE),
    // Sony
    (vendor::SONY, 0x0268, Family::PlayStation, "DualShock 3", DS3),
    (vendor::SONY, 0x05c4, Family::PlayStation, "DualShock 4", DS4),
    (vendor::SONY, 0x09cc, Family::PlayStation, "DualShock 4", DS4),
    (vendor::SONY, 0x0ba0, Family::PlayStation, "DualShock 4", DS4),
    (vendor::SONY, 0x0ce6, Family::PlayStation, "DualSense", DUALSENSE),
    (vendor::SONY, 0x0df2, Family::PlayStation, "DualSense Edge", DUALSENSE_EDGE),
    // Nintendo
    (vendor::NINTENDO, 0x2009, Family::Nintendo, "Switch Pro Controller", SWITCH_PRO),
    (vendor::NINTENDO, 0x2006, Family::Nintendo, "Joy-Con (L)", JOYCON),
    (vendor::NINTENDO, 0x2007, Family::Nintendo, "Joy-Con (R)", JOYCON),
    (vendor::NINTENDO, 0x200e, Family::Nintendo, "Joy-Con pair", JOYCON),
    (vendor::NINTENDO, 0x2017, Family::Nintendo, "SNES Controller", PLAIN),
    // Valve
    (vendor::VALVE, 0x1205, Family::Xbox, "Steam Deck", STEAM_DECK),
    (vendor::VALVE, 0x1102, Family::Xbox, "Steam Controller", STEAM_CONTROLLER),
    (vendor::VALVE, 0x1142, Family::Xbox, "Steam Controller", STEAM_CONTROLLER),
    // SteelSeries (A B X Y, Xbox style)
    (vendor::STEELSERIES, 0x1412, Family::Xbox, "SteelSeries Free", PLAIN),
    (vendor::STEELSERIES, 0x1418, Family::Xbox, "SteelSeries Stratus XL", PLAIN),
    (vendor::STEELSERIES, 0x1420, Family::Xbox, "SteelSeries Nimbus", PLAIN),
    (vendor::STEELSERIES, 0x1430, Family::Xbox, "SteelSeries Stratus Duo", PLAIN),
    (vendor::STEELSERIES, 0x1431, Family::Xbox, "SteelSeries Stratus Duo", PLAIN),
    (vendor::STEELSERIES_BT, 0x1417, Family::Xbox, "SteelSeries Stratus XL", PLAIN),
    (vendor::STEELSERIES_BT, 0x1419, Family::Xbox, "SteelSeries Stratus", PLAIN),
    (vendor::STEELSERIES_BT, 0x1420, Family::Xbox, "SteelSeries Nimbus", PLAIN),
    (vendor::STEELSERIES_BT, 0x1431, Family::Xbox, "SteelSeries Stratus Duo", PLAIN),
    // Logitech in XInput mode
    (vendor::LOGITECH, 0xc21d, Family::Xbox, "Logitech F310", PLAIN),
    (vendor::LOGITECH, 0xc21e, Family::Xbox, "Logitech F510", PLAIN),
    (vendor::LOGITECH, 0xc21f, Family::Xbox, "Logitech F710", PLAIN),
];

/// Words in a pad's name that give its family away, most specific first.
const BY_NAME: &[(&str, Family, &str, Features)] = &[
    ("dualsense edge", Family::PlayStation, "DualSense Edge", DUALSENSE_EDGE),
    ("dualsense", Family::PlayStation, "DualSense", DUALSENSE),
    ("ps5", Family::PlayStation, "DualSense", DUALSENSE),
    ("dualshock 4", Family::PlayStation, "DualShock 4", DS4),
    ("dualshock4", Family::PlayStation, "DualShock 4", DS4),
    ("ps4", Family::PlayStation, "DualShock 4", DS4),
    ("dualshock", Family::PlayStation, "DualShock 3", DS3),
    ("ps3", Family::PlayStation, "DualShock 3", DS3),
    ("playstation", Family::PlayStation, "PlayStation controller", DS4),
    ("steam deck", Family::Xbox, "Steam Deck", STEAM_DECK),
    ("steam controller", Family::Xbox, "Steam Controller", STEAM_CONTROLLER),
    ("elite", Family::Xbox, "Xbox Elite Controller", XBOX_ELITE),
    ("xbox series", Family::Xbox, "Xbox Series X|S Controller", XBOX_SERIES),
    ("xbox one", Family::Xbox, "Xbox One Controller", XBOX_ONE),
    ("xbox 360", Family::Xbox, "Xbox 360 Controller", XBOX_360),
    ("xbox", Family::Xbox, "Xbox controller", XBOX_ONE),
    ("x-box", Family::Xbox, "Xbox controller", XBOX_ONE),
    ("xinput", Family::Xbox, "XInput controller", XBOX_360),
    ("joy-con", Family::Nintendo, "Joy-Con", JOYCON),
    ("joycon", Family::Nintendo, "Joy-Con", JOYCON),
    ("pro controller", Family::Nintendo, "Switch Pro Controller", SWITCH_PRO),
    ("switch", Family::Nintendo, "Switch controller", SWITCH_PRO),
    ("nintendo", Family::Nintendo, "Nintendo controller", SWITCH_PRO),
    ("steelseries", Family::Xbox, "SteelSeries controller", PLAIN),
    ("stratus", Family::Xbox, "SteelSeries Stratus", PLAIN),
    ("nimbus", Family::Xbox, "SteelSeries Nimbus", PLAIN),
];

/// Makers whose unknown pads still print their buttons one way, and what to
/// call such a pad.
const MAKERS: &[(u16, Family, &str)] = &[
    (vendor::MICROSOFT, Family::Xbox, "Xbox controller"),
    (vendor::VALVE, Family::Xbox, "Valve controller"),
    (vendor::LOGITECH, Family::Xbox, "Logitech controller"),
    (vendor::STEELSERIES, Family::Xbox, "SteelSeries controller"),
    (vendor::STEELSERIES_BT, Family::Xbox, "SteelSeries controller"),
    (vendor::SONY, Family::PlayStation, "PlayStation controller"),
    (vendor::NINTENDO, Family::Nintendo, "Nintendo controller"),
    (vendor::EIGHTBITDO, Family::Generic, "8BitDo controller"),
];

/// A name a system gives a pad it does not know, rather than the pad's own:
/// Windows' "HID-compliant game controller" (in any language: "HID-
/// spelbesturing", "HID-konformer Gamecontroller"), DirectInput's "6 axis 16
/// button gamepad with hat switch" (or "As: 6 knop: 16 gamepad met
/// kapschakelaar"), "USB Joystick", "Generic X-Box pad".
pub fn is_generic_name(name: &str) -> bool {
    let n = name.trim().to_lowercase();
    if n.is_empty() {
        return true;
    }
    const WORDS: &[&str] = &[
        "hid", "generic", "usb joystick", "usb gamepad", "game controller", "gamecontroller",
        "joystick", "gamepad with", "gamepad met", "manette", "spelbesturing", "mando de juego",
        "controller (", "twin usb", "dragonrise", " axis ", " button ", "knop:",
    ];
    WORDS.iter().any(|w| n.contains(w))
}

/// Tell a pad from what it reports. `vendor` and `product` are `None` when
/// the system did not say (XInput never does).
pub fn identify(vendor: Option<u16>, product: Option<u16>, name: &str) -> Model {
    if let (Some(v), Some(p)) = (vendor, product) {
        if let Some((_, _, family, model, features)) = KNOWN.iter().find(|k| k.0 == v && k.1 == p) {
            return Model { family: *family, name: (*model).into(), features: *features, matched: Matched::Ids };
        }
    }
    let lower = name.to_lowercase();
    if let Some((_, family, model, features)) = BY_NAME.iter().find(|n| lower.contains(n.0)) {
        return Model { family: *family, name: (*model).into(), features: *features, matched: Matched::Name };
    }
    // An unknown product from a known maker still prints that maker's buttons,
    // and a name of its own beats the system's placeholder.
    if let Some((_, family, maker)) = vendor.and_then(|v| MAKERS.iter().find(|m| m.0 == v)) {
        let shown = if is_generic_name(name) { (*maker).to_string() } else { name.trim().to_string() };
        return Model { family: *family, name: shown, features: PLAIN, matched: Matched::Maker };
    }
    let shown = if is_generic_name(name) { "Controller".to_string() } else { name.trim().to_string() };
    Model { family: Family::Generic, name: shown, features: PLAIN, matched: Matched::Fallback }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids_win_over_the_name() {
        // A DualSense over Bluetooth is called "Wireless Controller".
        let m = identify(Some(0x054c), Some(0x0ce6), "Wireless Controller");
        assert_eq!((m.family, m.name.as_str(), m.matched), (Family::PlayStation, "DualSense", Matched::Ids));
        assert!(m.features.touchpad && m.features.misc);
    }

    #[test]
    fn xinput_has_no_ids_and_is_told_by_name() {
        let m = identify(None, None, "Xbox 360 Controller (XInput STANDARD GAMEPAD)");
        assert_eq!((m.family, m.matched), (Family::Xbox, Matched::Name));
        assert_eq!(identify(None, None, "XInput Controller #1").family, Family::Xbox);
    }

    #[test]
    fn edge_before_dualsense_and_elite_before_xbox() {
        assert_eq!(identify(None, None, "Sony DualSense Edge Wireless Controller").name, "DualSense Edge");
        assert_eq!(identify(None, None, "Xbox Elite Wireless Controller Series 2").name, "Xbox Elite Controller");
    }

    #[test]
    fn nintendo_and_unknown_makers() {
        assert_eq!(identify(Some(0x057e), Some(0x2009), "Pro Controller").family, Family::Nintendo);
        // A new Sony product still prints Sony's buttons.
        assert_eq!(identify(Some(0x054c), Some(0x9999), "Mystery").family, Family::PlayStation);
        let m = identify(Some(0x1234), Some(0x5678), "  ");
        assert_eq!((m.family, m.name.as_str(), m.matched), (Family::Generic, "Controller", Matched::Fallback));
    }

    #[test]
    fn a_steelseries_pad_windows_does_not_know() {
        // What Windows (in Dutch) and SDL called a SteelSeries pad.
        for name in ["HID-spelbesturing", "As: 6 knop: 16 gamepad met kapschakelaar"] {
            let m = identify(Some(0x0111), Some(0x1434), name);
            assert_eq!((m.family, m.name.as_str(), m.matched), (Family::Xbox, "SteelSeries controller", Matched::Maker));
        }
        assert_eq!(identify(Some(0x0111), Some(0x1431), "x").name, "SteelSeries Stratus Duo");
    }

    #[test]
    fn generic_names() {
        for n in ["HID-compliant game controller", "HID-spelbesturing", "6 axis 16 button gamepad with hat switch", "USB Joystick", "Generic X-Box pad", ""] {
            assert!(is_generic_name(n), "{n}");
        }
        for n in ["Xbox Wireless Controller", "DualSense Wireless Controller", "SteelSeries Stratus Duo", "8BitDo Pro 2"] {
            assert!(!is_generic_name(n), "{n}");
        }
    }

    #[test]
    fn family_ids_round_trip() {
        for f in Family::ALL {
            assert_eq!(Family::from_id(f.id()), Some(f));
        }
    }
}
