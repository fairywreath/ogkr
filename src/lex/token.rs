use crate::lex::LexError;

use super::{command::*, cursor::Cursor, Result};

/// These tokens are not strictly lexical and and conforms to the syntax of a command line.
/// The "lexer" here handles syntax within a single line while the "parser" will handle the overall
/// grammatical and syntatical meaning accross lines.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Token {
    SectionName,

    // Header.
    Version(Version),
    Creator(Creator),
    BpmDefinition(BpmDefinition),
    MeterDefinition(MeterDefinition),
    TickResolution(TickResolution),
    XResolution(XResolution),
    ClickDefinition(ClickDefinition),
    Tutorial(Tutorial),
    BulletDamage(BulletDamage),
    HardBulletDamage(HardBulletDamage),
    DangerBulletDamage(DangerBulletDamage),
    BeamDamage(BeamDamage),
    ProgJudgeBpm(ProgJudgeBpm),

    // Totals.
    TotalNotes(TotalNotes),
    TotalTapNotes(TotalTapNotes),
    TotalHoldNotes(TotalHoldNotes),
    TotalSideNotes(TotalSideNotes),
    TotalSideHoldNotes(TotalSideHoldNotes),
    TotalFlickNotes(TotalFlickNotes),
    TotalBellNotes(TotalBellNotes),

    // Bullet palette.
    BulletPalette(BulletPalette),

    // Not used.
    Btp(Btp),

    // Composition
    BpmChange(BpmChange),
    MeterChange(MeterChange),
    Soflan(Soflan),

    // Click sounds.
    ClickSound(ClickSound),

    // Enemy wave assignment.
    EnemySet(EnemySet),

    // Walls and lanes.
    WallLeftStart(WallPoint),
    WallLeftNext(WallPoint),
    WallLeftEnd(WallPoint),
    WallRightStart(WallPoint),
    WallRightNext(WallPoint),
    WallRightEnd(WallPoint),
    LaneLeftStart(LanePoint),
    LaneLeftNext(LanePoint),
    LaneLeftEnd(LanePoint),
    LaneCenterStart(LanePoint),
    LaneCenterNext(LanePoint),
    LaneCenterEnd(LanePoint),
    LaneRightStart(LanePoint),
    LaneRightNext(LanePoint),
    LaneRightEnd(LanePoint),
    ColorfulLaneStart(ColorfulLanePoint),
    ColorfulLaneNext(ColorfulLanePoint),
    ColorfulLaneEnd(ColorfulLanePoint),
    EnemyLaneStart(EnemyLanePoint),
    EnemyLaneNext(EnemyLanePoint),
    EnemyLaneEnd(EnemyLanePoint),
    LaneDisappearance(LaneEvent),
    LaneBlock(LaneEvent),

    // Bullets.
    Bullet(Bullet),

    // Beams.
    BeamStart(BeamPoint),
    BeamNext(BeamPoint),
    BeamEnd(BeamPoint),
    ObliqueBeamStart(ObliqueBeamPoint),
    ObliqueBeamNext(ObliqueBeamPoint),
    ObliqueBeamEnd(ObliqueBeamPoint),

    // Notes.
    Bell(Bell),
    Flick(Flick),
    CriticalFlick(Flick),
    Tap(Tap),
    CriticalTap(Tap),
    Hold(Hold),
    CriticalHold(Hold),
}

