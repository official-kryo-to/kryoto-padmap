//! Remapping: which physical control acts as which.
//!
//! A [`Remap`] is always a permutation of the controls: giving a control a new
//! job hands its old job to the control that had the new one (a swap). So no
//! job is ever left without a control, and no two controls fight over one,
//! which is also the only kind of remap an SDL mapping can express.

use std::collections::BTreeMap;

use crate::Control;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct Remap {
    /// physical -> the control it acts as. Controls not in here act as
    /// themselves.
    acts_as: BTreeMap<Control, Control>,
}

impl Remap {
    pub fn new() -> Self {
        Self::default()
    }

    /// What `physical` does now.
    pub fn acts_as(&self, physical: Control) -> Control {
        self.acts_as.get(&physical).copied().unwrap_or(physical)
    }

    /// Which physical control does `job` now.
    pub fn done_by(&self, job: Control) -> Control {
        // A permutation: exactly one physical control maps to `job`.
        Control::ALL.into_iter().find(|&p| self.acts_as(p) == job).unwrap_or(job)
    }

    /// Make `physical` act as `job`; whatever did `job` takes over what
    /// `physical` did. Refused (false) for controls that are not remappable.
    pub fn assign(&mut self, physical: Control, job: Control) -> bool {
        if !physical.remappable() || !job.remappable() {
            return false;
        }
        let old_job = self.acts_as(physical);
        if old_job == job {
            return true;
        }
        let other = self.done_by(job);
        self.set(physical, job);
        self.set(other, old_job);
        true
    }

    /// Back to every control doing its own job.
    pub fn reset(&mut self, physical: Control) {
        let job = self.acts_as(physical);
        if job != physical {
            self.assign(physical, physical);
        }
    }

    pub fn is_identity(&self) -> bool {
        self.acts_as.is_empty()
    }

    /// The changed controls, as (physical, job).
    pub fn changes(&self) -> impl Iterator<Item = (Control, Control)> + '_ {
        self.acts_as.iter().map(|(p, j)| (*p, *j))
    }

    /// Rebuild from stored pairs, keeping only a valid permutation: pairs
    /// that would break it (a job given twice, an unremappable control) are
    /// dropped, applied in order as swaps.
    pub fn from_pairs(pairs: impl IntoIterator<Item = (Control, Control)>) -> Self {
        let mut r = Remap::new();
        for (p, j) in pairs {
            r.assign(p, j);
        }
        r
    }

    fn set(&mut self, physical: Control, job: Control) {
        if physical == job {
            self.acts_as.remove(&physical);
        } else {
            self.acts_as.insert(physical, job);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Control::*;

    #[test]
    fn assign_swaps() {
        let mut r = Remap::new();
        assert!(r.assign(South, East));
        assert_eq!(r.acts_as(South), East);
        assert_eq!(r.acts_as(East), South);
        assert_eq!(r.done_by(East), South);
        // Chained: West now does East's job, South takes West's.
        r.assign(West, East);
        assert_eq!((r.acts_as(West), r.acts_as(South), r.acts_as(East)), (East, West, South));
        r.reset(West);
        assert_eq!(r.acts_as(West), West);
    }

    #[test]
    fn stays_a_permutation() {
        let r = Remap::from_pairs([(South, East), (East, East), (LeftTrigger, RightBumper), (Guide, South)]);
        let mut jobs: Vec<_> = Control::ALL.iter().map(|&c| r.acts_as(c)).collect();
        jobs.sort();
        let mut all = Control::ALL.to_vec();
        all.sort();
        assert_eq!(jobs, all);
        assert_eq!(r.acts_as(Guide), Guide);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn stores_as_a_json_object_of_sdl_names() {
        let mut r = Remap::new();
        r.assign(South, East);
        let json = serde_json::to_string(&r).unwrap();
        assert_eq!(json, r#"{"a":"b","b":"a"}"#);
        let back: Remap = serde_json::from_str(&json).unwrap();
        assert_eq!(back, r);
    }

    #[test]
    fn swapping_back_is_identity() {
        let mut r = Remap::new();
        r.assign(South, North);
        r.assign(South, South);
        assert!(r.is_identity());
    }
}
