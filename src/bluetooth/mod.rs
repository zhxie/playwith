//! Support for handling Bluetooth devices.

pub use bluer::l2cap::{SeqPacket, SeqPacketListener, SocketAddr};
use bluer::rfcomm::Role;
pub use bluer::rfcomm::{Profile, ProfileHandle};
pub use bluer::{Adapter, Address, AddressType, Error, Session, Uuid};
use std::io;
use std::process::Command;

/// Trait for setting Bluetooth adapter's class.
pub trait SetClass {
    // Sets the class.
    fn set_class(&self, class: u32) -> io::Result<()>;
}

impl SetClass for Adapter {
    fn set_class(&self, class: u32) -> io::Result<()> {
        match Command::new("hciconfig")
            .arg(self.name())
            .arg("class")
            .arg(format!("{}", class))
            .status()
        {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}

/// Trait for Bluetooth service record.
pub trait ServiceRecord {
    /// Creates a `Profile` which represents a service record.
    fn new_service_record(service: Uuid, service_record: String) -> Profile;
}

impl ServiceRecord for Profile {
    fn new_service_record(service: Uuid, service_record: String) -> Self {
        Profile {
            uuid: Uuid::new_v4(),
            name: None,
            service: Some(service),
            role: Some(Role::Server),
            channel: None,
            psm: None,
            require_authentication: Some(true),
            require_authorization: Some(true),
            auto_connect: None,
            service_record: Some(service_record),
            version: None,
            features: None,
            _non_exhaustive: (),
        }
    }
}
