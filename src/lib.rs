//! Emulate Nintendo Switch controllers over Bluetooth.

use log::{debug, info, warn, LevelFilter};
use std::fmt::{self, Display, Formatter};
use std::io;
use std::net::Shutdown;

pub mod bluetooth;
mod logger;
pub mod protocol;

use bluetooth::{
    Adapter, Address, AddressType, Profile, ProfileHandle, SeqPacket, SeqPacketListener,
    ServiceRecord, Session, SetClass, SocketAddr,
};
use logger::Logger;
use protocol::{ControllerType, Protocol};

/// Enumeration of error kinds.
#[derive(Debug)]
pub enum ErrorKind {
    /// Represents the Bluetooth error.
    Bluetooth(bluetooth::Error),
    /// Represents the IO error.
    Io(io::Error),
    /// Represents the protocol error.
    Protocol,
    /// Represents the other error.
    Other,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ErrorKind::Bluetooth(error) => write!(f, "{}", error),
            ErrorKind::Io(error) => write!(f, "{}", error),
            ErrorKind::Protocol => write!(f, "protocol"),
            ErrorKind::Other => write!(f, "other"),
        }
    }
}

/// Represents an error.
#[derive(Debug)]
pub struct Error {
    /// Represents the error kind.
    pub kind: ErrorKind,
    /// Represents the detailed message.
    pub message: String,
}

impl Error {
    /// Creates a `Error`.
    pub fn new(kind: ErrorKind, message: String) -> Self {
        Error { kind, message }
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Error::new(kind, String::new())
    }
}

impl From<bluetooth::Error> for Error {
    fn from(error: bluetooth::Error) -> Self {
        Error::new(ErrorKind::Bluetooth(error), String::new())
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::new(ErrorKind::Io(error), String::new())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.message.is_empty() {
            write!(f, "{}", &self.kind)
        } else {
            match self.kind {
                ErrorKind::Other => write!(f, "{}", &self.message),
                _ => write!(f, "{}: {}", &self.kind, &self.message),
            }
        }
    }
}

/// Represents an result.
pub type Result<T> = std::result::Result<T, Error>;

// Initializes the logger and set its level.
pub fn set_logger(verbose: usize) {
    let level = match verbose {
        0 => LevelFilter::Info,
        1 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };
    Logger::init(level);
}

/// Gets the list of Bluetooth adapters.
pub async fn adapters() -> Result<Vec<String>> {
    Ok(Session::new().await?.adapter_names().await?)
}

const NINTENDO_SWITCH_NAME: &str = "Nintendo Switch";
const GAMEPAD_JOYSITCK_COD: u32 = 0x002508;
const CTR_PSM: u16 = 17;
const ITR_PSM: u16 = 19;

const SERVICE: &str = "00001124-0000-1000-8000-00805f9b34fb";
const SERVICE_RECORD: &str = r#"<?xml version="1.0" encoding="UTF-8" ?>
<record>
    <attribute id="0x0001">
        <sequence>
            <uuid value="0x1124"/>
        </sequence>
    </attribute>
    <attribute id="0x0004">
        <sequence>
            <sequence>
                <uuid value="0x0100"/>
                <uint16 value="0x0011"/>
            </sequence>
            <sequence>
                <uuid value="0x0011"/>
            </sequence>
        </sequence>
    </attribute>
    <attribute id="0x0005">
        <sequence>
            <uuid value="0x1002"/>
        </sequence>
    </attribute>
    <attribute id="0x0006">
        <sequence>
            <uint16 value="0x656e"/>
            <uint16 value="0x006a"/>
            <uint16 value="0x0100"/>
        </sequence>
    </attribute>
    <attribute id="0x0009">
        <sequence>
            <sequence>
                <uuid value="0x1124"/>
                <uint16 value="0x0100"/>
            </sequence>
        </sequence>
    </attribute>
    <attribute id="0x000d">
        <sequence>
            <sequence>
                <sequence>
                    <uuid value="0x0100"/>
                    <uint16 value="0x0013"/>
                </sequence>
                <sequence>
                    <uuid value="0x0011"/>
                </sequence>
            </sequence>
        </sequence>
    </attribute>
    <attribute id="0x0100">
        <text value="Wireless Gamepad"/>
    </attribute>
    <attribute id="0x0101">
        <text value="Gamepad"/>
    </attribute>
    <attribute id="0x0102">
        <text value="Nintendo"/>
    </attribute>
    <attribute id="0x0200">
        <uint16 value="0x0100"/>
    </attribute>
    <attribute id="0x0201">
        <uint16 value="0x0111"/>
    </attribute>
    <attribute id="0x0202">
        <uint8 value="0x08"/>
    </attribute>
    <attribute id="0x0203">
        <uint8 value="0x00"/>
    </attribute>
    <attribute id="0x0204">
        <boolean value="true"/>
    </attribute>
    <attribute id="0x0205">
        <boolean value="true"/>
    </attribute>
    <attribute id="0x0206">
        <sequence>
            <sequence>
                <uint8 value="0x22"/>
                <text encoding="hex" value="050115000904a1018530050105091901290a150025017501950a5500650081020509190b290e150025017501950481027501950281030b01000100a1000b300001000b310001000b320001000b35000100150027ffff0000751095048102c00b39000100150025073500463b0165147504950181020509190f2912150025017501950481027508953481030600ff852109017508953f8103858109027508953f8103850109037508953f9183851009047508953f9183858009057508953f9183858209067508953f9183c0"/>
            </sequence>
        </sequence>
    </attribute>
    <attribute id="0x0207">
        <sequence>
            <sequence>
                <uint16 value="0x0409"/>
                <uint16 value="0x0100"/>
            </sequence>
        </sequence>
    </attribute>
    <attribute id="0x020b">
        <uint16 value="0x0100"/>
    </attribute>
    <attribute id="0x020c">
        <uint16 value="0x0c80"/>
    </attribute>
    <attribute id="0x020d">
        <boolean value="false"/>
    </attribute>
    <attribute id="0x020e">
        <boolean value="true"/>
    </attribute>
    <attribute id="0x020f">
        <uint16 value="0x0640"/>
    </attribute>
    <attribute id="0x0210">
        <uint16 value="0x0320"/>
    </attribute>
