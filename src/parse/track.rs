use crate::lex::{
    command::{BeamPoint, BulletType, LaneEvent, LanePoint, WallPoint},
    token::Token,
};

use super::{
    ClickSound, ColorfulLanePoint, Commands, EnemyLanePoint, FlickDirection, ObliqueBeamPoint,
    ParseError, Result,
};

/// Walls, lanes, and beams are grammar-enforce to be consequetive.
/// XXX FIXME: Handle non-consequetive, out-of-order, objects. The group id should suffice to
/// provide grouping information.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct Track {
    pub walls_left: Vec<WallSection>,
    pub walls_right: Vec<WallSection>,

    pub lanes_left: Vec<LaneSection>,
    pub lanes_center: Vec<LaneSection>,
    pub lanes_right: Vec<LaneSection>,

    pub colorful_lanes: Vec<ColorfulLaneSection>,
    pub enemy_lanes: Vec<EnemyLaneSection>,

    pub lane_disappearances: Vec<LaneEvent>,
    pub lane_blocks: Vec<LaneEvent>,

    pub beams: Vec<BeamSection>,
    pub oblique_beams: Vec<ObliqueBeamSection>,
}

// #[derive(Clone, Debug, PartialEq, Eq, Hash)]
// pub struct TrackPoint {
//     pub time: TimingPoint,
//     pub x: XPosition,
// }

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct WallSection {
    pub group_id: u32,
    // pub points: Vec<TrackPoint>,
    pub points: Vec<WallPoint>,
}

fn next_token_or(commands: &mut Commands, message: &str) -> Result<Token> {
    commands
        .next_command()
        .ok_or_else(|| ParseError::SemanticErrorExpectedCommand(message.to_string()))
}

fn verify_group_id(commands: &Commands, reference_id: u32, new_id: u32) -> Result<()> {
    if reference_id != new_id {
        return Err(commands.err_semantic("different group ids for consequetive section"));
    } else {
        Ok(())
    }
}

// XXX FIXME: Remove code duplication here?.
impl WallSection {
    pub(crate) fn wall_left_from_commands(
        commands: &mut Commands,
        first_point: WallPoint,
    ) -> Result<Self> {
        let group_id = first_point.group_id;
        let mut points = Vec::new();
        points.push(first_point);

        loop {
            match next_token_or(commands, "more commands for left wall section")? {
                Token::WallLeftNext(wall_point) => {
                    verify_group_id(commands, group_id, wall_point.group_id)?;
                    points.push(wall_point);
                }
                Token::WallLeftEnd(wall_point) => {
                    verify_group_id(commands, group_id, wall_point.group_id)?;
                    points.push(wall_point);
                    break;
                }
                _ => return Err(commands.err_semantic("unexpected command on left wall section")),
            }
        }

        Ok(Self { group_id, points })
    }

