use super::{cursor::Cursor, LexError, Result};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub release: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Creator {
    pub name: String,
}

/// Values are u32 bits that represent floats.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BpmDefinition {
    pub first: u32,
    pub common: u32,
    pub minimum: u32,
    pub maximum: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct MeterDefinition {
    /// Time signature numerator, number of beats in a measure.
    pub num_beats: u32,
    /// Time signature denominator, value of a beat in a measure.
    pub note_value: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TickResolution {
    pub resolution: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct XResolution {
    pub resolution: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ClickDefinition {
    pub value: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Tutorial {
    pub value: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BulletDamage {
    /// f32 represented as u32.
    pub damage: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct HardBulletDamage {
    /// f32 represented as u32.
    pub damage: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DangerBulletDamage {
    /// f32 represented as u32.
    pub damage: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BeamDamage {
    /// f32 represented as u32.
    pub damage: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TotalNotes {
    pub value: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TotalTapNotes {
    pub value: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TotalHoldNotes {
    pub value: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TotalSideNotes {
    pub value: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TotalSideHoldNotes {
    pub value: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TotalFlickNotes {
    pub value: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TotalBellNotes {
    pub value: u32,
}

/// The meaning of this command is still unknown.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ProgJudgeBpm {
    /// f32 represented as u32.
    pub value: u32,
}

/// Bullet source position.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BulletShooter {
    EndPosition,
    Enemy,
    Center,
}

impl BulletShooter {
    pub(crate) fn from_cursor(cursor: &mut Cursor) -> Result<Self> {
        Ok(match cursor.next_token() {
            Some("UPS") => BulletShooter::EndPosition,
            Some("ENE") => BulletShooter::Enemy,
            Some("CEN") => BulletShooter::Center,
            _ => return Err(cursor.err_expected_token("one of UPS, ENE, or CEN")),
        })
    }
}

/// Bullet target position.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BulletTarget {
    Player,
    FixedPosition,
}

impl BulletTarget {
    pub(crate) fn from_cursor(cursor: &mut Cursor) -> Result<Self> {
        Ok(match cursor.next_token() {
            Some("PLR") => BulletTarget::Player,
            Some("FIX") => BulletTarget::FixedPosition,
            _ => return Err(cursor.err_expected_token("one of PLR or FIX")),
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BulletSize {
    Normal,
    Large,
}

impl BulletSize {
    pub(crate) fn from_cursor(cursor: &mut Cursor) -> Result<Self> {
        Ok(match cursor.next_token() {
            Some("N") => BulletSize::Normal,
            Some("L") => BulletSize::Large,
            _ => return Err(cursor.err_expected_token("one of N or L")),
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BulletType {
    Circle,
    Square,
    Needle,
}

impl BulletType {
    pub(crate) fn from_cursor(cursor: &mut Cursor) -> Result<Self> {
        Ok(match cursor.next_token() {
            Some("CIR") => BulletType::Circle,
            Some("SQR") => BulletType::Square,
            Some("NDL") => BulletType::Needle,
            _ => return Err(cursor.err_expected_token("one of CIR, SQR or NDL")),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BulletPalette {
    pub id: String,
    pub shooter: BulletShooter,

    /// Add x offset to the x position obtained from `target` to obtain the bullet's final target
    /// position.
    pub target_x_offset: i32,
    pub target: BulletTarget,

    /// f32 represented as u32.
    pub speed: u32,

    pub size: Option<BulletSize>,
    pub ty: Option<BulletType>,

    /// +/- x position offset is applied on random from zero to this value.
    pub random_position_offset: Option<i32>,

    pub damage_type: Option<BulletDamageType>,
}

/// Unused command.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Btp;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct CommandTime {
    pub measure: u32,
    pub offset: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BpmChange {
    pub time: CommandTime,
    pub bpm: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MeterChange {
    pub time: CommandTime,
    pub num_beats: u32,
    pub note_value: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ClickSound {
    pub time: CommandTime,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Soflan {
    pub time: CommandTime,

    /// Duration in terms of a resolution within the measure, similar to `offset`.
    pub duration: u32,

    /// f32 represented as u32 in bits.
    pub current_speed_multiplier: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EnemyWave {
    Wave1,
    Wave2,
    Boss,
}

impl EnemyWave {
    pub(crate) fn from_cursor(cursor: &mut Cursor) -> Result<Self> {
        Ok(match cursor.next_token() {
            Some("WAVE1") => EnemyWave::Wave1,
            Some("WAVE2") => EnemyWave::Wave2,
            Some("BOSS") => EnemyWave::Boss,
            _ => return Err(cursor.err_expected_token("one of WAVE1, WAVE2 or BOSS")),
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EnemySet {
    pub time: CommandTime,
    pub wave: EnemyWave,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct WallPoint {
    pub group_id: u32,
    pub time: CommandTime,
    pub x_position: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LanePoint {
    pub group_id: u32,
    pub time: CommandTime,
    pub x_position: i32,
}

// XXX FIXME: Remove `EnemyLanePoint`.
impl From<EnemyLanePoint> for LanePoint {
    fn from(point: EnemyLanePoint) -> Self {
        Self {
            group_id: point.group_id,
            time: point.time,
            x_position: point.x_position,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ColorfulLanePoint {
    pub group_id: u32,
    pub time: CommandTime,
    pub x_position: i32,
    pub color: u32,
    pub brightness: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EnemyLanePoint {
    pub group_id: u32,
    pub time: CommandTime,
    pub x_position: i32,
}

/// Used for lane dissaperance and lane block.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LaneEvent {
    pub group_id: u32,
    pub start_time: CommandTime,
    pub start_x_position: i32,
    pub start_x_offset: i32,
    pub end_time: CommandTime,
    pub end_x_position: i32,
    pub end_x_offset: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BulletDamageType {
    Normal,
    Hard,
    Danger,
}

impl BulletDamageType {
    pub(crate) fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            "NML" => BulletDamageType::Normal,
            "STR" => BulletDamageType::Hard,
            "DNG" => BulletDamageType::Danger,
            _ => {
                return Err(LexError::ExpectedToken {
                    line: 0,
                    col: 0,
                    message: "internal dummy error",
                })
            }
        })
    }

    pub(crate) fn from_cursor(cursor: &mut Cursor) -> Result<Self> {
        Ok(match cursor.next_token() {
            Some("NML") => BulletDamageType::Normal,
            Some("STR") => BulletDamageType::Hard,
            Some("DNG") => BulletDamageType::Danger,
            _ => return Err(cursor.err_expected_token("one of NML, STR, or DNG")),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Bullet {
    pub pallete_id: String,
    pub time: CommandTime,
    pub x_position: i32,
    pub damage_type: BulletDamageType,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BeamPoint {
    pub record_id: u32,
    pub time: CommandTime,
    pub x_position: i32,
    pub width: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ObliqueBeamPoint {
    pub record_id: u32,
    pub time: CommandTime,
    pub x_position: i32,
    pub width: u32,
    pub shoot_position_x_offset: i32,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Bell {
    pub time: CommandTime,
    pub x_position: i32,
    pub bullet_palette_id: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FlickDirection {
    Left,
    Right,
}

impl FlickDirection {
    pub(crate) fn from_cursor(cursor: &mut Cursor) -> Result<Self> {
        Ok(match cursor.next_token() {
            Some("L") => FlickDirection::Left,
            Some("R") => FlickDirection::Right,
            _ => return Err(cursor.err_expected_token("one of L or R")),
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Flick {
    pub time: CommandTime,
    pub x_position: i32,
    pub direction: FlickDirection,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Tap {
    pub lane_group_id: u32,
    pub time: CommandTime,
    pub x_position: i32,
    pub x_offset: i32,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Hold {
    pub lane_group_id: u32,

    pub start_time: CommandTime,
    pub start_x_position: i32,
    pub start_x_offset: i32,

    pub end_time: CommandTime,
    pub end_x_position: i32,
    pub end_x_offset: i32,
}
