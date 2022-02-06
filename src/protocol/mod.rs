//! Support for Nintendo Switch controller protocol.

use crate::{Error, ErrorKind, Result};
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

/// Enumeration for controller types.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ControllerType {
    /// Represents the Joy-Con (L).
    JoyConL,
    /// Represents the Joy-Con (R).
    JoyConR,
    /// Represents the Nintendo Switch Pro Controller.
    ProController,
}

impl ControllerType {
    /// Returns the Bluetooth controller name.
    pub fn name(&self) -> &str {
        match self {
            ControllerType::JoyConL => "Joy-Con (L)",
            ControllerType::JoyConR => "Joy-Con (R)",
            ControllerType::ProController => "Pro Controller",
        }
    }
}

impl Display for ControllerType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl FromStr for ControllerType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "JOY_CON_L" => Ok(ControllerType::JoyConL),
            "JOY_CON_R" => Ok(ControllerType::JoyConR),
            "PRO_CONTROLLER" => Ok(ControllerType::ProController),
            _ => Err(Error::new(
                ErrorKind::Protocol,
                "unknown controller type".into(),
            )),
        }
    }
}

/// Represents a Nintendo Switch controller protocol.
pub struct Protocol {
    controller_type: ControllerType,
}

impl Protocol {
    /// Creates a `Protocol` with the given controller type.
    pub fn new(controller_type: ControllerType) -> Self {
        Self { controller_type }
    }

    /// Returns the controller type.
    pub fn controller_type(&self) -> ControllerType {
        return self.controller_type;
    }

    /// Resets the state.
    pub fn reset(&mut self) {}
}
