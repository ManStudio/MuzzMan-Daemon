use std::{ops::Range, path::PathBuf};

use bytes_kman::prelude::*;
use muzzman_lib::{
    prelude::{
        Data, ElementId, ElementInfo, Event, FileOrData, LocationId, LocationInfo, ModuleId,
        ModuleInfo, SessionEvent, Value,
    },
    session::SessionError,
    types::{MRef, Type, ID, UID},
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
    LoadModuleInfo {
        id: u128,
        module_info: ModuleInfo,
    },
    FindModule {
        id: u128,
        module_info: ModuleInfo,
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

    ModuleGetUid {
        id: u128,
        module_id: ModuleId,
    },

    ModuleGetVersion {
        id: u128,
        module_id: ModuleId,
    },

    ModuleSupportedVersions {
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
    ModuleAcceptedProtocols {
        id: u128,
        module_id: ModuleId,
    },
    ModuleAcceptedExtensions {
        id: u128,
        module_id: ModuleId,
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
    MoveElement {
        id: u128,
        element_id: ElementId,
        location_id: LocationId,
    },
    DestroyElement {
        id: u128,
        element_id: ElementId,
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
    ElementGetUrl {
        id: u128,
        element_id: ElementId,
    },
    ElementSetUrl {
        id: u128,
        element_id: ElementId,
        to: Option<String>,
    },

    ElementGetElementData {
        id: u128,
        element_id: ElementId,
    },
    ElementSetElementData {
        id: u128,
        element_id: ElementId,
        to: Data,
    },
    ElementGetModuleData {
        id: u128,
        element_id: ElementId,
    },
    ElementSetModuleData {
        id: u128,
        element_id: ElementId,
        to: Data,
    },
    ElementGetModule {
        id: u128,
        element_id: ElementId,
    },
    ElementSetModule {
        id: u128,
        element_id: ElementId,
        module: Option<ModuleId>,
    },
    ElementGetStatuses {
        id: u128,
        element_id: ElementId,
    },
    ElementSetStatuses {
        id: u128,
        element_id: ElementId,
        to: Vec<String>,
    },
    ElementGetStatus {
        id: u128,
        element_id: ElementId,
    },
    ElementSetStatus {
        id: u128,
        element_id: ElementId,
        to: usize,
    },
    ElementGetData {
        id: u128,
        element_id: ElementId,
    },
    ElementSetData {
        id: u128,
        element_id: ElementId,
        to: FileOrData,
    },
    ElementGetProgress {
        id: u128,
        element_id: ElementId,
    },
    ElementSetProgress {
        id: u128,
        element_id: ElementId,
        to: f32,
    },
    ElementGetShouldSave {
        id: u128,
        element_id: ElementId,
    },
    ElementSetShouldSave {
        id: u128,
        element_id: ElementId,
        to: bool,
    },
    ElementGetEnabled {
        id: u128,
        element_id: ElementId,
    },
    ElementSetEnabled {
        id: u128,
        element_id: ElementId,
        to: bool,
    },
    ElementResolvModule {
        id: u128,
        element_id: ElementId,
    },
    ElementWait {
        id: u128,
        element_id: ElementId,
    },
    ElementGetInfo {
        id: u128,
        element_id: ElementId,
    },
    ElementNotify {
        id: u128,
        element_id: ElementId,
        event: Event,
    },
    ElementEmit {
        id: u128,
        element_id: ElementId,
        event: Event,
    },
    ElementSubscribe {
        id: u128,
        element_id: ElementId,
        to: ID,
    },
    ElementUnSubscribe {
        id: u128,
        element_id: ElementId,
        to: ID,
    },

    CreateLocation {
        id: u128,
        name: String,
        location_id: LocationId,
    },
    GetLocationsLen {
        id: u128,
        location_id: LocationId,
    },
    GetLocations {
        id: u128,
        location_id: LocationId,
        range: Range<usize>,
    },
    DestroyLocation {
        id: u128,
        location_id: LocationId,
    },
    MoveLocation {
        id: u128,
        location_id: LocationId,
        to: LocationId,
    },
    LocationGetPath {
        id: u128,
        location_id: LocationId,
    },
    LocationSetPath {
        id: u128,
        location_id: LocationId,
        to: PathBuf,
    },
    LocationGetShouldSave {
        id: u128,
        location_id: LocationId,
    },
    LocationSetShouldSave {
        id: u128,
        location_id: LocationId,
        to: bool,
    },
    LocationGetElementsLen {
        id: u128,
        location_id: LocationId,
    },
    LocationGetElements {
        id: u128,
        location_id: LocationId,
        range: Range<usize>,
    },
    LocationNotify {
        id: u128,
        location_id: LocationId,
        event: Event,
    },
    LocationEmit {
        id: u128,
        location_id: LocationId,
        event: Event,
    },
    LocationSubscribe {
        id: u128,
        location_id: LocationId,
        to: ID,
    },
    LocationUnSubscribe {
        id: u128,
        location_id: LocationId,
        to: ID,
    },

    Tick,
}

pub type Actions = Vec<(String, ModuleId, Vec<(String, Value)>)>;

// recv
#[derive(Clone, Debug, Bytes)]
pub enum ClientPackets {
    LoadModule(u128, Result<ModuleId, SessionError>),
    RemoveModule(u128, Result<(), SessionError>),
    LoadModuleInfo(u128, Result<ModuleId, SessionError>),
    FindModule(u128, Result<ModuleId, SessionError>),

    GetActionsLen(u128, Result<usize, SessionError>),
    GetActions(u128, Result<Actions, SessionError>),
    RunAction(u128, Result<(), SessionError>),

    GetModulesLen(u128, Result<usize, SessionError>),
    GetModules(u128, Result<Vec<ModuleId>, SessionError>),
    ModuleGetName(u128, Result<String, SessionError>),
    ModuleSetName(u128, Result<(), SessionError>),
    ModuleGetDefaultName(u128, Result<String, SessionError>),
    ModuleGetUid(u128, Result<UID, SessionError>),
    ModuleGetVersion(u128, Result<String, SessionError>),
    ModuleSupportedVersions(u128, Result<std::ops::Range<u64>, SessionError>),
    ModuleGetDesc(u128, Result<String, SessionError>),
    ModuleSetDesc(u128, Result<(), SessionError>),
    ModuleGetDefaultDesc(u128, Result<String, SessionError>),
    ModuleGetProxy(u128, Result<usize, SessionError>),
    ModuleSetProxy(u128, Result<(), SessionError>),
    ModuleGetSettings(u128, Box<Result<Data, SessionError>>),
    ModuleSetSettings(u128, Result<(), SessionError>),
    ModuleGetElementSettings(u128, Result<Data, SessionError>),
    ModuleSetElementSettings(u128, Result<(), SessionError>),
    ModuleInitLocation(u128, Result<(), SessionError>),
    ModuleInitElement(u128, Result<(), SessionError>),
    ModuleAcceptUrl(u128, Result<bool, SessionError>),
    ModuleAcceptExtension(u128, Result<bool, SessionError>),
    ModuleAcceptedProtocols(u128, Result<Vec<String>, SessionError>),
    ModuleAcceptedExtensions(u128, Result<Vec<String>, SessionError>),

    CreateLocation(u128, Result<LocationId, SessionError>),
    GetLocationsLen(u128, Result<usize, SessionError>),
    GetLocations(u128, Result<Vec<LocationId>, SessionError>),
    DestroyLocation(u128, Result<(), SessionError>),
    MoveLocation(u128, Result<(), SessionError>),
    GetDefaultLocation(u128, Result<LocationId, SessionError>),
    LocationGetName(u128, Result<String, SessionError>),
    LocationSetName(u128, Result<(), SessionError>),
    LocationGetDesc(u128, Result<String, SessionError>),
    LocationSetDesc(u128, Result<(), SessionError>),
    LocationGetInfo(u128, Result<LocationInfo, SessionError>),
    LocationGetPath(u128, Result<PathBuf, SessionError>),
    LocationSetPath(u128, Result<(), SessionError>),
    LocationGetShouldSave(u128, Result<bool, SessionError>),
    LocationSetShouldSave(u128, Result<(), SessionError>),
    LocationGetElementsLen(u128, Result<usize, SessionError>),
    LocationGetElements(u128, Result<Vec<ElementId>, SessionError>),
    LocationNotify(u128, Result<(), SessionError>),
    LocationEmit(u128, Result<(), SessionError>),
    LocationSubscribe(u128, Result<(), SessionError>),
    LocationUnSubscribe(u128, Result<(), SessionError>),

    CreateElement(u128, Result<ElementId, SessionError>),
    MoveElement(u128, Result<(), SessionError>),
    DestroyElement(u128, Result<(), SessionError>),
    ElementGetName(u128, Result<String, SessionError>),
    ElementSetName(u128, Result<(), SessionError>),
    ElementGetDesc(u128, Result<String, SessionError>),
    ElementSetDesc(u128, Result<(), SessionError>),
    ElementGetMeta(u128, Result<String, SessionError>),
    ElementSetMeta(u128, Result<(), SessionError>),
    ElementGetUrl(u128, Result<Option<String>, SessionError>),
    ElementSetUrl(u128, Result<(), SessionError>),
    ElementGetElementData(u128, Result<Data, SessionError>),
    ElementSetElementData(u128, Result<(), SessionError>),
    ElementGetModuleData(u128, Result<Data, SessionError>),
    ElementSetModuleData(u128, Result<(), SessionError>),
    ElementGetModule(u128, Result<Option<ModuleId>, SessionError>),
    ElementSetModule(u128, Result<(), SessionError>),
    ElementGetStatuses(u128, Result<Vec<String>, SessionError>),
    ElementSetStatuses(u128, Result<(), SessionError>),
    ElementGetStatus(u128, Result<usize, SessionError>),
    ElementSetStatus(u128, Result<(), SessionError>),
    ElementGetData(u128, Result<FileOrData, SessionError>),
    ElementSetData(u128, Result<(), SessionError>),
    ElementGetProgress(u128, Result<f32, SessionError>),
    ElementSetProgress(u128, Result<(), SessionError>),
    ElementGetShouldSave(u128, Result<bool, SessionError>),
    ElementSetShouldSave(u128, Result<(), SessionError>),
    ElementGetEnabled(u128, Result<bool, SessionError>),
    ElementSetEnabled(u128, Result<(), SessionError>),
    ElementResolvModule(u128, Result<bool, SessionError>),
    ElementWait(u128, Result<(), SessionError>),
    ElementGetInfo(u128, Box<Result<ElementInfo, SessionError>>),
    ElementNotify(u128, Result<(), SessionError>),
    ElementEmit(u128, Result<(), SessionError>),
    ElementSubscribe(u128, Result<(), SessionError>),
    ElementUnSubscribe(u128, Result<(), SessionError>),

    NewSessionEvent(SessionEvent),
}

impl ClientPackets {
    pub fn id(&self) -> u128 {
        match self {
            ClientPackets::NewSessionEvent(_) => 0,
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
            ClientPackets::MoveElement(id, _) => *id,
            ClientPackets::DestroyElement(id, _) => *id,
            ClientPackets::ElementGetElementData(id, _) => *id,
            ClientPackets::ElementSetElementData(id, _) => *id,
            ClientPackets::ElementGetModuleData(id, _) => *id,
            ClientPackets::ElementSetModuleData(id, _) => *id,
            ClientPackets::ElementGetModule(id, _) => *id,
            ClientPackets::ElementSetModule(id, _) => *id,
            ClientPackets::ElementGetStatuses(id, _) => *id,
            ClientPackets::ElementSetStatuses(id, _) => *id,
            ClientPackets::ElementGetStatus(id, _) => *id,
            ClientPackets::ElementSetStatus(id, _) => *id,
            ClientPackets::ElementGetData(id, _) => *id,
            ClientPackets::ElementSetData(id, _) => *id,
            ClientPackets::ElementGetProgress(id, _) => *id,
            ClientPackets::ElementSetProgress(id, _) => *id,
            ClientPackets::ElementGetShouldSave(id, _) => *id,
            ClientPackets::ElementSetShouldSave(id, _) => *id,
            ClientPackets::ElementSetEnabled(id, _) => *id,
            ClientPackets::ElementGetEnabled(id, _) => *id,
            ClientPackets::ElementResolvModule(id, _) => *id,
            ClientPackets::ElementWait(id, _) => *id,
            ClientPackets::ElementNotify(id, _) => *id,
            ClientPackets::ElementEmit(id, _) => *id,
            ClientPackets::ElementSubscribe(id, _) => *id,
            ClientPackets::ElementUnSubscribe(id, _) => *id,
            ClientPackets::CreateLocation(id, _) => *id,
            ClientPackets::GetLocationsLen(id, _) => *id,
            ClientPackets::GetLocations(id, _) => *id,
            ClientPackets::DestroyLocation(id, _) => *id,
            ClientPackets::MoveLocation(id, _) => *id,
            ClientPackets::LocationGetPath(id, _) => *id,
            ClientPackets::LocationSetPath(id, _) => *id,
            ClientPackets::LocationGetShouldSave(id, _) => *id,
            ClientPackets::LocationSetShouldSave(id, _) => *id,
            ClientPackets::LocationGetElementsLen(id, _) => *id,
            ClientPackets::LocationGetElements(id, _) => *id,
            ClientPackets::LocationNotify(id, _) => *id,
            ClientPackets::LocationEmit(id, _) => *id,
            ClientPackets::LocationSubscribe(id, _) => *id,
            ClientPackets::LocationUnSubscribe(id, _) => *id,
            ClientPackets::ModuleAcceptedProtocols(id, _) => *id,
            ClientPackets::ElementGetUrl(id, _) => *id,
            ClientPackets::ElementSetUrl(id, _) => *id,
            ClientPackets::LoadModuleInfo(id, _) => *id,
            ClientPackets::FindModule(id, _) => *id,
            ClientPackets::ModuleGetUid(id, _) => *id,
            ClientPackets::ModuleGetVersion(id, _) => *id,
            ClientPackets::ModuleSupportedVersions(id, _) => *id,
            ClientPackets::ModuleAcceptedExtensions(id, _) => *id,
        }
    }
}
