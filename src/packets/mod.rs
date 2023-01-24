use bytes_kman::prelude::*;
use muzzman_lib::{
    prelude::{LocationId, LocationInfo},
    session::SessionError,
};

// send
#[derive(Clone, Debug, Bytes)]
pub enum ServerPackets {
    GetDefaultLocation {
        id: u128,
    },
    GetLocationName {
        id: u128,
        from: LocationId,
    },
    SetLocationName {
        id: u128,
        from: LocationId,
        to: String,
    },
    GetLocationDesc {
        id: u128,
        from: LocationId,
    },
    SetLocationDesc {
        id: u128,
        from: LocationId,
        to: String,
    },
    GetLocationInfo {
        id: u128,
        from: LocationId,
    },
}

// recv
#[derive(Clone, Debug, Bytes)]
pub enum ClientPackets {
    GetDefaultLocation(u128, Result<LocationId, SessionError>),
    GetLocationName(u128, Result<String, SessionError>),
    SetLocationName(u128, Result<(), SessionError>),
    GetLocationDesc(u128, Result<String, SessionError>),
    SetLocationDesc(u128, Result<(), SessionError>),
    GetLocationInfo(u128, Result<LocationInfo, SessionError>),
}

impl ClientPackets {
    pub fn id(&self) -> u128 {
        match self {
            ClientPackets::GetDefaultLocation(id, _) => *id,
            ClientPackets::GetLocationName(id, _) => *id,
            ClientPackets::SetLocationName(id, _) => *id,
            ClientPackets::GetLocationDesc(id, _) => *id,
            ClientPackets::SetLocationDesc(id, _) => *id,
            ClientPackets::GetLocationInfo(id, _) => *id,
        }
    }
}
