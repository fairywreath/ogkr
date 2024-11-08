use crate::lex::{
    command::*,
    token::{Token, TokenStream},
};

use super::{Commands, EnemyWaveAssignment, Header, ParseError, Result};

#[derive(Debug, Default)]
pub struct RawOgkr {
    /// Header information and metadata.
    pub header: Header,

    /// Musical(rhythmical) details of the game, such as bpms and time signatures.
    pub composition: RawComposition,

    /// Bullet pallete list for bullet behaviour.
    pub bullet_pallete_list: Vec<BulletPalette>,
    pub bullets: Vec<Bullet>,

    pub click_sounds: Vec<ClickSound>,
    pub enemy_wave_assignment: EnemyWaveAssignment,

    pub track: RawTrack,
    pub notes: RawNotes,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct RawComposition {
    pub bpm_first: u32,
    pub bpm_changes: Vec<BpmChange>,
    pub meter_first: MeterDefinition,
    pub meter_changes: Vec<MeterChange>,
    pub soflans: Vec<Soflan>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct RawNotes {
    pub bells: Vec<Bell>,
    pub flicks: Vec<Flick>,
    pub critical_flicks: Vec<Flick>,
    pub taps: Vec<Tap>,
    pub critical_taps: Vec<Tap>,
    pub holds: Vec<Hold>,
    pub critical_holds: Vec<Hold>,
}

/// Walls, lanes, and beams are grammar-enforce to be consequetive.
/// XXX FIXME: Handle non-consequetive, out-of-order, objects. The group id should suffice to
/// provide grouping information.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct RawTrack {
    pub walls_left: Vec<WallSection>,
    pub walls_right: Vec<WallSection>,

    pub lanes_left: Vec<LaneSection>,
    pub lanes_center: Vec<LaneSection>,
    pub lanes_right: Vec<LaneSection>,

    pub colorful_lanes: Vec<ColorfulLaneSection>,
    pub enemy_lanes: Vec<LaneSection>,

    pub lane_disappearances: Vec<LaneEvent>,
    pub lane_blocks: Vec<LaneEvent>,

    pub beams: Vec<BeamSection>,
    pub oblique_beams: Vec<ObliqueBeamSection>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct WallSection {
    pub group_id: u32,
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

    pub(crate) fn enemy_lane_from_commands(
        commands: &mut Commands,
        first_point: EnemyLanePoint,
    ) -> Result<Self> {
        let group_id = first_point.group_id;
        let mut points = Vec::new();
        points.push(first_point.into());

        loop {
            match next_token_or(commands, "more commands for enemy lane section")? {
                Token::EnemyLaneNext(lane_point) => {
                    verify_group_id(commands, group_id, lane_point.group_id)?;
                    points.push(lane_point.into());
                }
                Token::EnemyLaneEnd(lane_point) => {
                    verify_group_id(commands, group_id, lane_point.group_id)?;
                    points.push(lane_point.into());
                    break;
                }
                _ => return Err(commands.err_semantic("unexpected command on enemy lane section")),
            }
        }

        Ok(Self { group_id, points })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct ColorfulLaneSection {
    pub group_id: u32,
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
pub struct BeamSection {
    pub record_id: u32,
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

/// Parses command tokens.
pub fn parse_tokens(token_stream: TokenStream) -> Result<RawOgkr> {
    let mut commands = Commands::new_from_token_stream(token_stream);
    let mut ogkr = RawOgkr::default();

    // Commands can be out-of-order or not grouped by sections, except for walls, lanes and beams
    // with distance start, next and end commands.
    while let Some(token) = commands.next_command() {
        match token {
            Token::SectionName => continue,

            // Header.
            Token::Version(version) => ogkr.header.version = Some(version),
            Token::Creator(creator) => ogkr.header.creator = Some(creator),
            Token::BpmDefinition(bpm_def) => {
                ogkr.header.bpm_definition = Some(bpm_def);
                ogkr.composition.bpm_first = bpm_def.first;
            }
            Token::MeterDefinition(meter_def) => {
                ogkr.header.meter_definition = Some(meter_def);
                ogkr.composition.meter_first = meter_def;
            }
            Token::TickResolution(tick_res) => ogkr.header.tick_resolution = Some(tick_res),
            Token::XResolution(x_res) => ogkr.header.x_resolution = Some(x_res),
            Token::ClickDefinition(click_def) => ogkr.header.click_definition = Some(click_def),
            Token::Tutorial(tutorial) => ogkr.header.tutorial = Some(tutorial),
            Token::BulletDamage(bullet_damage) => {
                ogkr.header.damage_values.normal = bullet_damage.damage
            }
            Token::HardBulletDamage(hard_bullet_damage) => {
                ogkr.header.damage_values.hard = hard_bullet_damage.damage
            }
            Token::DangerBulletDamage(danger_bullet_damage) => {
                ogkr.header.damage_values.danger = danger_bullet_damage.damage
            }
            Token::BeamDamage(beam_damage) => ogkr.header.damage_values.beam = beam_damage.damage,
            Token::ProgJudgeBpm(prog_judge_bpm) => {
                ogkr.header.prog_judge_bpm = Some(prog_judge_bpm)
            }

            // Totals.
            Token::TotalNotes(total_notes) => ogkr.header.totals.notes = total_notes.value,
            Token::TotalTapNotes(total_tap_notes) => {
                ogkr.header.totals.notes = total_tap_notes.value
            }
            Token::TotalHoldNotes(total_hold_notes) => {
                ogkr.header.totals.hold = total_hold_notes.value
            }
            Token::TotalSideNotes(total_side_notes) => {
                ogkr.header.totals.side = total_side_notes.value
            }
            Token::TotalSideHoldNotes(total_side_hold_notes) => {
                ogkr.header.totals.side = total_side_hold_notes.value
            }
            Token::TotalFlickNotes(total_flick_notes) => {
                ogkr.header.totals.flick = total_flick_notes.value
            }
            Token::TotalBellNotes(total_bell_notes) => {
                ogkr.header.totals.bell = total_bell_notes.value
            }

            // Bullet palette.
            Token::BulletPalette(bullet_palette) => ogkr.bullet_pallete_list.push(bullet_palette),

            // Composition.
            Token::BpmChange(bpm_change) => ogkr.composition.bpm_changes.push(bpm_change),
            Token::MeterChange(meter_change) => ogkr.composition.meter_changes.push(meter_change),
            Token::Soflan(soflan) => ogkr.composition.soflans.push(soflan),

            // Click sounds.
            Token::ClickSound(click_sound) => ogkr.click_sounds.push(click_sound),

            // Enemy wave assignment.
            Token::EnemySet(enemy_set) => ogkr.enemy_wave_assignment.update_from_command(enemy_set),

            // Walls and lanes.
            Token::WallLeftStart(wall_point) => {
                ogkr.track
                    .walls_left
                    .push(WallSection::wall_left_from_commands(
                        &mut commands,
                        wall_point,
                    )?)
            }
            Token::WallRightStart(wall_point) => {
                ogkr.track
                    .walls_right
                    .push(WallSection::wall_right_from_commands(
                        &mut commands,
                        wall_point,
                    )?)
            }
            Token::LaneLeftStart(lane_point) => {
                ogkr.track
                    .lanes_left
                    .push(LaneSection::lane_left_from_commands(
                        &mut commands,
                        lane_point,
                    )?)
            }

            Token::LaneCenterStart(lane_point) => {
                ogkr.track
                    .lanes_center
                    .push(LaneSection::lane_center_from_commands(
                        &mut commands,
                        lane_point,
                    )?)
            }
            Token::LaneRightStart(lane_point) => {
                ogkr.track
                    .lanes_right
                    .push(LaneSection::lane_right_from_commands(
                        &mut commands,
                        lane_point,
                    )?)
            }
            Token::ColorfulLaneStart(lane_point) => {
                ogkr.track
                    .colorful_lanes
                    .push(ColorfulLaneSection::from_commands(
                        &mut commands,
                        lane_point,
                    )?)
            }
            Token::EnemyLaneStart(lane_point) => {
                ogkr.track
                    .enemy_lanes
                    .push(LaneSection::enemy_lane_from_commands(
                        &mut commands,
                        lane_point,
                    )?)
            }
            Token::LaneDisappearance(lane_disp) => ogkr.track.lane_disappearances.push(lane_disp),
            Token::LaneBlock(lane_block) => ogkr.track.lane_blocks.push(lane_block),

            // Bullets.
            Token::Bullet(bullet) => ogkr.bullets.push(bullet),

            // Beams.
            Token::BeamStart(beam_point) => ogkr
                .track
                .beams
                .push(BeamSection::from_commands(&mut commands, beam_point)?),
            Token::ObliqueBeamStart(beam_point) => {
                ogkr.track
                    .oblique_beams
                    .push(ObliqueBeamSection::from_commands(
                        &mut commands,
                        beam_point,
                    )?)
            }

            // Notes.
            Token::Bell(bell) => ogkr.notes.bells.push(bell),
            Token::Flick(flick) => ogkr.notes.flicks.push(flick),
            Token::CriticalFlick(critical_flick) => ogkr.notes.critical_flicks.push(critical_flick),
            Token::Tap(tap) => ogkr.notes.taps.push(tap),
            Token::CriticalTap(critical_tap) => ogkr.notes.critical_taps.push(critical_tap),
            Token::Hold(hold) => ogkr.notes.holds.push(hold),
            Token::CriticalHold(critical_hold) => ogkr.notes.critical_holds.push(critical_hold),

            // Unexpected commands.
            _ => {
                return Err(ParseError::SyntaxError(format!(
                    "Unexpected command token {:?}",
                    token
                )))
            }
        }
    }

    Ok(ogkr)
}
