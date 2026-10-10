//! What each control is called on a given kind of pad.

use crate::{Control, Family};

/// A control's name on one family of pads.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Label {
    /// What is printed on it, or its usual short name: "A", "L1", "ZR", "View".
    pub short: &'static str,
    /// Its name in a sentence: "Cross button", "Left trigger".
    pub long: &'static str,
    /// A PlayStation face button's shape, for drawing it: `cross`, `circle`,
    /// `square` or `triangle`. `None` everywhere else.
    pub symbol: Option<&'static str>,
}

const fn l(short: &'static str, long: &'static str) -> Label {
    Label { short, long, symbol: None }
}

const fn sym(short: &'static str, long: &'static str, symbol: &'static str) -> Label {
    Label { short, long, symbol: Some(symbol) }
}

/// The names of a family's controls.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Layout(pub Family);

impl Layout {
    pub fn label(self, control: Control) -> Label {
        use Control::*;
        use Family::*;
        match (self.0, control) {
            // Face buttons: the one thing every family prints differently.
            (Xbox, South) => l("A", "A button"),
            (Xbox, East) => l("B", "B button"),
            (Xbox, West) => l("X", "X button"),
            (Xbox, North) => l("Y", "Y button"),
            (PlayStation, South) => sym("Cross", "Cross button", "cross"),
            (PlayStation, East) => sym("Circle", "Circle button", "circle"),
            (PlayStation, West) => sym("Square", "Square button", "square"),
            (PlayStation, North) => sym("Triangle", "Triangle button", "triangle"),
            (Nintendo, South) => l("B", "B button"),
            (Nintendo, East) => l("A", "A button"),
            (Nintendo, West) => l("Y", "Y button"),
            (Nintendo, North) => l("X", "X button"),
            (Generic, South) => l("S", "Bottom face button"),
            (Generic, East) => l("E", "Right face button"),
            (Generic, West) => l("W", "Left face button"),
            (Generic, North) => l("N", "Top face button"),

            (PlayStation, LeftBumper) => l("L1", "L1"),
            (PlayStation, RightBumper) => l("R1", "R1"),
            (PlayStation, LeftTrigger) => l("L2", "L2"),
            (PlayStation, RightTrigger) => l("R2", "R2"),
            (PlayStation, LeftStick) => l("L3", "Left stick press"),
            (PlayStation, RightStick) => l("R3", "Right stick press"),
            (PlayStation, Select) => l("Create", "Create / Share button"),
            (PlayStation, Start) => l("Options", "Options button"),
            (PlayStation, Guide) => l("PS", "PS button"),
            (PlayStation, Misc) => l("Mic", "Mute button"),

            (Nintendo, LeftBumper) => l("L", "L button"),
            (Nintendo, RightBumper) => l("R", "R button"),
            (Nintendo, LeftTrigger) => l("ZL", "ZL button"),
            (Nintendo, RightTrigger) => l("ZR", "ZR button"),
            (Nintendo, Select) => l("-", "Minus button"),
            (Nintendo, Start) => l("+", "Plus button"),
            (Nintendo, Guide) => l("Home", "Home button"),
            (Nintendo, Misc) => l("Capture", "Capture button"),

            (Xbox, Select) => l("View", "View button"),
            (Xbox, Start) => l("Menu", "Menu button"),
            (Xbox, Guide) => l("Xbox", "Xbox button"),
            (Xbox, Misc) => l("Share", "Share button"),

            (Generic, Select) => l("Select", "Select button"),
            (Generic, Start) => l("Start", "Start button"),
            (Generic, Guide) => l("Home", "Home button"),
            (Generic, Misc) => l("Misc", "Extra button"),

            // Xbox, Nintendo and Generic share the rest.
            (_, LeftBumper) => l("LB", "Left bumper"),
            (_, RightBumper) => l("RB", "Right bumper"),
            (_, LeftTrigger) => l("LT", "Left trigger"),
            (_, RightTrigger) => l("RT", "Right trigger"),
            (_, LeftStick) => l("LS", "Left stick press"),
            (_, RightStick) => l("RS", "Right stick press"),
            (_, DpadUp) => l("Up", "D-pad up"),
            (_, DpadDown) => l("Down", "D-pad down"),
            (_, DpadLeft) => l("Left", "D-pad left"),
            (_, DpadRight) => l("Right", "D-pad right"),
            (_, Touchpad) => l("Touchpad", "Touchpad click"),
            (_, PaddleUpperRight) => l("P1", "Upper right paddle"),
            (_, PaddleUpperLeft) => l("P2", "Upper left paddle"),
            (_, PaddleLowerRight) => l("P3", "Lower right paddle"),
            (_, PaddleLowerLeft) => l("P4", "Lower left paddle"),
        }
    }

    /// The button that confirms in menus. South everywhere except on
    /// Nintendo pads, where A (East) is "yes" and B (South) is "back".
    pub fn confirm(self) -> Control {
        if self.0 == Family::Nintendo { Control::East } else { Control::South }
    }

    pub fn back(self) -> Control {
        if self.0 == Family::Nintendo { Control::South } else { Control::East }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_control_has_a_name_on_every_family() {
        for f in Family::ALL {
            for c in Control::ALL {
                let label = Layout(f).label(c);
                assert!(!label.short.is_empty() && !label.long.is_empty(), "{f:?} {c:?}");
            }
        }
    }

    #[test]
    fn same_place_different_names() {
        assert_eq!(Layout(Family::Xbox).label(Control::South).short, "A");
        assert_eq!(Layout(Family::Nintendo).label(Control::South).short, "B");
        assert_eq!(Layout(Family::PlayStation).label(Control::South).symbol, Some("cross"));
        assert_eq!(Layout(Family::Nintendo).confirm(), Control::East);
    }
}
