use crate::lex::command::*;

use super::FlickDirection;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct Notes {
    pub bells: Vec<Bell>,
    pub flicks: Vec<Flick>,
    pub critical_flicks: Vec<Flick>,
    pub taps: Vec<Tap>,
    pub critical_taps: Vec<Tap>,
    pub holds: Vec<Hold>,
    pub critical_holds: Vec<Hold>,
    // pub bells: Vec<BellNote>,
    // pub flicks: Vec<FlickNote>,
    // pub taps: Vec<TapNote>,
    // pub critical_taps: Vec<TapNote>,
    // pub holds: Vec<HoldNote>,
    // pub critical_holds: Vec<HoldNote>,
}

// #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
// pub enum LanePosition {
//     Left,
//     Center,
//     Right,
// }
//
// #[derive(Clone, Debug, PartialEq, Eq, Hash)]
// pub struct BellNote {
//     pub point: TrackPoint,
//     pub bullet_palette_id: Option<String>,
// }
//
// #[derive(Clone, Debug, PartialEq, Eq, Hash)]
// pub struct FlickNote {
//     pub point: TrackPoint,
//     pub direction: FlickDirection,
// }
//
// #[derive(Clone, Debug, PartialEq, Eq, Hash)]
// pub struct TapNote {
//     pub point: TrackPoint,
//     pub lane_id: u32,
//     pub lane_position: LanePosition,
// }
//
// #[derive(Clone, Debug, PartialEq, Eq, Hash)]
// pub struct HoldNote {
//     pub lane_id: u32,
//     pub start_point: TrackPoint,
//     pub end_point: TrackPoint,
// }