impl Token {
    pub(crate) fn from_cursor(cursor: &mut Cursor) -> Result<Self> {
        loop {
            let command = cursor
                .next_token()
                .ok_or_else(|| cursor.err_expected_token("valid command"))?;

            if command.starts_with('[') {
                log::debug!("Ignoring section name {} line", command);
                cursor.current_remaining_line();
                return Ok(Self::SectionName);
            }

            break Ok(match command {
                "VERSION" => Self::Version(Version::from_cursor(cursor)?),
                "CREATOR" => Self::Creator(Creator::from_cursor(cursor)?),
                "BPM_DEF" => Self::BpmDefinition(BpmDefinition::from_cursor(cursor)?),
                "MET_DEF" => Self::MeterDefinition(MeterDefinition::from_cursor(cursor)?),
                "TRESOLUTION" => Self::TickResolution(TickResolution::from_cursor(cursor)?),
                "XRESOLUTION" => Self::XResolution(XResolution::from_cursor(cursor)?),
                "CLK_DEF" => Self::ClickDefinition(ClickDefinition::from_cursor(cursor)?),
                "TUTORIAL" => Self::Tutorial(Tutorial::from_cursor(cursor)?),
                "BULLET_DAMAGE" => Self::BulletDamage(BulletDamage::from_cursor(cursor)?),
                "HARDBULLET_DAMAGE" => {
                    Self::HardBulletDamage(HardBulletDamage::from_cursor(cursor)?)
                }
                "DANGERBULLET_DAMAGE" => {
                    Self::DangerBulletDamage(DangerBulletDamage::from_cursor(cursor)?)
                }
                "BEAM_DAMAGE" => Self::BeamDamage(BeamDamage::from_cursor(cursor)?),
                "T_TOTAL" => Self::TotalNotes(TotalNotes::from_cursor(cursor)?),
                "T_TAP" => Self::TotalTapNotes(TotalTapNotes::from_cursor(cursor)?),
                "T_HOLD" => Self::TotalHoldNotes(TotalHoldNotes::from_cursor(cursor)?),
                "T_SIDE" => Self::TotalSideNotes(TotalSideNotes::from_cursor(cursor)?),
                "T_SHOLD" => Self::TotalSideHoldNotes(TotalSideHoldNotes::from_cursor(cursor)?),
                "T_FLICK" => Self::TotalFlickNotes(TotalFlickNotes::from_cursor(cursor)?),
                "T_BELL" => Self::TotalBellNotes(TotalBellNotes::from_cursor(cursor)?),
                "PROGJUDGE_BPM" => Self::ProgJudgeBpm(ProgJudgeBpm::from_cursor(cursor)?),
                "BPL" => Self::BulletPalette(BulletPalette::from_cursor(cursor)?),
                "BTP" => Self::Btp(Btp),
                "BPM" => Self::BpmChange(BpmChange::from_cursor(cursor)?),
                "MET" => Self::MeterChange(MeterChange::from_cursor(cursor)?),
                "CLK" => Self::ClickSound(ClickSound::from_cursor(cursor)?),
                "SFL" => Self::Soflan(Soflan::from_cursor(cursor)?),
                "EST" => Self::EnemySet(EnemySet::from_cursor(cursor)?),
                "WLS" => Self::WallLeftStart(WallPoint::from_cursor(cursor)?),
                "WLN" => Self::WallLeftNext(WallPoint::from_cursor(cursor)?),
                "WLE" => Self::WallLeftEnd(WallPoint::from_cursor(cursor)?),
                "WRS" => Self::WallRightStart(WallPoint::from_cursor(cursor)?),
                "WRN" => Self::WallRightNext(WallPoint::from_cursor(cursor)?),
                "WRE" => Self::WallRightEnd(WallPoint::from_cursor(cursor)?),
                "LLS" => Self::LaneLeftStart(LanePoint::from_cursor(cursor)?),
                "LLN" => Self::LaneLeftNext(LanePoint::from_cursor(cursor)?),
                "LLE" => Self::LaneLeftEnd(LanePoint::from_cursor(cursor)?),
                "LCS" => Self::LaneCenterStart(LanePoint::from_cursor(cursor)?),
                "LCN" => Self::LaneCenterNext(LanePoint::from_cursor(cursor)?),
                "LCE" => Self::LaneCenterEnd(LanePoint::from_cursor(cursor)?),
                "LRS" => Self::LaneRightStart(LanePoint::from_cursor(cursor)?),
                "LRN" => Self::LaneRightNext(LanePoint::from_cursor(cursor)?),
                "LRE" => Self::LaneRightEnd(LanePoint::from_cursor(cursor)?),
                "CLS" => Self::ColorfulLaneStart(ColorfulLanePoint::from_cursor(cursor)?),
                "CLN" => Self::ColorfulLaneNext(ColorfulLanePoint::from_cursor(cursor)?),
                "CLE" => Self::ColorfulLaneEnd(ColorfulLanePoint::from_cursor(cursor)?),
                "ENS" => Self::EnemyLaneStart(EnemyLanePoint::from_cursor(cursor)?),
                "ENN" => Self::EnemyLaneNext(EnemyLanePoint::from_cursor(cursor)?),
                "ENE" => Self::EnemyLaneEnd(EnemyLanePoint::from_cursor(cursor)?),
                "LDP" => Self::LaneDisappearance(LaneEvent::from_cursor(cursor)?),
                "LBK" => Self::LaneBlock(LaneEvent::from_cursor(cursor)?),
                "BLT" => Self::Bullet(Bullet::from_cursor(cursor)?),
                "BMS" => Self::BeamStart(BeamPoint::from_cursor(cursor)?),
                "BMN" => Self::BeamNext(BeamPoint::from_cursor(cursor)?),
                "BME" => Self::BeamEnd(BeamPoint::from_cursor(cursor)?),
                "OBS" => Self::ObliqueBeamStart(ObliqueBeamPoint::from_cursor(cursor)?),
                "OBN" => Self::ObliqueBeamNext(ObliqueBeamPoint::from_cursor(cursor)?),
                "OBE" => Self::ObliqueBeamEnd(ObliqueBeamPoint::from_cursor(cursor)?),
                "BEL" => Self::Bell(Bell::from_cursor(cursor)?),
                "FLK" => Self::Flick(Flick::from_cursor(cursor)?),
                "CFK" => Self::CriticalFlick(Flick::from_cursor(cursor)?),
                "TAP" => Self::Tap(Tap::from_cursor(cursor)?),
                "CTP" | "XTP" => Self::CriticalTap(Tap::from_cursor(cursor)?),
                "HLD" => Self::Hold(Hold::from_cursor(cursor)?),
                "CHD" | "XHD" => Self::CriticalHold(Hold::from_cursor(cursor)?),
                _ => {
                    return Err(LexError::UnknownCommand {
                        line: cursor.line(),
                        col: cursor.col(),
                    })
                }
            });
        }
    }
}

