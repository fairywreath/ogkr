use std::collections::{BTreeMap, HashMap};

use super::{
    raw::{
        BeamSection, ColorfulLaneSection, LaneSection, ObliqueBeamSection, RawComposition,
        RawNotes, RawOgkr, RawTrack, WallSection,
    },
    BulletDamageType, BulletShooter, BulletSize, BulletTarget, BulletType, EnemyWaveAssignment,
    FlickDirection, Header, LanePoint, ParseError, Result, WallPoint,
};

use crate::lex::command;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TimingPoint {
    /// Measure or bar the object is in.
    pub measure: u32,
    /// Offset from the start of the measure. Maximum value inside a measure is specified by [`TickResolution`].
    pub beat_offset: u32,
}

impl TimingPoint {
    pub fn new(measure: u32, beat_offset: u32) -> Self {
        Self {
            measure,
            beat_offset,
        }
    }
}

impl PartialOrd for TimingPoint {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TimingPoint {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.measure
            .cmp(&other.measure)
            .then(self.beat_offset.cmp(&other.beat_offset))
    }
}

impl From<command::CommandTime> for TimingPoint {
    fn from(command_time: command::CommandTime) -> Self {
        Self {
            measure: command_time.measure,
            beat_offset: command_time.offset,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct XPosition {
    pub position: i32,
    /// Uses [`XResolution`] as the width reference value.
    pub offset: i32,
}

impl XPosition {
    pub fn new(position: i32, offset: i32) -> Self {
        Self { position, offset }
    }

    pub fn new_position(position: i32) -> Self {
        Self::new(position, 0)
    }
}

impl PartialOrd for XPosition {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for XPosition {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.position
            .cmp(&other.position)
            .then(self.offset.cmp(&other.position))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TrackPosition {
    pub time: TimingPoint,
    pub x: XPosition,
}

impl TrackPosition {
    pub fn new(time: TimingPoint, x: XPosition) -> Self {
        Self { time, x }
    }

    pub fn from_command_info(time: command::CommandTime, x_position: i32, x_offset: i32) -> Self {
        Self {
            time: time.into(),
            x: XPosition::new(x_position, x_offset),
        }
    }

    pub fn from_wall_point(wall_point: WallPoint) -> Self {
        Self {
            time: wall_point.time.into(),
            x: XPosition::new_position(wall_point.x_position),
        }
    }

    pub fn from_lane_point(lane_point: LanePoint) -> Self {
        Self {
            time: lane_point.time.into(),
            x: XPosition::new_position(lane_point.x_position),
        }
    }

    pub fn from_command_colorful_lane_point(lane_point: command::ColorfulLanePoint) -> Self {
        Self {
            time: lane_point.time.into(),
            x: XPosition::new_position(lane_point.x_position),
        }
    }
}

impl PartialOrd for TrackPosition {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TrackPosition {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.time.cmp(&other.time)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BulletPaletteId(pub String);

#[derive(Clone, Debug)]
pub struct BulletPalette {
    pub id: BulletPaletteId,
    pub shooter: BulletShooter,
    pub target: BulletTarget,
    pub x_offset: i32,
    pub speed: f32,

    pub size: Option<BulletSize>,
    pub bullet_type: Option<BulletType>,
    pub random_position_offset: Option<i32>,

    pub damage_type: Option<BulletDamageType>,
}

impl From<command::BulletPalette> for BulletPalette {
    fn from(palette: command::BulletPalette) -> Self {
        Self {
            id: BulletPaletteId(palette.id),
            shooter: palette.shooter,
            target: palette.target,
            x_offset: palette.target_x_offset,
            speed: f32::from_bits(palette.speed),

            size: palette.size,
            bullet_type: palette.ty,
            random_position_offset: palette.random_position_offset,

            damage_type: palette.damage_type,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LaneType {
    WallLeft,
    WallRight,
    Left,
    Center,
    Right,
    /// Not a real "lane" for the player.
    Enemy,
}

/// Represents both walls and lanes - they have unique IDs.
/// Notes distinguish between wall and lane placement based on this id.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LaneId(pub u32);

#[derive(Clone, Debug)]
pub struct Lane {
    pub id: LaneId,
    pub lane_type: LaneType,

    /// Sorted by time.
    pub points: Vec<TrackPosition>,
}

impl Lane {
    // pub fn get_points_within_time_interval(
    //     &self,
    //     start: TimingPoint,
    //     end: TimingPoint,
    // ) -> Result<&[TrackPosition]> {
    //     let start_index = self.points.binary_search_by(|point| point.time.cmp(&start));
    //     let end_index = self.points.binary_search_by(|point| point.time.cmp(&end));
    //
    //     if let (Ok(start_index), Ok(end_index)) = (start_index, end_index) {
    //         // +1 on end index to make inclusive.
    //         Ok(&self.points[start_index..end_index + 1])
    //     } else {
    //         Err(ParseError::SemanticError(format!(
    //             "Lane {:#?} has invalid time range {:#?} {:#?}",
    //             self.id, start, end
    //         )))
    //     }
    // }

    /// Start and end may not explicitly exist within `points`. In this case we append them to the
    /// existing interval within `points`.
    pub fn create_points_within_time_interval(
        &self,
        start: TrackPosition,
        end: TrackPosition,
    ) -> Result<Vec<TrackPosition>> {
        // let start_index = self
        //     .points
        //     .binary_search_by(|point| point.time.cmp(&start.time))
        //     .unwrap_or_else(|idx| idx);
        //
        // let end_index = self
        //     .points
        //     .binary_search_by(|point| point.time.cmp(&end.time))
        //     .unwrap_or_else(|idx| if idx > 0 { idx - 1 } else { idx });

        let (start_index, start_exact) = match self
            .points
            .binary_search_by(|point| point.time.cmp(&start.time))
        {
            Ok(idx) => (idx, true),
            Err(idx) => (idx, false),
        };

        let (end_index, end_exact) = match self
            .points
            .binary_search_by(|point| point.time.cmp(&end.time))
        {
            Ok(idx) => (idx, true),
            Err(idx) => (if idx > 0 { idx - 1 } else { idx }, false),
        };

        if start_index <= end_index && end_index < self.points.len() {
            let mut result = vec![];
            if !start_exact {
                result.push(start);
            }
            result.extend_from_slice(&self.points[start_index..=end_index]);
            if !end_exact {
                result.push(end);
            }
            // println!("{:#?}", &result);
            assert_ne!(result[0], result[1]);
            assert_ne!(result[result.len() - 2], result[result.len() - 1]);
            Ok(result)
        } else {
            // XXX: We assume that start and end is always valid, but this may not always be the
            // case.
            Ok(vec![start, end])
        }
        // else {
        //     println!("lane: {:#?}", self.points);
        //     println!("start idx {}, end idx {}", start_index, end_index);
        //     Err(ParseError::SemanticError(format!(
        //         "Lane {:#?} has invalid time range {:#?} {:#?}",
        //         self.id, start, end
        //     )))
        // }
    }

    pub fn from_wall_section(wall_section: WallSection, lane_type: LaneType) -> Result<Self> {
        if wall_section.points.len() >= 2 {
            Ok(Self {
                id: LaneId(wall_section.group_id),
                lane_type,
                points: wall_section
                    .points
                    .into_iter()
                    .map(|p| TrackPosition::from_wall_point(p))
                    .collect(),
            })
        } else {
            Err(ParseError::SemanticError(format!(
                "Lane section requires at least 2 points, id {}",
                wall_section.group_id
            )))
        }
    }

    pub fn from_lane_section(lane_section: LaneSection, lane_type: LaneType) -> Result<Self> {
        if lane_section.points.len() >= 2 {
            Ok(Self {
                id: LaneId(lane_section.group_id),
                lane_type,
                points: lane_section
                    .points
                    .into_iter()
                    .map(|p| TrackPosition::from_lane_point(p))
                    .collect(),
            })
        } else {
            Err(ParseError::SemanticError(format!(
                "Lane section requires at least 2 points, id {}",
                lane_section.group_id
            )))
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ColorfulLaneId(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ColorfulLaneColor(pub u32);

#[derive(Clone, Debug)]
pub struct ColorfulLanePoint {
    pub position: TrackPosition,
    pub color: ColorfulLaneColor,
    pub brightness: u32,
}

impl From<command::ColorfulLanePoint> for ColorfulLanePoint {
    fn from(point: command::ColorfulLanePoint) -> Self {
        Self {
            position: TrackPosition::from_command_colorful_lane_point(point),
            color: ColorfulLaneColor(point.color),
            brightness: point.brightness,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ColorfulLane {
    pub id: ColorfulLaneId,
    pub start: ColorfulLanePoint,
    pub middle: Vec<ColorfulLanePoint>,
    pub end: ColorfulLanePoint,
}

impl ColorfulLane {
    pub fn from_section(lane_section: ColorfulLaneSection) -> Result<Self> {
        if let (Some(start), Some(end)) = (lane_section.points.first(), lane_section.points.last())
        {
            Ok(Self {
                id: ColorfulLaneId(lane_section.group_id),
                start: (*start).into(),
                middle: lane_section.points[1..lane_section.points.len() - 1]
                    .into_iter()
                    .map(|p| (*p).into())
                    .collect(),
                end: (*end).into(),
            })
        } else {
            Err(ParseError::SemanticError(format!(
                "Colorful lane requires at least 2 points, id {}",
                lane_section.group_id
            )))
        }
    }
}

#[derive(Clone, Debug)]
pub struct LaneDisappearance {
    pub lane_id: LaneId,
    pub start: TrackPosition,
    pub end: TrackPosition,
}

impl From<command::LaneEvent> for LaneDisappearance {
    fn from(event: command::LaneEvent) -> Self {
        Self {
            lane_id: LaneId(event.group_id),
            start: TrackPosition::from_command_info(
                event.start_time,
                event.start_x_position,
                event.start_x_offset,
            ),
            end: TrackPosition::from_command_info(
                event.end_time,
                event.end_x_position,
                event.end_x_offset,
            ),
        }
    }
}

#[derive(Clone, Debug)]
pub struct LaneBlock {
    pub lane_id: LaneId,
    pub start: TrackPosition,
    pub end: TrackPosition,
}

impl From<command::LaneEvent> for LaneBlock {
    fn from(event: command::LaneEvent) -> Self {
        Self {
            lane_id: LaneId(event.group_id),
            start: TrackPosition::from_command_info(
                event.start_time,
                event.start_x_position,
                event.start_x_offset,
            ),
            end: TrackPosition::from_command_info(
                event.end_time,
                event.end_x_position,
                event.end_x_offset,
            ),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Bullet {
    pub palette_id: BulletPaletteId,

    /// End position of the bullet if the player is not the target.
    pub position: TrackPosition,

    pub damage_type: BulletDamageType,
}

impl From<command::Bullet> for Bullet {
    fn from(bullet: command::Bullet) -> Self {
        Self {
            palette_id: BulletPaletteId(bullet.pallete_id),
            position: TrackPosition::from_command_info(bullet.time, bullet.x_position, 0),
            damage_type: bullet.damage_type,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BeamId(pub u32);

#[derive(Clone, Debug)]
pub struct BeamPoint {
    pub position: TrackPosition,
    pub width: u32,
}

impl From<command::BeamPoint> for BeamPoint {
    fn from(point: command::BeamPoint) -> Self {
        Self {
            position: TrackPosition::from_command_info(point.time, point.x_position, 0),
            width: point.width,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Beam {
    pub id: BeamId,
    pub start: BeamPoint,
    pub middle: Vec<BeamPoint>,
    pub end: BeamPoint,
}

impl Beam {
    pub fn from_section(section: BeamSection) -> Result<Self> {
        if let (Some(start), Some(end)) = (section.points.first(), section.points.last()) {
            Ok(Self {
                id: BeamId(section.record_id),
                start: (*start).into(),
                middle: section.points[1..section.points.len() - 1]
                    .into_iter()
                    .map(|p| (*p).into())
                    .collect(),
                end: (*end).into(),
            })
        } else {
            Err(ParseError::SemanticError(format!(
                "Beam section requires at least 2 points, id {}",
                section.record_id
            )))
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ObliqueBeamId(pub u32);

#[derive(Clone, Debug)]
pub struct ObliqueBeamPoint {
    pub position: TrackPosition,
    pub width: u32,
    pub shoot_x_offset: i32,
}

impl From<command::ObliqueBeamPoint> for ObliqueBeamPoint {
    fn from(point: command::ObliqueBeamPoint) -> Self {
        Self {
            position: TrackPosition::from_command_info(point.time, point.x_position, 0),
            width: point.width,
            shoot_x_offset: point.shoot_position_x_offset,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ObliqueBeam {
    pub id: ObliqueBeamId,
    pub start: ObliqueBeamPoint,
    pub middle: Vec<ObliqueBeamPoint>,
    pub end: ObliqueBeamPoint,
}

impl ObliqueBeam {
    pub fn from_section(section: ObliqueBeamSection) -> Result<Self> {
        if let (Some(start), Some(end)) = (section.points.first(), section.points.last()) {
            Ok(Self {
                id: ObliqueBeamId(section.record_id),
                start: (*start).into(),
                middle: section.points[1..section.points.len() - 1]
                    .into_iter()
                    .map(|p| (*p).into())
                    .collect(),
                end: (*end).into(),
            })
        } else {
            Err(ParseError::SemanticError(format!(
                "Oblique beam section requires at least 2 points, id {}",
                section.record_id
            )))
        }
    }
}

#[derive(Clone, Debug)]
pub struct BellNote {
    pub position: TrackPosition,
    pub bullet_palette: Option<BulletPaletteId>,
}

impl From<command::Bell> for BellNote {
    fn from(bell: command::Bell) -> Self {
        Self {
            position: TrackPosition::from_command_info(bell.time, bell.x_position, 0),
            bullet_palette: bell.bullet_palette_id.map(|b| BulletPaletteId(b)),
        }
    }
}

#[derive(Clone, Debug)]
pub struct FlickNote {
    pub position: TrackPosition,
    pub direction: FlickDirection,
    pub is_critical: bool,
}

impl FlickNote {
    pub fn from_flick(flick: command::Flick, is_critical: bool) -> Self {
        Self {
            position: TrackPosition::from_command_info(flick.time, flick.x_position, 0),
            direction: flick.direction,
            is_critical,
        }
    }
}

#[derive(Clone, Debug)]
pub struct TapNote {
    pub lane_id: LaneId,
    pub lane_type: LaneType,

    pub position: TrackPosition,

    pub is_critical: bool,
}

impl TapNote {
    pub fn from_tap(tap: command::Tap, lane_type: LaneType, is_critical: bool) -> Self {
        Self {
            lane_id: LaneId(tap.lane_group_id),
            lane_type,
            position: TrackPosition::from_command_info(tap.time, tap.x_position, tap.x_offset),
            is_critical,
        }
    }
}

/// XXX TODO: Fill in points/positions for hold note based on lane.
#[derive(Clone, Debug)]
pub struct HoldNote {
    pub lane_id: LaneId,
    pub lane_type: LaneType,

    pub start: TrackPosition,
    pub end: TrackPosition,

    /// Includes start and end.
    pub points: Vec<TrackPosition>,

    pub is_critical: bool,
}

impl HoldNote {
    pub fn from_hold_and_lane(hold: command::Hold, lane: &Lane, is_critical: bool) -> Result<Self> {
        // XXX TODO: Verify start and end in hold has similar x positions as in lane.
        let start = TrackPosition::from_command_info(
            hold.start_time,
            hold.start_x_position,
            hold.start_x_offset,
        );
        let end =
            TrackPosition::from_command_info(hold.end_time, hold.end_x_position, hold.end_x_offset);

        Ok(Self {
            lane_id: LaneId(hold.lane_group_id),
            lane_type: lane.lane_type,
            start,
            end,
            points: lane.create_points_within_time_interval(start, end)?,
            is_critical,
        })
    }
}

#[derive(Clone, Debug)]
pub struct Track {
    // XXX: Maybe this is not the best representation for lanes.
    pub lanes_left: BTreeMap<TimingPoint, Vec<LaneId>>,
    pub lanes_center: BTreeMap<TimingPoint, Vec<LaneId>>,
    pub lanes_right: BTreeMap<TimingPoint, Vec<LaneId>>,

    pub colorful_lanes: BTreeMap<TimingPoint, ColorfulLaneId>,

    pub walls_left: BTreeMap<TimingPoint, LaneId>,
    pub walls_right: BTreeMap<TimingPoint, LaneId>,

    pub enemy_lanes: BTreeMap<TimingPoint, Vec<LaneId>>,

    // pub lanes_all: BTreeMap<TimingPoint, LaneId>,
    pub beams: BTreeMap<TimingPoint, BeamId>,
    pub oblique_beams: BTreeMap<TimingPoint, ObliqueBeamId>,

    pub lanes_data: HashMap<LaneId, Lane>,
    pub colorful_lanes_data: HashMap<ColorfulLaneId, ColorfulLane>,
    pub beams_data: HashMap<BeamId, Beam>,
    pub oblique_beams_data: HashMap<ObliqueBeamId, ObliqueBeam>,
}

impl Track {
    pub fn get_lane(&self, id: LaneId) -> Option<&Lane> {
        self.lanes_data.get(&id)
    }

    pub fn from_raw(raw: RawTrack) -> Result<Self> {
        let (lanes_left, lanes_left_data) = Self::map_lanes(raw.lanes_left, LaneType::Left)?;
        let (lanes_center, lanes_center_data) =
            Self::map_lanes(raw.lanes_center, LaneType::Center)?;
        let (lanes_right, lanes_right_data) = Self::map_lanes(raw.lanes_right, LaneType::Right)?;
        let (enemy_lanes, enemy_lanes_data) = Self::map_lanes(raw.enemy_lanes, LaneType::Enemy)?;
        let (walls_left, walls_left_data) = Self::map_walls(raw.walls_left, LaneType::WallLeft)?;
        let (walls_right, walls_right_data) =
            Self::map_walls(raw.walls_right, LaneType::WallRight)?;

        let lanes_data = {
            let mut data = lanes_left_data;
            data.extend(lanes_center_data);
            data.extend(lanes_right_data);
            data.extend(enemy_lanes_data);
            data.extend(walls_left_data);
            data.extend(walls_right_data);
            data
        };

        let (colorful_lanes, colorful_lanes_data) = Self::map_colorful_lanes(raw.colorful_lanes)?;
        let (beams, beams_data) = Self::map_beams(raw.beams)?;
        let (oblique_beams, oblique_beams_data) = Self::map_oblique_beams(raw.oblique_beams)?;

        Ok(Self {
            lanes_left,
            lanes_center,
            lanes_right,
            walls_left,
            walls_right,
            colorful_lanes,
            enemy_lanes,
            beams,
            oblique_beams,

            lanes_data,
            colorful_lanes_data,
            beams_data,
            oblique_beams_data,
        })
    }

    fn map_lanes(
        lanes: Vec<LaneSection>,
        lane_type: LaneType,
    ) -> Result<(BTreeMap<TimingPoint, Vec<LaneId>>, HashMap<LaneId, Lane>)> {
        let lanes_data = lanes
            .into_iter()
            .try_fold(HashMap::new(), |mut m, lane_section| {
                let lane = Lane::from_lane_section(lane_section, lane_type)?;

                if m.contains_key(&lane.id) {
                    log::warn!("Internal error: found duplicate lane ID {:?}, previous lane with this ID be ignored", lane.id);
                }
                m.insert(lane.id, lane);

                Ok(m)
            })?;

        let lanes_sorted = lanes_data
            .values()
            .try_fold(BTreeMap::new(), |mut m, lane| {
                m.entry(lane.points.first().unwrap().time)
                    .or_insert_with(Vec::new)
                    .push(lane.id);

                Ok(m)
            })?;

        Ok((lanes_sorted, lanes_data))
    }

    // XXX TODO: Remove `WallSection` since it is simply a lane.
    fn map_walls(
        walls: Vec<WallSection>,
        lane_type: LaneType,
    ) -> Result<(BTreeMap<TimingPoint, LaneId>, HashMap<LaneId, Lane>)> {
        let walls_data = walls
            .into_iter()
            .try_fold(HashMap::new(), |mut m, wall_section| {
                let wall = Lane::from_wall_section(wall_section, lane_type)?;

                if m.contains_key(&wall.id) {
                    log::warn!("Internal error: found duplicate wall ID {:?}, previous wall with this ID be ignored", wall.id);
                }
                m.insert(wall.id, wall);
                Ok(m)
            })?;

        let walls_sorted = walls_data
            .values()
            .try_fold(BTreeMap::new(), |mut m, wall| {
                // XXX FIXME: remove unwrap here and return Err if required.
                m.insert(wall.points.first().unwrap().time, wall.id);
                Ok(m)
            })?;

        Ok((walls_sorted, walls_data))
    }

    fn map_colorful_lanes(
        lanes: Vec<ColorfulLaneSection>,
    ) -> Result<(
        BTreeMap<TimingPoint, ColorfulLaneId>,
        HashMap<ColorfulLaneId, ColorfulLane>,
    )> {
        let lanes_data = lanes
            .into_iter()
            .try_fold(HashMap::new(), |mut m, lane_section| {
                let lane = ColorfulLane::from_section(lane_section)?;

                if m.contains_key(&lane.id) {
                    log::warn!("Internal error: found duplicate lane ID {:?}, previous lane with this ID be ignored", lane.id);
                }
                m.insert(lane.id, lane);
                Ok(m)
            })?;

        let lanes_sorted = lanes_data
            .values()
            .try_fold(BTreeMap::new(), |mut m, lane| {
                m.insert(lane.start.position.time, lane.id);
                Ok(m)
            })?;

        Ok((lanes_sorted, lanes_data))
    }

    fn map_beams(
        beams: Vec<BeamSection>,
    ) -> Result<(BTreeMap<TimingPoint, BeamId>, HashMap<BeamId, Beam>)> {
        let beams_data = beams
            .into_iter()
            .try_fold(HashMap::new(), |mut m, beam_section| {
                let beam = Beam::from_section(beam_section)?;

                if m.contains_key(&beam.id) {
                    log::warn!("Internal error: found duplicate beam ID {:?}, previous beam with this ID be ignored", beam.id);
                }
                m.insert(beam.id, beam);
                Ok(m)
            })?;

        let beams_sorted = beams_data
            .values()
            .try_fold(BTreeMap::new(), |mut m, beam| {
                m.insert(beam.start.position.time, beam.id);
                Ok(m)
            })?;

        Ok((beams_sorted, beams_data))
    }

    fn map_oblique_beams(
        beams: Vec<ObliqueBeamSection>,
    ) -> Result<(
        BTreeMap<TimingPoint, ObliqueBeamId>,
        HashMap<ObliqueBeamId, ObliqueBeam>,
    )> {
        let beams_data = beams
            .into_iter()
            .try_fold(HashMap::new(), |mut m, beam_section| {
                let beam = ObliqueBeam::from_section(beam_section)?;

                if m.contains_key(&beam.id) {
                    log::warn!("Internal error: found duplicate beam ID {:?}, previous beam with this ID be ignored", beam.id);
                }
                m.insert(beam.id, beam);
                Ok(m)
            })?;

        let beams_sorted = beams_data
            .values()
            .try_fold(BTreeMap::new(), |mut m, beam| {
                m.insert(beam.start.position.time, beam.id);
                Ok(m)
            })?;

        Ok((beams_sorted, beams_data))
    }
}

#[derive(Clone, Debug)]
pub struct Notes {
    pub taps: BTreeMap<TimingPoint, Vec<TapNote>>,
    pub holds: BTreeMap<TimingPoint, Vec<HoldNote>>,
    pub bells: BTreeMap<TimingPoint, Vec<BellNote>>,
    pub flicks: BTreeMap<TimingPoint, Vec<FlickNote>>,
}

impl Notes {
    pub fn from_raw(raw: RawNotes, track: &Track) -> Result<Self> {
        let taps = Self::map_tap_notes(raw.taps, track, false)?
            .into_iter()
            .chain(Self::map_tap_notes(raw.critical_taps, track, true)?)
            .collect::<BTreeMap<_, _>>();
        let holds = Self::map_hold_notes(raw.holds, track, false)?
            .into_iter()
            .chain(Self::map_hold_notes(raw.critical_holds, track, true)?)
            .collect::<BTreeMap<_, _>>();
        let bells = Self::map_bell_notes(raw.bells)?;
        let flicks = Self::map_flick_notes(raw.flicks, false)?
            .into_iter()
            .chain(Self::map_flick_notes(raw.critical_flicks, true)?)
            .collect::<BTreeMap<_, _>>();

        Ok(Self {
            taps,
            holds,
            bells,
            flicks,
        })
    }

    /// Returns iterator of tap notes sorted by time.
    pub fn all_taps(&self) -> impl Iterator<Item = &TapNote> {
        self.taps.values().flatten()
    }

    /// Creates a new array containing tap notes flattened from the sorted map.
    pub fn create_all_taps(&self) -> Vec<TapNote> {
        self.all_taps().cloned().collect()
    }

    /// Returns iterator of hold notes sorted by time.
    pub fn all_holds(&self) -> impl Iterator<Item = &HoldNote> {
        self.holds.values().flatten()
    }

    /// Creates a new array containing hold notes flattened from the sorted map.
    pub fn create_all_holds(&self) -> Vec<HoldNote> {
        self.all_holds().cloned().collect()
    }

    /// Returns iterator of bell notes sorted by time.
    pub fn all_bells(&self) -> impl Iterator<Item = &BellNote> {
        self.bells.values().flatten()
    }

    /// Creates a new array containing bell notes flattened from the sorted map.
    pub fn create_all_bells(&self) -> Vec<BellNote> {
        self.all_bells().cloned().collect()
    }

    /// Returns iterator of flick notes sorted by time.
    pub fn all_flicks(&self) -> impl Iterator<Item = &FlickNote> {
        self.flicks.values().flatten()
    }

    /// Creates a new array containing flick notes flattened from the sorted map.
    pub fn create_all_flicks(&self) -> Vec<FlickNote> {
        self.all_flicks().cloned().collect()
    }

    fn map_tap_notes(
        taps: Vec<command::Tap>,
        track: &Track,
        is_critical: bool,
    ) -> Result<BTreeMap<TimingPoint, Vec<TapNote>>> {
        taps.into_iter().try_fold(BTreeMap::new(), |mut m, note| {
            if let Some(lane) = track.get_lane(LaneId(note.lane_group_id)) {
                let tap_note = TapNote::from_tap(note, lane.lane_type, is_critical);
                m.entry(tap_note.position.time)
                    .or_insert_with(Vec::new)
                    .push(tap_note);
                Ok(m)
            } else {
                log::error!(
                    "Tap note {:?} uses invalid lane id {:?}",
                    &note,
                    note.lane_group_id
                );
                Err(ParseError::SemanticError(format!(
                    "Tap note {:?} uses invalid lane id {:?}",
                    &note, note.lane_group_id
                )))
            }
        })
    }

    fn map_hold_notes(
        holds: Vec<command::Hold>,
        track: &Track,
        is_critical: bool,
    ) -> Result<BTreeMap<TimingPoint, Vec<HoldNote>>> {
        holds.into_iter().try_fold(BTreeMap::new(), |mut m, note| {
            if let Some(lane) = track.get_lane(LaneId(note.lane_group_id)) {
                let hold_note = HoldNote::from_hold_and_lane(note, lane, is_critical)?;
                m.entry(hold_note.start.time)
                    .or_insert(Vec::new())
                    .push(hold_note);
                Ok(m)
            } else {
                log::error!(
                    "hold note {:?} uses invalid lane id {:?}",
                    &note,
                    note.lane_group_id
                );
                Err(ParseError::SemanticError(format!(
                    "hold note {:?} uses invalid lane id {:?}",
                    &note, note.lane_group_id
                )))
            }
        })
    }

    fn map_bell_notes(bells: Vec<command::Bell>) -> Result<BTreeMap<TimingPoint, Vec<BellNote>>> {
        bells.into_iter().try_fold(BTreeMap::new(), |mut m, note| {
            // XXX TODO: check that bullet palette exists if provided.
            let bell_note: BellNote = note.into();
            m.entry(bell_note.position.time)
                .or_insert(Vec::new())
                .push(bell_note);
            Ok(m)
        })
    }

    fn map_flick_notes(
        flicks: Vec<command::Flick>,
        is_critical: bool,
    ) -> Result<BTreeMap<TimingPoint, Vec<FlickNote>>> {
        flicks.into_iter().try_fold(BTreeMap::new(), |mut m, note| {
            let flick_note = FlickNote::from_flick(note, is_critical);
            m.entry(flick_note.position.time)
                .or_insert(Vec::new())
                .push(flick_note);
            Ok(m)
        })
    }
}

#[derive(Clone, Debug)]
pub struct Bullets {
    pub bullet_palette_list: HashMap<BulletPaletteId, BulletPalette>,
    pub bullets: BTreeMap<TimingPoint, Vec<Bullet>>,
}

impl Bullets {
    pub fn from_raw(
        palettes: Vec<command::BulletPalette>,
        bullets: Vec<command::Bullet>,
    ) -> Result<Bullets> {
        let bullet_palette_list = palettes.into_iter().fold(HashMap::new(), |mut m, p| {
            let palette = BulletPalette::from(p);
            m.insert(palette.id.clone(), palette);
            m
        });

        let bullets = bullets.into_iter().try_fold(BTreeMap::new(), |mut m, b| {
            let bullet = Bullet::from(b);
            if bullet_palette_list.contains_key(&bullet.palette_id) {
                m.entry(bullet.position.time)
                    .or_insert_with(Vec::new)
                    .push(bullet);
                Ok(m)
            } else {
                Err(ParseError::SemanticError(format!(
                    "Bullet {:?} uses invalid palette id {:?}",
                    &bullet, &bullet.palette_id
                )))
            }
        })?;

        Ok(Self {
            bullet_palette_list,
            bullets,
        })
    }

    pub fn get_bullet_palette(&self, id: &BulletPaletteId) -> Option<&BulletPalette> {
        self.bullet_palette_list.get(id)
    }

    /// Returns iterator of bullets sorted by time.
    pub fn all_bullets(&self) -> impl Iterator<Item = &Bullet> {
        self.bullets.values().flatten()
    }
}

#[derive(Clone, Debug)]
pub struct BpmChange {
    pub time: TimingPoint,
    pub bpm: u32,
}

impl From<command::BpmChange> for BpmChange {
    fn from(bpm_change: command::BpmChange) -> Self {
        Self {
            time: bpm_change.time.into(),
            bpm: bpm_change.bpm,
        }
    }
}

#[derive(Clone, Debug)]
pub struct MeterChange {
    pub time: TimingPoint,
    /// Time signature numerator.
    pub num_beats: u32,
    /// Time signature demoninator.
    pub note_value: u32,
}

impl From<command::MeterChange> for MeterChange {
    fn from(meter_change: command::MeterChange) -> Self {
        Self {
            time: meter_change.time.into(),
            num_beats: meter_change.num_beats,
            note_value: meter_change.note_value,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Soflan {
    pub time: TimingPoint,
    pub duration: u32,
    pub speed_multiplier: f32,
}

impl From<command::Soflan> for Soflan {
    fn from(soflan: command::Soflan) -> Self {
        Self {
            time: soflan.time.into(),
            duration: soflan.duration,
            speed_multiplier: f32::from_bits(soflan.current_speed_multiplier),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Composition {
    pub bpm_changes: BTreeMap<TimingPoint, BpmChange>,
    pub meter_changes: BTreeMap<TimingPoint, MeterChange>,
    pub soflans: BTreeMap<TimingPoint, Soflan>,
}

impl Composition {
    pub fn from_raw(raw: RawComposition) -> Self {
        let bpm_changes = raw
            .bpm_changes
            .into_iter()
            .fold(BTreeMap::new(), |mut m, b| {
                let bpm_change = BpmChange::from(b);
                m.insert(bpm_change.time, bpm_change);
                m
            });

        let meter_changes = raw
            .meter_changes
            .into_iter()
            .fold(BTreeMap::new(), |mut m, b| {
                let meter_change = MeterChange::from(b);
                m.insert(meter_change.time, meter_change);
                m
            });

        let soflans = raw.soflans.into_iter().fold(BTreeMap::new(), |mut m, b| {
            let soflan = Soflan::from(b);
            m.insert(soflan.time, soflan);
            m
        });

        Self {
            bpm_changes,
            meter_changes,
            soflans,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ClickSound {
    pub time: TimingPoint,
}
impl From<command::ClickSound> for ClickSound {
    fn from(click_sound: command::ClickSound) -> Self {
        Self {
            time: click_sound.time.into(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ExtraMetadata {
    pub num_measures: u32,
}

impl ExtraMetadata {
    fn new(track: &Track, notes: &Notes, bullets: &Bullets) -> Self {
        // XXX TODO: Properly check from all lanes, notes and bullets.
        let num_measures = track
            .walls_left
            .last_key_value()
            .unwrap()
            .0
            .measure
            .max(track.walls_right.last_key_value().unwrap().0.measure);

        Self { num_measures }
    }
}

#[derive(Clone, Debug)]
pub struct Ogkr {
    pub header: Header,
    pub composition: Composition,
    pub track: Track,
    pub notes: Notes,
    pub bullets: Bullets,
    pub click_sounds: Vec<ClickSound>,
    pub enemy_wave_assignment: EnemyWaveAssignment,
    pub extra_metadata: ExtraMetadata,
}

impl Ogkr {
    pub fn from_raw(raw: RawOgkr) -> Result<Self> {
        let header = raw.header;
        let composition = Composition::from_raw(raw.composition);
        let track = Track::from_raw(raw.track)?;
        let notes = Notes::from_raw(raw.notes, &track)?;
        let bullets = Bullets::from_raw(raw.bullet_pallete_list, raw.bullets)?;
        let click_sounds = Self::map_click_sounds(raw.click_sounds);
        let enemy_wave_assignment = raw.enemy_wave_assignment;
        let extra_metadata = ExtraMetadata::new(&track, &notes, &bullets);

        Ok(Self {
            header,
            composition,
            track,
            notes,
            bullets,
            click_sounds,
            enemy_wave_assignment,
            extra_metadata,
        })
    }

    fn map_click_sounds(click_sounds: Vec<command::ClickSound>) -> Vec<ClickSound> {
        click_sounds.into_iter().map(ClickSound::from).collect()
    }
}

/// XXX TODO: Handle random number generation for some fields, eg. bullets.
pub fn parse_raw_ogkr(raw: RawOgkr) -> Result<Ogkr> {
    Ogkr::from_raw(raw)
}
