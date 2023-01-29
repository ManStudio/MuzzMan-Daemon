use std::{ops::Range, path::PathBuf};

use bytes_kman::prelude::*;
use muzzman_lib::{
    prelude::{
        Data, ElementId, ElementInfo, FileOrData, LocationId, LocationInfo, ModuleId, SessionEvent,
        Value,
    },
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

    GetModulesLen {
        id: u128,
    },
    GetModules {
        id: u128,
        range: Range<usize>,
    },
    ModuleGetName {
        id: u128,
        module_id: ModuleId,
    },
    ModuleSetName {
        id: u128,
        module_id: ModuleId,
        to: String,
    },
    ModuleGetDefaultName {
        id: u128,
        module_id: ModuleId,
    },
    ModuleGetDesc {
        id: u128,
        module_id: ModuleId,
    },
    ModuleSetDesc {
        id: u128,
        module_id: ModuleId,
        to: String,
    },
    ModuleGetDefaultDesc {
        id: u128,
        module_id: ModuleId,
    },
    ModuleGetProxy {
        id: u128,
        module_id: ModuleId,
    },
    ModuleSetProxy {
        id: u128,
        module_id: ModuleId,
        to: usize,
    },
    ModuleGetSettings {
        id: u128,
        module_id: ModuleId,
    },
    ModuleSetSettings {
        id: u128,
        module_id: ModuleId,
        to: Data,
    },

    ModuleGetElementSettings {
        id: u128,
        module_id: ModuleId,
    },
    ModuleSetElementSettings {
        id: u128,
        module_id: ModuleId,
        to: Data,
    },
    ModuleInitLocation {
        id: u128,
        module_id: ModuleId,
        location_id: LocationId,
        data: FileOrData,
    },
    ModuleInitElement {
        id: u128,
        module_id: ModuleId,
        element_id: ElementId,
    },
    ModuleAcceptUrl {
        id: u128,
        module_id: ModuleId,
        url: String,
    },
    ModuleAcceptExtension {
        id: u128,
        module_id: ModuleId,
        filename: String,
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

    GetModulesLen(u128, Result<usize, SessionError>),
    GetModules(u128, Result<Vec<ModuleId>, SessionError>),
    ModuleGetName(u128, Result<String, SessionError>),
    ModuleSetName(u128, Result<(), SessionError>),
    ModuleGetDefaultName(u128, Result<String, SessionError>),
    ModuleGetDesc(u128, Result<String, SessionError>),
    ModuleSetDesc(u128, Result<(), SessionError>),
    ModuleGetDefaultDesc(u128, Result<String, SessionError>),
    ModuleGetProxy(u128, Result<usize, SessionError>),
    ModuleSetProxy(u128, Result<(), SessionError>),
    ModuleGetSettings(u128, Result<Data, SessionError>),
    ModuleSetSettings(u128, Result<(), SessionError>),
    ModuleGetElementSettings(u128, Result<Data, SessionError>),
    ModuleSetElementSettings(u128, Result<(), SessionError>),
    ModuleInitLocation(u128, Result<(), SessionError>),
    ModuleInitElement(u128, Result<(), SessionError>),
    ModuleAcceptUrl(u128, Result<bool, SessionError>),
    ModuleAcceptExtension(u128, Result<bool, SessionError>),

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
            ClientPackets::GetModulesLen(id, _) => *id,
            ClientPackets::GetModules(id, _) => *id,
            ClientPackets::ModuleGetName(id, _) => *id,
            ClientPackets::ModuleSetName(id, _) => *id,
            ClientPackets::ModuleGetDefaultName(id, _) => *id,
            ClientPackets::ModuleGetDesc(id, _) => *id,
            ClientPackets::ModuleSetDesc(id, _) => *id,
            ClientPackets::ModuleGetDefaultDesc(id, _) => *id,
            ClientPackets::ModuleGetProxy(id, _) => *id,
            ClientPackets::ModuleSetProxy(id, _) => *id,
            ClientPackets::ModuleGetSettings(id, _) => *id,
            ClientPackets::ModuleSetSettings(id, _) => *id,
            ClientPackets::ModuleGetElementSettings(id, _) => *id,
            ClientPackets::ModuleSetElementSettings(id, _) => *id,
            ClientPackets::ModuleInitLocation(id, _) => *id,
            ClientPackets::ModuleInitElement(id, _) => *id,
            ClientPackets::ModuleAcceptUrl(id, _) => *id,
            ClientPackets::ModuleAcceptExtension(id, _) => *id,
            ClientPackets::NewSessionEvent(_) => 0,
        }
    }
}