    pub(crate) fn wall_right_from_commands(
        commands: &mut Commands,
        first_point: WallPoint,
    ) -> Result<Self> {
        let group_id = first_point.group_id;
        let mut points = Vec::new();
        points.push(first_point);

        loop {
            match next_token_or(commands, "more commands for right wall section")? {
                Token::WallRightNext(wall_point) => {
                    verify_group_id(commands, group_id, wall_point.group_id)?;
                    points.push(wall_point);
                }
                Token::WallRightEnd(wall_point) => {
                    verify_group_id(commands, group_id, wall_point.group_id)?;
                    points.push(wall_point);
                    break;
                }
                _ => return Err(commands.err_semantic("unexpected command on right wall section")),
            }
        }

        Ok(Self { group_id, points })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct LaneSection {
    pub group_id: u32,
    // pub points: Vec<TrackPoint>,
    pub points: Vec<LanePoint>,
}

// XXX FIXME: Remove code duplication here?.
impl LaneSection {
    pub(crate) fn lane_left_from_commands(
        commands: &mut Commands,
        first_point: LanePoint,
    ) -> Result<Self> {
        let group_id = first_point.group_id;
        let mut points = Vec::new();
        points.push(first_point);

        loop {
            match next_token_or(commands, "more commands for left lane section")? {
                Token::LaneLeftNext(lane_point) => {
                    verify_group_id(commands, group_id, lane_point.group_id)?;
                    points.push(lane_point);
                }
                Token::LaneLeftEnd(lane_point) => {
                    verify_group_id(commands, group_id, lane_point.group_id)?;
                    points.push(lane_point);
                    break;
                }
                _ => return Err(commands.err_semantic("unexpected command on left lane section")),
            }
        }

        Ok(Self { group_id, points })
    }

    pub(crate) fn lane_center_from_commands(
        commands: &mut Commands,
        first_point: LanePoint,
    ) -> Result<Self> {
        let group_id = first_point.group_id;
        let mut points = Vec::new();
        points.push(first_point);

        loop {
            match next_token_or(commands, "more commands for center lane section")? {
                Token::LaneCenterNext(lane_point) => {
                    verify_group_id(commands, group_id, lane_point.group_id)?;
                    points.push(lane_point);
                }
                Token::LaneCenterEnd(lane_point) => {
                    verify_group_id(commands, group_id, lane_point.group_id)?;
                    points.push(lane_point);
                    break;
                }
                _ => return Err(commands.err_semantic("unexpected command on center lane section")),
            }
        }

        Ok(Self { group_id, points })
    }

    pub(crate) fn lane_right_from_commands(
        commands: &mut Commands,
        first_point: LanePoint,
    ) -> Result<Self> {
        let group_id = first_point.group_id;
        let mut points = Vec::new();
        points.push(first_point);

        loop {
            match next_token_or(commands, "more commands for right lane section")? {
                Token::LaneRightNext(lane_point) => {
                    verify_group_id(commands, group_id, lane_point.group_id)?;
                    points.push(lane_point);
                }
                Token::LaneRightEnd(lane_point) => {
                    verify_group_id(commands, group_id, lane_point.group_id)?;
                    points.push(lane_point);
                    break;
                }
                _ => return Err(commands.err_semantic("unexpected command on right lane section")),
            }
        }

        Ok(Self { group_id, points })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct ColorfulLaneSection {
    pub group_id: u32,
    // pub points: Vec<TrackPoint>,
    pub points: Vec<ColorfulLanePoint>,
}

impl ColorfulLaneSection {
    pub(crate) fn from_commands(
        commands: &mut Commands,
        first_point: ColorfulLanePoint,
    ) -> Result<Self> {
        let group_id = first_point.group_id;
        let mut points = Vec::new();
        points.push(first_point);

        loop {
            match next_token_or(commands, "more commands for colorful lane section")? {
                Token::ColorfulLaneNext(lane_point) => {
                    verify_group_id(commands, group_id, lane_point.group_id)?;
                    points.push(lane_point);
                }
                Token::ColorfulLaneEnd(lane_point) => {
                    verify_group_id(commands, group_id, lane_point.group_id)?;
                    points.push(lane_point);
                    break;
                }
                _ => {
                    return Err(commands.err_semantic("unexpected command on colorful lane section"))
                }
            }
        }

        Ok(Self { group_id, points })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct EnemyLaneSection {
    pub group_id: u32,
    // pub points: Vec<TrackPoint>,
    pub points: Vec<EnemyLanePoint>,
}

impl EnemyLaneSection {
    pub(crate) fn from_commands(
        commands: &mut Commands,
        first_point: EnemyLanePoint,
    ) -> Result<Self> {
        let group_id = first_point.group_id;
        let mut points = Vec::new();
        points.push(first_point);

        loop {
            match next_token_or(commands, "more commands for enemy lane section")? {
                Token::EnemyLaneNext(lane_point) => {
                    verify_group_id(commands, group_id, lane_point.group_id)?;
                    points.push(lane_point);
                }
                Token::EnemyLaneEnd(lane_point) => {
                    verify_group_id(commands, group_id, lane_point.group_id)?;
                    points.push(lane_point);
                    break;
                }
                _ => return Err(commands.err_semantic("unexpected command on enemy lane section")),
            }
        }

        Ok(Self { group_id, points })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct BeamSection {
    pub record_id: u32,
    // pub points: Vec<TrackPoint>,
    pub points: Vec<BeamPoint>,
    // pub width_id: u32,
}

impl BeamSection {
    pub(crate) fn from_commands(commands: &mut Commands, first_point: BeamPoint) -> Result<Self> {
        let record_id = first_point.record_id;

        // XXX TODO: Figure out what this does.
        // let width_id = first_point.width;

        let mut points = Vec::new();
        points.push(first_point);

        loop {
            match next_token_or(commands, "more commands for enemy lane section")? {
                Token::BeamNext(beam_point) => {
                    verify_group_id(commands, record_id, beam_point.record_id)?;
                    points.push(beam_point);
                }
                Token::BeamEnd(beam_point) => {
                    verify_group_id(commands, record_id, beam_point.record_id)?;
                    points.push(beam_point);
                    break;
                }
                _ => return Err(commands.err_semantic("unexpected command on enemy lane section")),
            }
        }

        Ok(Self {
            record_id,
            points,
            // width_id,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct ObliqueBeamSection {
    pub record_id: u32,
    // pub points: Vec<TrackPoint>,
    pub points: Vec<ObliqueBeamPoint>,
}

impl ObliqueBeamSection {
    pub(crate) fn from_commands(
        commands: &mut Commands,
        first_point: ObliqueBeamPoint,
    ) -> Result<Self> {
        let record_id = first_point.record_id;

        // XXX TODO: Figure out what this does.
        // let width_id = first_point.width;

        let mut points = Vec::new();
        points.push(first_point);

        loop {
            match next_token_or(commands, "more commands for enemy lane section")? {
                Token::ObliqueBeamNext(beam_point) => {
                    verify_group_id(commands, record_id, beam_point.record_id)?;
                    points.push(beam_point);
                }
                Token::ObliqueBeamEnd(beam_point) => {
                    verify_group_id(commands, record_id, beam_point.record_id)?;
                    points.push(beam_point);
                    break;
                }
                _ => return Err(commands.err_semantic("unexpected command on enemy lane section")),
            }
        }

        Ok(Self {
            record_id,
            points,
            // width_id,
        })
    }
}

// #[derive(Clone, Debug, PartialEq, Eq, Hash)]
// pub struct LaneEvent {
//     pub group_id: u32,
//     pub start_point: TrackPoint,
//     pub end_point: TrackPoint,
// }

// #[derive(Clone, Debug, PartialEq, Eq, Hash)]
// pub struct Bullet {
//     pub id: String,
//     pub point: TrackPoint,
//     pub bullet_type: BulletType,
// }
