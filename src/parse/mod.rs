pub mod composition;
pub mod header;
pub mod notes;
pub mod track;

use composition::Composition;
use header::Header;
use notes::Notes;
use thiserror::Error;
use track::{
    BeamSection, ColorfulLaneSection, EnemyLaneSection, LaneSection, ObliqueBeamSection, Track,
    WallSection,
};

use crate::lex::{
    command::*,
    token::{Token, TokenStream},
};
//
// #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
// pub struct TimingPoint {
//     /// Measure or bar the object is in.
//     pub measure: u32,
//     /// Offset from the start of the measure. Maximum value inside a measure is specified by [`TickResolution`].
//     pub beat_offset: u32,
// }
//
// impl TimingPoint {
//     pub fn new(measure: u32, beat_offset: u32) -> Self {
//         Self {
//             measure,
//             beat_offset,
//         }
//     }
// }
//
// impl PartialOrd for TimingPoint {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         Some(self.cmp(other))
//     }
// }
//
// impl Ord for TimingPoint {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         self.measure
//             .cmp(&other.measure)
//             .then(self.beat_offset.cmp(&other.beat_offset))
//     }
// }
//
// #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
// pub struct XPosition {
//     pub position: i32,
//     /// Uses [`XResolution`] as the width reference value.
//     pub offset: i32,
// }
//
// impl XPosition {
//     pub fn new(position: i32, offset: i32) -> Self {
//         Self { position, offset }
//     }
//
//     pub fn new_position(position: i32) -> Self {
//         Self::new(position, 0)
//     }
// }

#[derive(Clone, Debug, PartialEq, Eq, Hash, Error)]
pub enum ParseError {
    #[error("syntax error: {0}")]
    SyntaxError(String),
    #[error("semantic error: {0}")]
    SemanticError(String),
    #[error("semantic error, expected more commands: {0}")]
    SemanticErrorExpectedCommand(String),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct EnemyWaveAssignment {
    pub wave_1: CommandTime,
    pub wave_2: CommandTime,
    pub boss: CommandTime,
}

impl EnemyWaveAssignment {
    fn update_from_command(&mut self, command: EnemySet) {
        match command.wave {
            EnemyWave::Wave1 => self.wave_1 = command.time,
            EnemyWave::Wave2 => self.wave_2 = command.time,
            EnemyWave::Boss => self.boss = command.time,
        }
    }
}

pub type Result<T> = std::result::Result<T, ParseError>;

#[derive(Debug, Default)]
pub struct RawOgkr {
    /// Header information and metadata.
    pub header: Header,

    /// Musical(rhythmical) details of the game, such as bpms and time signatures.
    pub composition: Composition,

    /// Bullet pallete list for bullet behaviour.
    pub bullet_pallete_list: Vec<BulletPalette>,

    pub click_sounds: Vec<ClickSound>,
    pub enemy_wave_assignment: EnemyWaveAssignment,

    pub bullets: Vec<Bullet>,

    pub track: Track,
    pub notes: Notes,
}

pub(crate) struct Commands {
    /// Tokens/commands are in reverse order, simply pop from the end to consume next token.
    tokens: Vec<Token>,
}

impl Commands {
    fn new_from_token_stream(token_stream: TokenStream) -> Self {
        Self {
            tokens: token_stream.into_iter().rev().collect(),
        }
    }

    pub(crate) fn is_end(&self) -> bool {
        self.tokens.is_empty()
    }

    /// Consumes token and returns the token/command.
    pub(crate) fn next_command(&mut self) -> Option<Token> {
        self.tokens.pop()
    }

    pub(crate) fn err_semantic(&self, message: &str) -> ParseError {
        log::error!(
            "Semantically wrong command, next command is: {:?}",
            &self.tokens.last(),
        );
        ParseError::SemanticError(message.to_string())
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

            // Totals..
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
            Token::EnemyLaneStart(lane_point) => ogkr
                .track
                .enemy_lanes
                .push(EnemyLaneSection::from_commands(&mut commands, lane_point)?),
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