pub struct TokenStream {
    tokens: Vec<Token>,
}

impl TokenStream {
    pub(crate) fn from_tokens(tokens: Vec<Token>) -> Self {
        Self { tokens }
    }

    pub fn iter(&self) -> TokenStreamIter<'_> {
        TokenStreamIter {
            iter: self.tokens.iter(),
        }
    }
}

impl IntoIterator for TokenStream {
    type Item = Token;
    type IntoIter = <Vec<Token> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.tokens.into_iter()
    }
}

#[derive(Debug)]
pub struct TokenStreamIter<'t> {
    iter: std::slice::Iter<'t, Token>,
}

impl<'t> Iterator for TokenStreamIter<'t> {
    type Item = &'t Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

fn next_token_or<'a>(cursor: &mut Cursor<'a>, message: &'static str) -> Result<&'a str> {
    cursor
        .next_token()
        .ok_or_else(|| cursor.err_expected_token(message))
}

fn next_token_u32_or<'a>(cursor: &mut Cursor<'a>, message: &'static str) -> Result<u32> {
    cursor
        .next_token()
        .ok_or_else(|| cursor.err_expected_token(message))?
        .parse::<u32>()
        .map_err(|_| cursor.err_expected_token(message))
}

fn next_token_f32_or<'a>(cursor: &mut Cursor<'a>, message: &'static str) -> Result<u32> {
    Ok(cursor
        .next_token()
        .ok_or_else(|| cursor.err_expected_token(message))?
        .parse::<f32>()
        .map_err(|_| cursor.err_expected_token(message))?
        .to_bits())
}

