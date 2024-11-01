use crate::lex::command::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct Header {
    pub version: Option<Version>,
    pub creator: Option<Creator>,
    pub bpm_definition: Option<BpmDefinition>,
    pub meter_definition: Option<MeterDefinition>,
    pub tick_resolution: Option<TickResolution>,
    pub x_resolution: Option<XResolution>,
    pub click_definition: Option<ClickDefinition>,
    pub tutorial: Option<Tutorial>,
    pub damage_values: DamageValues,
    pub totals: Totals,
    pub prog_judge_bpm: Option<ProgJudgeBpm>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct DamageValues {
    pub normal: u32,
    pub hard: u32,
    pub danger: u32,
    pub beam: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct Totals {
    pub notes: u32,
    pub tap: u32,
    pub hold: u32,
    pub side: u32,
    pub side_hold: u32,
    pub flick: u32,
    pub bell: u32,
}