</record>
"#;

/// Represents an emulated Nintendo Switch controller.
pub struct Controller {
    session: Session,
    adapter: Adapter,
    protocol: Protocol,
    handle: Option<ProfileHandle>,
    ctr_seq_packet: Option<SeqPacket>,
    itr_seq_packet: Option<SeqPacket>,
}

impl Controller {
    /// Creates a `Controller` with the given adapter and controller type.
    pub async fn new(adapter: &str, controller_type: ControllerType) -> Result<Self> {
        let session = Session::new().await?;
        let adapter = session.adapter(adapter)?;

        Ok(Controller {
            session,
            adapter,
            protocol: Protocol::new(controller_type),
            handle: None,
            ctr_seq_packet: None,
            itr_seq_packet: None,
        })
    }

    /// Pairs a new device.
    pub async fn pair(&mut self) -> Result<Address> {
        // Check active service records
        if let Some(uuids) = self.adapter.uuids().await? {
            if uuids.len() > 3 {
                warn!("Too many service records active");
            }
        }

        // Unpair paired Nintendo Switches
        for device_addr in self.adapter.device_addresses().await?.into_iter() {
            if let Some(name) = self.adapter.device(device_addr)?.name().await? {
                if name == NINTENDO_SWITCH_NAME {
                    warn!("Unpair previous device {}", device_addr);
                    self.adapter.remove_device(device_addr).await?;
                }
            };
        }

        self.protocol.reset();

        // Listeners
        let addr = self.adapter.address().await?;
        let ctr_listener =
            SeqPacketListener::bind(SocketAddr::new(addr, AddressType::BrEdr, CTR_PSM)).await?;
        let itr_listener =
            SeqPacketListener::bind(SocketAddr::new(addr, AddressType::BrEdr, ITR_PSM)).await?;

        self.adapter.set_powered(true).await?;
        self.adapter.set_pairable(true).await?;
        self.adapter
            .set_alias(self.protocol.controller_type().name().into())
            .await?;

        // Register service record
        self.handle = Some(
            self.session
                .register_profile(Profile::new_service_record(
                    SERVICE.parse().unwrap(),
                    SERVICE_RECORD.into(),
                ))
                .await?,
        );

        self.adapter.set_discoverable(true).await?;
        self.adapter.set_class(GAMEPAD_JOYSITCK_COD)?;
        if self.adapter.class().await? != GAMEPAD_JOYSITCK_COD {
            return Err(Error::new(
                ErrorKind::Other,
                format!("cannot set class for adapter {}", self.adapter.name()).into(),
            ));
        }

        // Accept
        info!("Wait for device to connect");
        let (ctr_seq_packet, ctr_addr) = ctr_listener.accept().await?;
        debug!("accept {}, PSM = {} (CTR)", ctr_addr.addr, ctr_addr.psm);
        let (itr_seq_packet, itr_addr) = itr_listener.accept().await?;
        debug!("accept {}, PSM = {} (ITR)", itr_addr.addr, itr_addr.psm);
        assert!(ctr_addr.addr == itr_addr.addr);
        self.ctr_seq_packet = Some(ctr_seq_packet);
        self.itr_seq_packet = Some(itr_seq_packet);

        self.adapter.set_discoverable(false).await?;
        self.adapter.set_pairable(false).await?;

        Ok(itr_addr.addr)
    }

    fn close(&mut self) {
        // Close transportation
        if let Some(itr_seq_packet) = &self.itr_seq_packet {
            if let Err(e) = itr_seq_packet.shutdown(Shutdown::Both) {
                warn!("{}", e);
            }
            self.itr_seq_packet.take();
        }
        if let Some(ctr_seq_packet) = &self.ctr_seq_packet {
            if let Err(e) = ctr_seq_packet.shutdown(Shutdown::Both) {
                warn!("{}", e);
            }
            self.ctr_seq_packet.take();
        }

        // Unregister service record
        self.handle.take();
    }
}

impl Drop for Controller {
    fn drop(&mut self) {
        self.close();
    }
}