fn next_token_i32_or<'a>(cursor: &mut Cursor<'a>, message: &'static str) -> Result<i32> {
    cursor
        .next_token()
        .ok_or_else(|| cursor.err_expected_token(message))?
        .parse::<i32>()
        .map_err(|_| cursor.err_expected_token(message))
}

impl Version {
    pub(crate) fn from_cursor(cursor: &mut Cursor) -> Result<Self> {
        let major = next_token_u32_or(cursor, "Version major")?;
        let minor = next_token_u32_or(cursor, "Version minor")?;
        let release = next_token_u32_or(cursor, "Version release")?;

        Ok(Self {
            major,
            minor,
            release,
        })
    }
}

impl Creator {
    pub(crate) fn from_cursor(cursor: &mut Cursor) -> Result<Self> {
        Ok(Self {
            name: cursor.current_remaining_line().to_string(),
        })
    }
}

impl BpmDefinition {
    pub(crate) fn from_cursor(cursor: &mut Cursor) -> Result<Self> {
        let first = next_token_f32_or(cursor, "Bpm first")?;
        let common = next_token_f32_or(cursor, "Bpm common")?;
        let minimum = next_token_f32_or(cursor, "Bpm minimum")?;
        let maximum = next_token_f32_or(cursor, "Bpm maximum")?;

        Ok(Self {
            first,
            common,
            minimum,
            maximum,
        })
    }
}

impl MeterDefinition {
    pub(crate) fn from_cursor(cursor: &mut Cursor) -> Result<Self> {
        let num_beats = next_token_u32_or(cursor, "MeterDefinition num_beats")?;
        let note_value = next_token_u32_or(cursor, "MeterDefinition note_value")?;

        Ok(Self {
            num_beats,
            note_value,
        })
    }
}

impl TickResolution {
    pub(crate) fn from_cursor(cursor: &mut Cursor) -> Result<Self> {
        let resolution = next_token_u32_or(cursor, "TickResolution resolution")?;

        Ok(Self { resolution })
    }
}

impl XResolution {
    pub(crate) fn from_cursor(cursor: &mut Cursor) -> Result<Self> {
        let resolution = next_token_u32_or(cursor, "XResolution resolution")?;

        Ok(Self { resolution })
    }
}

impl ClickDefinition {
    pub(crate) fn from_cursor(cursor: &mut Cursor) -> Result<Self> {
        let value = next_token_u32_or(cursor, "ClickDefinition value")?;

        Ok(Self { value })
    }
}

impl Tutorial {
    pub(crate) fn from_cursor(cursor: &mut Cursor) -> Result<Self> {
        let value = next_token_u32_or(cursor, "Tutorial value")?;

        Ok(Self { value })
    }
}

impl BulletDamage {
    pub(crate) fn from_cursor(cursor: &mut Cursor) -> Result<Self> {
        let damage = next_token_f32_or(cursor, "BulletDamage damage")?;

        Ok(Self { damage })
    }
}

impl HardBulletDamage {
    pub(crate) fn from_cursor(cursor: &mut Cursor) -> Result<Self> {
        let damage = next_token_f32_or(cursor, "HardBulletDamage damage")?;

        Ok(Self { damage })
    }
}

impl DangerBulletDamage {
    pub(crate) fn from_cursor(cursor: &mut Cursor) -> Result<Self> {
        let damage = next_token_f32_or(cursor, "DangerBulletDamage damage")?;

        Ok(Self { damage })
    }
}

impl BeamDamage {
    pub(crate) fn from_cursor(cursor: &mut Cursor) -> Result<Self> {
        let damage = next_token_f32_or(cursor, "BeamDamage damage")?;

        Ok(Self { damage })
    }
}

