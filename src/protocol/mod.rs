//! Support for Nintendo Switch controller protocol.

use crate::{Error, ErrorKind, Result};
use std::fmt::{self, Display, Formatter};

/// Enumeration for direction.
#[repr(u8)]
pub enum Direction {
    /// Represents the input (from controller to device) direction.
    Input = 0xA1,
    /// Represents the output (from device to controller) direction.
    Output = 0xA2,
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Direction::Input => write!(f, "Input"),
            Direction::Output => write!(f, "Output"),
        }
    }
}

/// Enumeration for types.
#[repr(u8)]
pub enum Type {
    /// Represents the subcommand.
    Subcommand = 0x01,
    /// Represents the rumble.
    Rumble = 0x10,
    /// Represents the request of IR, NFC, or MCU data.
    RequestIrNfcMcu = 0x11,
}

impl TryFrom<u8> for Type {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self> {
        match value {
            0x01 => Ok(Type::Subcommand),
            0x10 => Ok(Type::Rumble),
            0x11 => Ok(Type::RequestIrNfcMcu),
            _ => Err(Error::new(
                ErrorKind::Protocol,
                "invalid output type".into(),
            )),
        }
    }
}

/// Enumeration for subcommands,
pub enum Subcommand {}

pub struct Output {
    direction: Direction,
    t: Type,
    timer: u8,
    left_rumble: u32,
    right_rumble: u32,
    subcommand: Option<u8>,
    data: Option<Vec<u8>>,
}

impl TryFrom<&[u8]> for Output {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self> {
        if value.len() < 12 {
            return Err(Error::new(
                ErrorKind::Protocol,
                "invalid output length".into(),
            ));
        }

        // Direction
        if value[0] != Direction::Output as u8 {
            return Err(Error::new(
                ErrorKind::Protocol,
                "invalid output direction".into(),
            ));
        }

        // Type
        let t: Type = value[1].try_into()?;

        // Timer
        let timer = value[2];

        todo!();
    }
}
