use bytes_kman::prelude::*;
use muzzman_lib::{prelude::LocationId, session::SessionError};

// send
#[derive(Clone, Debug, Bytes)]
pub enum ServerPackets {
    GetDefaultLocation,
    GetLocationName { from: LocationId },
    SetLocationName { from: LocationId, to: String },
    GetLocationDesc { from: LocationId },
    SetLocationDesc { from: LocationId, to: String },
    GetLocationInfo { from: LocationId },
}

// recv
#[derive(Clone, Debug, Bytes)]
pub enum ClientPackets {
    GetDefaultLocation(Result<LocationId, SessionError>),
    GetLocationName(Result<(LocationId, String), SessionError>),
    SetLocationName(Result<(LocationId, String), SessionError>),
    GetLocationDesc(Result<(LocationId, String), SessionError>),
    SetLocationDesc(Result<(LocationId, String), SessionError>),
}