impl TotalNotes {
    pub(crate) fn from_cursor(cursor: &mut Cursor) -> Result<Self> {
        let value = next_token_u32_or(cursor, "TotalNotes value")?;
        Ok(Self { value })
    }
}

impl TotalTapNotes {
    pub(crate) fn from_cursor(cursor: &mut Cursor) -> Result<Self> {
        let value = next_token_u32_or(cursor, "TotalTapNotes value")?;
        Ok(Self { value })
    }
}

impl TotalHoldNotes {
    pub(crate) fn from_cursor(cursor: &mut Cursor) -> Result<Self> {
        let value = next_token_u32_or(cursor, "TotalHoldNotes value")?;
        Ok(Self { value })
    }
}

impl TotalSideNotes {
    pub(crate) fn from_cursor(cursor: &mut Cursor) -> Result<Self> {
        let value = next_token_u32_or(cursor, "TotalSideNotes value")?;
        Ok(Self { value })
    }
}

impl TotalSideHoldNotes {
    pub(crate) fn from_cursor(cursor: &mut Cursor) -> Result<Self> {
        let value = next_token_u32_or(cursor, "TotalSideHoldNotes value")?;
        Ok(Self { value })
    }
}

impl TotalFlickNotes {
    pub(crate) fn from_cursor(cursor: &mut Cursor) -> Result<Self> {
        let value = next_token_u32_or(cursor, "TotalFlickNotes value")?;
        Ok(Self { value })
    }
}

impl TotalBellNotes {
    pub(crate) fn from_cursor(cursor: &mut Cursor) -> Result<Self> {
        let value = next_token_u32_or(cursor, "TotalBellNotes value")?;
        Ok(Self { value })
    }
}

impl ProgJudgeBpm {
    pub(crate) fn from_cursor(cursor: &mut Cursor) -> Result<Self> {
        let value = next_token_f32_or(cursor, "ProgJudgeBpm value")?;
        Ok(Self { value })
    }
}

impl BulletPalette {
    pub(crate) fn from_cursor(cursor: &mut Cursor) -> Result<Self> {
        let id = next_token_or(cursor, "BulletPalette id")?.to_owned();
        let shooter = BulletShooter::from_cursor(cursor)?;
        let target_x_offset = next_token_i32_or(cursor, "BulletPalette target_x_offset")?;
        let target = BulletTarget::from_cursor(cursor)?;
        let speed = next_token_f32_or(cursor, "BulletPalette speed")?;

        // Different versions have different syntax.Older versions have damage type here isntead of size, bullet type,
        // and random position offset.
        let next_token = cursor.peek_token().unwrap_or("");
        let (size, ty, random_position_offset, damage_type) =
            if let Ok(_) = BulletDamageType::from_str(next_token) {
                (
                    None,
                    None,
                    None,
                    Some(BulletDamageType::from_cursor(cursor)?),
                )
            } else {
                let size = BulletSize::from_cursor(cursor)?;
                let ty = BulletType::from_cursor(cursor)?;
                let random_position_offset =
                    next_token_i32_or(cursor, "BulletPalette random_position_offset")?;

                (Some(size), Some(ty), Some(random_position_offset), None)
            };

        Ok(Self {
            id,
            shooter,
            target_x_offset,
            target,
            speed,
            size,
            ty,
            random_position_offset,
            damage_type,
        })
    }
}

impl CommandTime {
    pub(crate) fn from_cursor(cursor: &mut Cursor, message: &'static str) -> Result<Self> {
        Ok(Self {
            measure: next_token_u32_or(cursor, message)?,
            offset: next_token_u32_or(cursor, message)?,
        })
    }
}

impl BpmChange {
    pub(crate) fn from_cursor(cursor: &mut Cursor) -> Result<Self> {
        Ok(Self {
            time: CommandTime::from_cursor(cursor, "BpmChange time")?,
            bpm: next_token_u32_or(cursor, "BpmChange bpm")?,
        })
    }
}

