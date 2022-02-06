//! Support for handling Bluetooth devices.

use crate::{Error, ErrorKind, Result};
pub use bluer::l2cap::{SeqPacketListener, SocketAddr};
use bluer::rfcomm::Role;
pub use bluer::rfcomm::{Profile, ProfileHandle};
use bluer::Address;
pub use bluer::Uuid;
use std::collections::HashSet;
use std::process::Command;

/// Represents a Bluetooth session.
pub struct Session {
    session: bluer::Session,
}

impl Session {
    /// Creates a `Session`.
    pub async fn new() -> Result<Self> {
        let session = match bluer::Session::new().await {
            Ok(session) => session,
            Err(_) => {
                return Err(Error::new(
                    ErrorKind::Bluetooth,
                    "cannot create session".into(),
                ))
            }
        };

        Ok(Session { session })
    }

    /// Creates an interface to the Bluetooth adapter with the given name.
    pub fn adapter(&self, name: &str) -> Result<Adapter> {
        match self.session.adapter(name) {
            Ok(adapter) => Ok(Adapter::new(adapter)),
            Err(_) => Err(Error::new(
                ErrorKind::Bluetooth,
                format!("cannot get adapter {}", name),
            )),
        }
    }

    /// Enumerates Bluetooth adapters and returns their names.
    pub async fn adapter_names(&self) -> Result<Vec<String>> {
        match self.session.adapter_names().await {
            Ok(adapter_names) => Ok(adapter_names),
            Err(_) => Err(Error::new(
                ErrorKind::Bluetooth,
                "cannot get adapter names".into(),
            )),
        }
    }

    /// Registers a Bluetooth RFCOMM profile and returns its handle.
    pub async fn register_profile(&self, profile: Profile) -> Result<ProfileHandle> {
        match self.session.register_profile(profile.into()).await {
            Ok(handle) => Ok(handle),
            Err(_) => Err(Error::new(
                ErrorKind::Bluetooth,
                "cannot register profile".into(),
            )),
        }
    }
}

/// Represents a Bluetooth adapter.
pub struct Adapter {
    adapter: bluer::Adapter,
}

impl Adapter {
    /// Creates a `Adapter`.
    pub fn new(adapter: bluer::Adapter) -> Self {
        Adapter { adapter }
    }

    /// Returns the address.
    pub async fn address(&self) -> Result<Address> {
        match self.adapter.address().await {
            Ok(address) => Ok(address),
            Err(_) => Err(Error::new(
                ErrorKind::Bluetooth,
                format!("cannot get address of adapter {}", self.adapter.name()),
            )),
        }
    }

    /// Returns the alias.
    pub async fn alias(&self) -> Result<String> {
        match self.adapter.alias().await {
            Ok(alias) => Ok(alias),
            Err(_) => Err(Error::new(
                ErrorKind::Bluetooth,
                format!("cannot get alias of adapter {}", self.adapter.name()),
            )),
        }
    }

    /// Returns the class.
    pub async fn class(&self) -> Result<u32> {
        match self.adapter.class().await {
            Ok(class) => Ok(class),
            Err(_) => Err(Error::new(
                ErrorKind::Bluetooth,
                format!("cannot get class of adapter {}", self.adapter.name()),
            )),
        }
    }

    /// Returns if the adapter is discoverable or not.
    pub async fn discoverable(&self) -> Result<bool> {
        match self.adapter.is_discoverable().await {
            Ok(discoverable) => Ok(discoverable),
            Err(_) => Err(Error::new(
                ErrorKind::Bluetooth,
                format!("cannot get discoverable of adapter {}", self.adapter.name(),),
            )),
        }
    }

    /// Returns the name.
    pub fn name(&self) -> &str {
        self.adapter.name()
    }

    /// Returns if the adapter is pairable or not.
    pub async fn pairable(&self) -> Result<bool> {
        match self.adapter.is_pairable().await {
            Ok(pairable) => Ok(pairable),
            Err(_) => Err(Error::new(
                ErrorKind::Bluetooth,
                format!("cannot get pairable of adapter {}", self.adapter.name()),
            )),
        }
    }

    /// Returns if the adapter is powered on or not.
    pub async fn powered(&self) -> Result<bool> {
        match self.adapter.is_powered().await {
            Ok(powered) => Ok(powered),
            Err(_) => Err(Error::new(
                ErrorKind::Bluetooth,
                format!("cannot get powered of adapter {}", self.adapter.name()),
            )),
        }
    }

    /// Returns the UUIDs.
    pub async fn uuids(&self) -> Result<HashSet<Uuid>> {
        match self.adapter.uuids().await {
            Ok(uuids) => match uuids {
                Some(uuids) => Ok(uuids),
                None => Ok(HashSet::new()),
            },
            Err(_) => Err(Error::new(
                ErrorKind::Bluetooth,
                format!("cannot get uuid of adapter {}", self.adapter.name()),
            )),
        }
    }

    // Sets the alias.
    pub async fn set_alias(&mut self, alias: &str) -> Result<()> {
        match self.adapter.set_alias(alias.to_string()).await {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::new(
                ErrorKind::Bluetooth,
                format!("cannot set adapter {} alias to {}", self.name(), alias),
            )),
        }
    }

    // Sets the class.
    pub async fn set_class(&mut self, class: u32) -> Result<()> {
        if let Err(_) = Command::new("hciconfig")
            .arg(self.name())
            .arg("class")
            .arg(format!("{}", class))
            .status()
        {
            return Err(Error::new(
                ErrorKind::Bluetooth,
                format!("cannot set adapter {} class to {}", self.name(), class),
            ));
        }

        if class != self.class().await? {
            return Err(Error::new(
                ErrorKind::Bluetooth,
                format!("cannot set adapter {} class to {}", self.name(), class),
            ));
        }

        Ok(())
    }

    /// Sets the adapter to discoverable or not.
    pub async fn set_discoverable(&mut self, discoverable: bool) -> Result<()> {
        match self.adapter.set_discoverable(discoverable).await {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::new(
                ErrorKind::Bluetooth,
                format!(
                    "cannot set adapter {} discoverable to {}",
                    self.name(),
                    discoverable
                ),
            )),
        }
    }

    /// Sets the adapter to pairable or not.
    pub async fn set_pairable(&mut self, pairable: bool) -> Result<()> {
        match self.adapter.set_pairable(pairable).await {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::new(
                ErrorKind::Bluetooth,
                format!(
                    "cannot set adapter {} pairable to {}",
                    self.name(),
                    pairable
                ),
            )),
        }
    }

    /// Sets the adapter powered on or off.
    pub async fn set_powered(&mut self, powered: bool) -> Result<()> {
        match self.adapter.set_powered(powered).await {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::new(
                ErrorKind::Bluetooth,
                format!("cannot set adapter {} powered to {}", self.name(), powered),
            )),
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
