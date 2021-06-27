use thiserror::Error;

pub mod effect;
pub mod flare;
pub mod ghost;
pub mod shader_lib;

#[derive(Error, Debug)]
pub enum LfgError {
    #[error("Invalid value for parameter {0}")]
    InvalidEffectValue(String),
}