impl MeterChange {
    pub(crate) fn from_cursor(cursor: &mut Cursor) -> Result<Self> {
        Ok(Self {
            time: CommandTime::from_cursor(cursor, "MeterChange time")?,
            num_beats: next_token_u32_or(cursor, "MeterChange num_beats")?,
            note_value: next_token_u32_or(cursor, "MeterChange note_value")?,
        })
    }
}

impl ClickSound {
    pub(crate) fn from_cursor(cursor: &mut Cursor) -> Result<Self> {
        Ok(Self {
            time: CommandTime::from_cursor(cursor, "ClickSound time")?,
        })
    }
}

impl Soflan {
    pub(crate) fn from_cursor(cursor: &mut Cursor) -> Result<Self> {
        Ok(Self {
            time: CommandTime::from_cursor(cursor, "Soflan time")?,
            duration: next_token_u32_or(cursor, "Soflan duration")?,
            current_speed_multiplier: next_token_f32_or(cursor, "Soflan current_speed_multiplier")?,
        })
    }
}

impl EnemySet {
    pub(crate) fn from_cursor(cursor: &mut Cursor) -> Result<Self> {
        Ok(Self {
            time: CommandTime::from_cursor(cursor, "EnemySet time")?,
            wave: EnemyWave::from_cursor(cursor)?,
        })
    }
}

impl WallPoint {
    pub(crate) fn from_cursor(cursor: &mut Cursor) -> Result<Self> {
        Ok(Self {
            group_id: next_token_u32_or(cursor, "WallPoint group_id")?,
            time: CommandTime::from_cursor(cursor, "WallPoint time")?,
            x_position: next_token_i32_or(cursor, "WallPoint x_position")?,
        })
    }
}

impl LanePoint {
    pub(crate) fn from_cursor(cursor: &mut Cursor) -> Result<Self> {
        Ok(Self {
            group_id: next_token_u32_or(cursor, "LanePoint group_id")?,
            time: CommandTime::from_cursor(cursor, "LanePoint time")?,
            x_position: next_token_i32_or(cursor, "LanePoint x_position")?,
        })
    }
}

impl ColorfulLanePoint {
    pub(crate) fn from_cursor(cursor: &mut Cursor) -> Result<Self> {
        Ok(Self {
            group_id: next_token_u32_or(cursor, "ColorfulLanePoint group_id")?,
            time: CommandTime::from_cursor(cursor, "ColorfulLanePoint time")?,
            x_position: next_token_i32_or(cursor, "ColorfulLanePoint x_position")?,
            color: next_token_u32_or(cursor, "ColorfulLanePoint color")?,
            brightness: next_token_u32_or(cursor, "ColorfulLanePoint brightness")?,
        })
    }
}

impl EnemyLanePoint {
    pub(crate) fn from_cursor(cursor: &mut Cursor) -> Result<Self> {
        Ok(Self {
            group_id: next_token_u32_or(cursor, "EnemyLanePoint group_id")?,
            time: CommandTime::from_cursor(cursor, "EnemyLanePoint time")?,
            x_position: next_token_i32_or(cursor, "EnemyLanePoint x_position")?,
        })
    }
}

impl LaneEvent {
    pub(crate) fn from_cursor(cursor: &mut Cursor) -> Result<Self> {
        Ok(Self {
            group_id: next_token_u32_or(cursor, "LaneEvent group_id")?,
            start_time: CommandTime::from_cursor(cursor, "LaneEvent start_time")?,
            start_x_position: next_token_i32_or(cursor, "LaneEvent start_x_position")?,
            start_x_offset: next_token_i32_or(cursor, "LaneEvent start_x_offset")?,
            end_time: CommandTime::from_cursor(cursor, "LaneEvent end_time")?,
            end_x_position: next_token_i32_or(cursor, "LaneEvent end_x_position")?,
            end_x_offset: next_token_i32_or(cursor, "LaneEvent end_x_offset")?,
        })
    }
}

