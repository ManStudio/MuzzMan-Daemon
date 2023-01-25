use std::{ops::Range, path::PathBuf};

use bytes_kman::prelude::*;
use muzzman_lib::{
    prelude::{ElementId, ElementInfo, LocationId, LocationInfo, ModuleId, SessionEvent, Value},
    session::{Actions, SessionError},
    types::Type,
};

// send
#[derive(Clone, Debug, Bytes)]
pub enum ServerPackets {
    LoadModule {
        id: u128,
        path: PathBuf,
    },
    RemoveModule {
        id: u128,
        module_id: ModuleId,
    },

    GetActionsLen {
        id: u128,
    },
    GetActions {
        id: u128,
        range: Range<usize>,
    },
    RunAction {
        id: u128,
        module_id: ModuleId,
        name: String,
        data: Vec<Type>,
    },

    GetDefaultLocation {
        id: u128,
    },
    LocationGetName {
        id: u128,
        from: LocationId,
    },
    LocationSetName {
        id: u128,
        from: LocationId,
        to: String,
    },
    LocationGetDesc {
        id: u128,
        from: LocationId,
    },
    LocationSetDesc {
        id: u128,
        from: LocationId,
        to: String,
    },
    LocationGetInfo {
        id: u128,
        from: LocationId,
    },

    CreateElement {
        id: u128,
        location_id: LocationId,
        name: String,
    },
    ElementGetName {
        id: u128,
        element_id: ElementId,
    },
    ElementSetName {
        id: u128,
        element_id: ElementId,
        to: String,
    },
    ElementGetDesc {
        id: u128,
        element_id: ElementId,
    },
    ElementSetDesc {
        id: u128,
        element_id: ElementId,
        to: String,
    },
    ElementGetMeta {
        id: u128,
        element_id: ElementId,
    },
    ElementSetMeta {
        id: u128,
        element_id: ElementId,
        to: String,
    },
    ElementGetInfo {
        id: u128,
        element_id: ElementId,
    },

    Tick,
}

// recv
#[derive(Clone, Debug, Bytes)]
pub enum ClientPackets {
    LoadModule(u128, Result<ModuleId, SessionError>),
    RemoveModule(u128, Result<(), SessionError>),

    GetActionsLen(u128, Result<usize, SessionError>),
    GetActions(
        u128,
        Result<Vec<(String, ModuleId, Vec<(String, Value)>)>, SessionError>,
    ),
    RunAction(u128, Result<(), SessionError>),

    GetDefaultLocation(u128, Result<LocationId, SessionError>),
    LocationGetName(u128, Result<String, SessionError>),
    LocationSetName(u128, Result<(), SessionError>),
    LocationGetDesc(u128, Result<String, SessionError>),
    LocationSetDesc(u128, Result<(), SessionError>),
    LocationGetInfo(u128, Result<LocationInfo, SessionError>),

    CreateElement(u128, Result<ElementId, SessionError>),
    ElementGetName(u128, Result<String, SessionError>),
    ElementSetName(u128, Result<(), SessionError>),
    ElementGetDesc(u128, Result<String, SessionError>),
    ElementSetDesc(u128, Result<(), SessionError>),
    ElementGetMeta(u128, Result<String, SessionError>),
    ElementSetMeta(u128, Result<(), SessionError>),
    ElementGetInfo(u128, Result<ElementInfo, SessionError>),

    NewSessionEvent(SessionEvent),
}

impl ClientPackets {
    pub fn id(&self) -> u128 {
        match self {
            ClientPackets::GetDefaultLocation(id, _) => *id,
            ClientPackets::LocationGetName(id, _) => *id,
            ClientPackets::LocationSetName(id, _) => *id,
            ClientPackets::LocationGetDesc(id, _) => *id,
            ClientPackets::LocationSetDesc(id, _) => *id,
            ClientPackets::LocationGetInfo(id, _) => *id,
            ClientPackets::CreateElement(id, _) => *id,
            ClientPackets::ElementGetName(id, _) => *id,
            ClientPackets::ElementSetName(id, _) => *id,
            ClientPackets::ElementGetDesc(id, _) => *id,
            ClientPackets::ElementSetDesc(id, _) => *id,
            ClientPackets::ElementGetMeta(id, _) => *id,
            ClientPackets::ElementSetMeta(id, _) => *id,
            ClientPackets::ElementGetInfo(id, _) => *id,
            ClientPackets::LoadModule(id, _) => *id,
            ClientPackets::RemoveModule(id, _) => *id,
            ClientPackets::GetActionsLen(id, _) => *id,
            ClientPackets::GetActions(id, _) => *id,
            ClientPackets::RunAction(id, _) => *id,
            ClientPackets::NewSessionEvent(_) => 0,
        }
    }
}
