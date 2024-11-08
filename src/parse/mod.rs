pub mod analysis;
pub mod raw;

use thiserror::Error;

use crate::lex::{
    command::*,
    token::{Token, TokenStream},
};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Error)]
pub enum ParseError {
    #[error("syntax error: {0}")]
    SyntaxError(String),
    #[error("semantic error: {0}")]
    SemanticError(String),
    #[error("semantic error, expected more commands: {0}")]
    SemanticErrorExpectedCommand(String),
}

pub type Result<T> = std::result::Result<T, ParseError>;

/// XXX TODO: Have a proper parsed version of this where the u32 bits are properly converted to
/// float.
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