impl Bullet {
    pub(crate) fn from_cursor(cursor: &mut Cursor) -> Result<Self> {
        Ok(Self {
            pallete_id: next_token_or(cursor, "Bullet pallete_id")?.to_string(),
            time: CommandTime::from_cursor(cursor, "Bullet time")?,
            x_position: next_token_i32_or(cursor, "Bullet x_position")?,

            // XXX FIXME: Older versions damage type is specified in the palette list.
            // damage_type: BulletDamageType::from_cursor(cursor)?,
            damage_type: BulletDamageType::Normal,
        })
    }
}

impl BeamPoint {
    pub(crate) fn from_cursor(cursor: &mut Cursor) -> Result<Self> {
        Ok(Self {
            record_id: next_token_u32_or(cursor, "BeamPoint record_id")?,
            time: CommandTime::from_cursor(cursor, "BeamPoint time")?,
            x_position: next_token_i32_or(cursor, "BeamPoint x_position")?,
            width: next_token_u32_or(cursor, "BeamPoint width")?,
        })
    }
}

impl ObliqueBeamPoint {
    pub(crate) fn from_cursor(cursor: &mut Cursor) -> Result<Self> {
        Ok(Self {
            record_id: next_token_u32_or(cursor, "ObliqueBeamPoint record_id")?,
            time: CommandTime::from_cursor(cursor, "ObliqueBeamPoint time")?,
            x_position: next_token_i32_or(cursor, "ObliqueBeamPoint x_position")?,
            width: next_token_u32_or(cursor, "ObliqueBeamPoint width")?,
            shoot_position_x_offset: next_token_i32_or(
                cursor,
                "ObliqueBeamPoint shoot_position_x_offset",
            )?,
        })
    }
}

impl Bell {
    pub(crate) fn from_cursor(cursor: &mut Cursor) -> Result<Self> {
        Ok(Self {
            time: CommandTime::from_cursor(cursor, "Bell time")?,
            x_position: next_token_i32_or(cursor, "Bell x_position")?,
            bullet_palette_id: {
                let current_remaining_line = cursor.current_remaining_line();
                if !current_remaining_line.is_empty() {
                    Some(current_remaining_line.to_string())
                } else {
                    None
                }
            },
        })
    }
}

impl Flick {
    pub(crate) fn from_cursor(cursor: &mut Cursor) -> Result<Self> {
        Ok(Self {
            time: CommandTime::from_cursor(cursor, "Flick time")?,
            x_position: next_token_i32_or(cursor, "Flick x_position")?,
            direction: FlickDirection::from_cursor(cursor)?,
        })
    }
}

impl Tap {
    pub(crate) fn from_cursor(cursor: &mut Cursor) -> Result<Self> {
        Ok(Self {
            lane_group_id: next_token_u32_or(cursor, "Tap lane_group_id")?,
            time: CommandTime::from_cursor(cursor, "Tap time")?,
            x_position: next_token_i32_or(cursor, "Tap x_position")?,
            x_offset: next_token_i32_or(cursor, "Tap x_offset")?,
        })
    }
}

impl Hold {
    pub(crate) fn from_cursor(cursor: &mut Cursor) -> Result<Self> {
        Ok(Self {
            lane_group_id: next_token_u32_or(cursor, "Hold lane_group_id")?,
            start_time: CommandTime::from_cursor(cursor, "Hold start_time")?,
            start_x_position: next_token_i32_or(cursor, "Hold start_x_position")?,
            start_x_offset: next_token_i32_or(cursor, "Hold start_x_offset")?,
            end_time: CommandTime::from_cursor(cursor, "Hold end_time")?,
            end_x_position: next_token_i32_or(cursor, "Hold end_x_position")?,
            end_x_offset: next_token_i32_or(cursor, "Hold end_x_offset")?,
        })
    }
}
