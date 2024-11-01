use crate::lex::command::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct Composition {
    pub bpm_first: u32,
    pub bpm_changes: Vec<BpmChange>,
    pub meter_first: MeterDefinition,
    pub meter_changes: Vec<MeterChange>,
    pub soflans: Vec<Soflan>,
}
