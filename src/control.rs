//! The controls of a gamepad, named by where they are, not what is printed on
//! them.
//!
//! The bottom face button is `South` on every pad: Xbox prints A on it,
//! PlayStation a cross, Nintendo B. Naming by place is what SDL does (its `a`
//! is "the bottom one"), and it is the only naming under which "swap A and B"
//! means the same thing on every controller. What a control is *called* on a
//! given pad is [`crate::Layout`]'s job.

/// One control: a button, a trigger, a stick (its press and its two axes are
/// one control here), or a pad of the d-pad.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Control {
    #[cfg_attr(feature = "serde", serde(rename = "a"))]
    South,
    #[cfg_attr(feature = "serde", serde(rename = "b"))]
    East,
    #[cfg_attr(feature = "serde", serde(rename = "x"))]
    West,
    #[cfg_attr(feature = "serde", serde(rename = "y"))]
    North,
    #[cfg_attr(feature = "serde", serde(rename = "leftshoulder"))]
    LeftBumper,
    #[cfg_attr(feature = "serde", serde(rename = "rightshoulder"))]
    RightBumper,
    #[cfg_attr(feature = "serde", serde(rename = "lefttrigger"))]
    LeftTrigger,
    #[cfg_attr(feature = "serde", serde(rename = "righttrigger"))]
    RightTrigger,
    #[cfg_attr(feature = "serde", serde(rename = "back"))]
    Select,
    #[cfg_attr(feature = "serde", serde(rename = "start"))]
    Start,
    #[cfg_attr(feature = "serde", serde(rename = "guide"))]
    Guide,
    #[cfg_attr(feature = "serde", serde(rename = "leftstick"))]
    LeftStick,
    #[cfg_attr(feature = "serde", serde(rename = "rightstick"))]
    RightStick,
    #[cfg_attr(feature = "serde", serde(rename = "dpup"))]
    DpadUp,
    #[cfg_attr(feature = "serde", serde(rename = "dpdown"))]
    DpadDown,
    #[cfg_attr(feature = "serde", serde(rename = "dpleft"))]
    DpadLeft,
    #[cfg_attr(feature = "serde", serde(rename = "dpright"))]
    DpadRight,
    /// The extra middle button: Share on an Xbox Series pad, the microphone
    /// button on a DualSense, Capture on a Switch Pro Controller.
    #[cfg_attr(feature = "serde", serde(rename = "misc1"))]
    Misc,
    /// A PlayStation touchpad's click.
    #[cfg_attr(feature = "serde", serde(rename = "touchpad"))]
    Touchpad,
    /// The back paddles of an Elite, an Edge or a Steam Deck: upper right,
    /// upper left, lower right, lower left (SDL's paddle1 to paddle4).
    #[cfg_attr(feature = "serde", serde(rename = "paddle1"))]
    PaddleUpperRight,
    #[cfg_attr(feature = "serde", serde(rename = "paddle2"))]
    PaddleUpperLeft,
    #[cfg_attr(feature = "serde", serde(rename = "paddle3"))]
    PaddleLowerRight,
    #[cfg_attr(feature = "serde", serde(rename = "paddle4"))]
    PaddleLowerLeft,
}

impl Control {
    /// Every control, in the order a settings screen lists them.
    pub const ALL: [Control; 23] = [
        Control::South,
        Control::East,
        Control::West,
        Control::North,
        Control::DpadUp,
        Control::DpadDown,
        Control::DpadLeft,
        Control::DpadRight,
        Control::LeftBumper,
        Control::RightBumper,
        Control::LeftTrigger,
        Control::RightTrigger,
        Control::LeftStick,
        Control::RightStick,
        Control::Select,
        Control::Start,
        Control::Guide,
        Control::Misc,
        Control::Touchpad,
        Control::PaddleUpperRight,
        Control::PaddleUpperLeft,
        Control::PaddleLowerRight,
        Control::PaddleLowerLeft,
    ];

    /// The name SDL's controller mappings use for this control. For the
    /// sticks that is the press (`leftstick`); their axes are `leftx`/`lefty`.
    pub const fn sdl_name(self) -> &'static str {
        match self {
            Control::South => "a",
            Control::East => "b",
            Control::West => "x",
            Control::North => "y",
            Control::LeftBumper => "leftshoulder",
            Control::RightBumper => "rightshoulder",
            Control::LeftTrigger => "lefttrigger",
            Control::RightTrigger => "righttrigger",
            Control::Select => "back",
            Control::Start => "start",
            Control::Guide => "guide",
            Control::LeftStick => "leftstick",
            Control::RightStick => "rightstick",
            Control::DpadUp => "dpup",
            Control::DpadDown => "dpdown",
            Control::DpadLeft => "dpleft",
            Control::DpadRight => "dpright",
            Control::Misc => "misc1",
            Control::Touchpad => "touchpad",
            Control::PaddleUpperRight => "paddle1",
            Control::PaddleUpperLeft => "paddle2",
            Control::PaddleLowerRight => "paddle3",
            Control::PaddleLowerLeft => "paddle4",
        }
    }

    pub fn from_sdl_name(name: &str) -> Option<Control> {
        Control::ALL.into_iter().find(|c| c.sdl_name() == name)
    }

    /// Whether a game may be given this control in place of another. The
    /// Guide button is the system's (it opens Steam, Game Bar or the
    /// launcher), so it is never moved, and nothing is moved onto it.
    pub const fn remappable(self) -> bool {
        !matches!(self, Control::Guide)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sdl_names_round_trip_and_are_unique() {
        for c in Control::ALL {
            assert_eq!(Control::from_sdl_name(c.sdl_name()), Some(c));
        }
        let mut names: Vec<_> = Control::ALL.iter().map(|c| c.sdl_name()).collect();
        names.sort();
        names.dedup();
        assert_eq!(names.len(), Control::ALL.len());
    }
}
