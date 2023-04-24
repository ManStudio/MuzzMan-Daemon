use std::path::PathBuf;

use crate::{
    packets::{ClientPackets, ServerPackets},
    TDaemonSession,
};
use muzzman_lib::prelude::*;

pub const DAEMON_CLIENT_VERSION: u64 = 1;

impl TSession for Box<dyn TDaemonSession> {
    fn load_module(&self, path: PathBuf) -> Result<MRef, SessionError> {
        let id = self.generate();
        let packet = ServerPackets::LoadModule { id, path };

        self.send(packet);
        if let Some(ClientPackets::LoadModule(_, response)) = self.waiting_for(id) {
            match response {
                Ok(ok) => Ok(self.mref_get_or_add(ok)),
                Err(err) => Err(err),
            }
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn remove_module(&self, module_id: ModuleId) -> Result<MRow, SessionError> {
        let id = self.generate();
        let packet = ServerPackets::RemoveModule { id, module_id };

        self.send(packet);
        if let Some(ClientPackets::RemoveModule(_, response)) = self.waiting_for(id) {
            match response {
                Ok(_) => Err(SessionError::Custom("Cannot be transfered".into())),
                Err(err) => Err(err),
            }
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn load_module_info(&self, info: ModuleInfo) -> Result<MRef, SessionError> {
        let id = self.generate();
        let packet = ServerPackets::LoadModuleInfo {
            id,
            module_info: info,
        };

        self.send(packet);
        if let Some(ClientPackets::LoadModuleInfo(_, response)) = self.waiting_for(id) {
            match response {
                Ok(id) => self.get_module_ref(&id),
                Err(err) => Err(err),
            }
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn find_module(&self, info: ModuleInfo) -> Result<MRef, SessionError> {
        let id = self.generate();
        let packet = ServerPackets::FindModule {
            id,
            module_info: info,
        };

        self.send(packet);
        if let Some(ClientPackets::FindModule(_, response)) = self.waiting_for(id) {
            match response {
                Ok(id) => self.get_module_ref(&id),
                Err(err) => Err(err),
            }
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn register_action(
        &self,
        _module_id: &ModuleId,
        _name: String,
        _values: Vec<(String, Value)>,
        _callback: fn(MRef, values: Vec<Type>),
    ) -> Result<(), SessionError> {
        panic!("Cannot register action over the network because function calls are unexpected behaviour\nIf you want to register action you should a module that do what you need to do!");
    }

    fn remove_action(&self, _module_id: &ModuleId, _name: String) -> Result<(), SessionError> {
        panic!("remove_action is not implemented because you cannot register action!")
    }

    fn get_actions(&self, range: std::ops::Range<usize>) -> Result<Actions, SessionError> {
        let id = self.generate();
        let packet = ServerPackets::GetActions { id, range };

        self.send(packet);
        if let Some(ClientPackets::GetActions(_, response)) = self.waiting_for(id) {
            match response {
                Ok(ok) => {
                    let mut tmp = Vec::new();
                    for k in ok {
                        tmp.push((k.0, self.mref_get_or_add(k.1), k.2))
                    }
                    Ok(tmp)
                }
                Err(err) => Err(err),
            }
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn get_actions_len(&self) -> Result<usize, SessionError> {
        let id = self.generate();
        let packet = ServerPackets::GetActionsLen { id };

        self.send(packet);
        if let Some(ClientPackets::GetActionsLen(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn run_action(
        &self,
        module_id: &ModuleId,
        name: String,
        data: Vec<Type>,
    ) -> Result<(), SessionError> {
        let id = self.generate();
        let packet = ServerPackets::RunAction {
            id,
            module_id: *module_id,
            name,
            data,
        };

        self.send(packet);
        if let Some(ClientPackets::RunAction(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn get_modules_len(&self) -> Result<usize, SessionError> {
        let id = self.generate();
        let packet = ServerPackets::GetModulesLen { id };

        self.send(packet);
        if let Some(ClientPackets::GetModulesLen(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn get_modules(&self, range: std::ops::Range<usize>) -> Result<Vec<MRef>, SessionError> {
        let id = self.generate();
        let packet = ServerPackets::GetModules { id, range };

        self.send(packet);
        if let Some(ClientPackets::GetModules(_, response)) = self.waiting_for(id) {
            match response {
                Ok(ok) => {
                    let mut tmp = Vec::new();
                    for k in ok {
                        tmp.push(self.mref_get_or_add(k))
                    }
                    Ok(tmp)
                }
                Err(err) => Err(err),
            }
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn module_get_name(&self, module_id: &ModuleId) -> Result<String, SessionError> {
        let id = self.generate();
        let packet = ServerPackets::ModuleGetName {
            id,
            module_id: *module_id,
        };

        self.send(packet);
        if let Some(ClientPackets::ModuleGetName(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn module_set_name(&self, module_id: &ModuleId, name: String) -> Result<(), SessionError> {
        let id = self.generate();
        let packet = ServerPackets::ModuleSetName {
            id,
            module_id: *module_id,
            to: name,
        };

        self.send(packet);
        if let Some(ClientPackets::ModuleSetName(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn module_get_default_name(&self, module_id: &ModuleId) -> Result<String, SessionError> {
        let id = self.generate();
        let packet = ServerPackets::ModuleGetDefaultName {
            id,
            module_id: *module_id,
        };

        self.send(packet);
        if let Some(ClientPackets::ModuleGetDefaultName(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn module_get_uid(&self, module_id: &ModuleId) -> Result<UID, SessionError> {
        let id = self.generate();
        let packet = ServerPackets::ModuleGetUid {
            id,
            module_id: *module_id,
        };

        self.send(packet);
        if let Some(ClientPackets::ModuleGetUid(_, res)) = self.waiting_for(id) {
            res
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn module_get_version(&self, module_id: &ModuleId) -> Result<String, SessionError> {
        let id = self.generate();
        let packet = ServerPackets::ModuleGetVersion {
            id,
            module_id: *module_id,
        };

        self.send(packet);
        if let Some(ClientPackets::ModuleGetVersion(_, res)) = self.waiting_for(id) {
            res
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn module_supported_versions(
        &self,
        module_id: &ModuleId,
    ) -> Result<std::ops::Range<u64>, SessionError> {
        let id = self.generate();
        let packet = ServerPackets::ModuleSupportedVersions {
            id,
            module_id: *module_id,
        };

        self.send(packet);
        if let Some(ClientPackets::ModuleSupportedVersions(_, res)) = self.waiting_for(id) {
            res
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn module_get_desc(&self, module_id: &ModuleId) -> Result<String, SessionError> {
        let id = self.generate();
        let packet = ServerPackets::ModuleGetDesc {
            id,
            module_id: *module_id,
        };

        self.send(packet);
        if let Some(ClientPackets::ModuleGetDesc(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn module_set_desc(&self, module_id: &ModuleId, desc: String) -> Result<(), SessionError> {
        let id = self.generate();
        let packet = ServerPackets::ModuleSetDesc {
            id,
            module_id: *module_id,
            to: desc,
        };

        self.send(packet);
        if let Some(ClientPackets::ModuleSetDesc(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn module_get_default_desc(&self, module_id: &ModuleId) -> Result<String, SessionError> {
        let id = self.generate();
        let packet = ServerPackets::ModuleGetDefaultDesc {
            id,
            module_id: *module_id,
        };

        self.send(packet);
        if let Some(ClientPackets::ModuleGetDefaultDesc(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn module_get_proxy(&self, module_id: &ModuleId) -> Result<usize, SessionError> {
        let id = self.generate();
        let packet = ServerPackets::ModuleGetProxy {
            id,
            module_id: *module_id,
        };

        self.send(packet);
        if let Some(ClientPackets::ModuleGetProxy(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn module_set_proxy(&self, module_id: &ModuleId, proxy: usize) -> Result<(), SessionError> {
        let id = self.generate();
        let packet = ServerPackets::ModuleSetProxy {
            id,
            module_id: *module_id,
            to: proxy,
        };

        self.send(packet);
        if let Some(ClientPackets::ModuleSetProxy(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn module_get_settings(&self, module_id: &ModuleId) -> Result<Values, SessionError> {
        let id = self.generate();
        let packet = ServerPackets::ModuleGetSettings {
            id,
            module_id: *module_id,
        };

        self.send(packet);
        if let Some(ClientPackets::ModuleGetSettings(_, response)) = self.waiting_for(id) {
            *response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn module_set_settings(&self, module_id: &ModuleId, data: Values) -> Result<(), SessionError> {
        let id = self.generate();
        let packet = ServerPackets::ModuleSetSettings {
            id,
            module_id: *module_id,
            to: data,
        };

        self.send(packet);
        if let Some(ClientPackets::ModuleSetSettings(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn module_get_element_settings(&self, module_id: &ModuleId) -> Result<Values, SessionError> {
        let id = self.generate();
        let packet = ServerPackets::ModuleGetElementSettings {
            id,
            module_id: *module_id,
        };

        self.send(packet);
        if let Some(ClientPackets::ModuleGetElementSettings(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn module_set_element_settings(
        &self,
        module_id: &ModuleId,
        data: Values,
    ) -> Result<(), SessionError> {
        let id = self.generate();
        let packet = ServerPackets::ModuleSetElementSettings {
            id,
            module_id: *module_id,
            to: data,
        };

        self.send(packet);
        if let Some(ClientPackets::ModuleSetElementSettings(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn module_init_location(
        &self,
        module_id: &ModuleId,
        location_id: &LocationId,
    ) -> Result<(), SessionError> {
        let id = self.generate();
        let packet = ServerPackets::ModuleInitLocation {
            id,
            module_id: *module_id,
            location_id: location_id.clone(),
        };

        self.send(packet);
        if let Some(ClientPackets::ModuleInitLocation(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn module_init_element(
        &self,
        module_id: &ModuleId,
        element_id: &ElementId,
    ) -> Result<(), SessionError> {
        let id = self.generate();
        let packet = ServerPackets::ModuleInitElement {
            id,
            module_id: *module_id,
            element_id: element_id.clone(),
        };

        self.send(packet);
        if let Some(ClientPackets::ModuleInitElement(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn module_accept_url(&self, module_id: &ModuleId, url: String) -> Result<bool, SessionError> {
        let id = self.generate();
        let packet = ServerPackets::ModuleAcceptUrl {
            id,
            module_id: *module_id,
            url: url.to_string(),
        };

        self.send(packet);
        if let Some(ClientPackets::ModuleAcceptUrl(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn module_accept_extension(
        &self,
        module_id: &ModuleId,
        filename: &str,
    ) -> Result<bool, SessionError> {
        let id = self.generate();
        let packet = ServerPackets::ModuleAcceptExtension {
            id,
            module_id: *module_id,
            filename: filename.to_owned(),
        };

        self.send(packet);
        if let Some(ClientPackets::ModuleAcceptExtension(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn module_accepted_protocols(&self, module_id: &ModuleId) -> Result<Vec<String>, SessionError> {
        let id = self.generate();
        let packet = ServerPackets::ModuleAcceptedProtocols {
            id,
            module_id: *module_id,
        };

        self.send(packet);
        if let Some(ClientPackets::ModuleAcceptedProtocols(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn module_accepted_extensions(
        &self,
        module_id: &ModuleId,
    ) -> Result<Vec<String>, SessionError> {
        let id = self.generate();
        let packet = ServerPackets::ModuleAcceptedExtensions {
            id,
            module_id: *module_id,
        };

        self.send(packet);
        if let Some(ClientPackets::ModuleAcceptedExtensions(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn module_step_element(
        &self,
        _module_id: &ModuleId,
        _element_id: &ElementId,
        _control_flow: ControlFlow,
        _storage: Storage,
    ) -> Result<(ControlFlow, Storage), SessionError> {
        panic!("module_step_element cannot be implemented because Storage cannot be transfered! Storage contains row pointers that is only avalibile for the current program!");
    }

    fn module_step_location(
        &self,
        _module_id: &ModuleId,
        _location_id: &LocationId,
        _control_flow: ControlFlow,
        _storage: Storage,
    ) -> Result<(ControlFlow, Storage), SessionError> {
        panic!("module_step_location cannot be implemented because Storage cannot be transgered! Storage contains row pointers that is only avalibile for the current program!");
    }

    fn create_element(&self, name: &str, location_id: &LocationId) -> Result<ERef, SessionError> {
        let id = self.generate();
        let packet = ServerPackets::CreateElement {
            id,
            location_id: location_id.clone(),
            name: name.to_string(),
        };

        self.send(packet);
        if let Some(ClientPackets::CreateElement(_, response)) = self.waiting_for(id) {
            match response {
                Ok(ok) => Ok(self.eref_get_or_add(ok)),
                Err(err) => Err(err),
            }
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn load_element_info(&self, element_info: ElementInfo) -> Result<ERef, SessionError> {
        let id = self.generate();
        let packet = ServerPackets::LoadElementInfo { id, element_info };

        self.send(packet);
        if let Some(ClientPackets::LoadElementInfo(_, id)) = self.waiting_for(id) {
            self.get_element_ref(&id?)
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn move_element(
        &self,
        element: &ElementId,
        location_id: &LocationId,
    ) -> Result<(), SessionError> {
        let id = self.generate();
        let packet = ServerPackets::MoveElement {
            id,
            element_id: element.clone(),
            location_id: location_id.clone(),
        };

        self.send(packet);
        if let Some(ClientPackets::MoveElement(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn destroy_element(&self, element_id: ElementId) -> Result<ERow, SessionError> {
        let id = self.generate();
        let packet = ServerPackets::DestroyElement { id, element_id };

        self.send(packet);
        if let Some(ClientPackets::DestroyElement(_, response)) = self.waiting_for(id) {
            match response {
                Ok(_) => Err(SessionError::Custom("Cannot Transfer ERow".into())),
                Err(err) => Err(err),
            }
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn element_get_name(&self, element_id: &ElementId) -> Result<String, SessionError> {
        let id = self.generate();
        let packet = ServerPackets::ElementGetName {
            id,
            element_id: element_id.clone(),
        };

        self.send(packet);
        if let Some(ClientPackets::ElementGetName(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn element_set_name(&self, element_id: &ElementId, name: &str) -> Result<(), SessionError> {
        let id = self.generate();
        let packet = ServerPackets::ElementSetName {
            id,
            element_id: element_id.clone(),
            to: name.to_string(),
        };

        self.send(packet);
        if let Some(ClientPackets::ElementSetName(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn element_get_desc(&self, element_id: &ElementId) -> Result<String, SessionError> {
        let id = self.generate();
        let packet = ServerPackets::ElementGetDesc {
            id,
            element_id: element_id.clone(),
        };

        self.send(packet);
        if let Some(ClientPackets::ElementGetDesc(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn element_set_desc(&self, element_id: &ElementId, desc: &str) -> Result<(), SessionError> {
        let id = self.generate();
        let packet = ServerPackets::ElementSetDesc {
            id,
            element_id: element_id.clone(),
            to: desc.to_string(),
        };

        self.send(packet);
        if let Some(ClientPackets::ElementSetDesc(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn element_get_meta(&self, element_id: &ElementId) -> Result<String, SessionError> {
        let id = self.generate();
        let packet = ServerPackets::ElementGetMeta {
            id,
            element_id: element_id.clone(),
        };

        self.send(packet);
        if let Some(ClientPackets::ElementGetMeta(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn element_set_meta(&self, element_id: &ElementId, meta: &str) -> Result<(), SessionError> {
        let id = self.generate();
        let packet = ServerPackets::ElementSetMeta {
            id,
            element_id: element_id.clone(),
            to: meta.to_string(),
        };

        self.send(packet);
        if let Some(ClientPackets::ElementSetMeta(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn element_get_url(&self, element_id: &ElementId) -> Result<Option<String>, SessionError> {
        let id = self.generate();
        let packet = ServerPackets::ElementGetUrl {
            id,
            element_id: element_id.clone(),
        };
        self.send(packet);

        if let Some(ClientPackets::ElementGetUrl(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn element_set_url(
        &self,
        element_id: &ElementId,
        url: Option<String>,
    ) -> Result<(), SessionError> {
        let id = self.generate();
        let packet = ServerPackets::ElementSetUrl {
            id,
            element_id: element_id.clone(),
            to: url,
        };
        self.send(packet);

        if let Some(ClientPackets::ElementSetUrl(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn element_get_element_data(&self, element_id: &ElementId) -> Result<Values, SessionError> {
        let id = self.generate();
        let packet = ServerPackets::ElementGetElementData {
            id,
            element_id: element_id.clone(),
        };

        self.send(packet);
        if let Some(ClientPackets::ElementGetElementData(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn element_set_element_data(
        &self,
        element_id: &ElementId,
        data: Values,
    ) -> Result<(), SessionError> {
        let id = self.generate();
        let packet = ServerPackets::ElementSetElementData {
            id,
            element_id: element_id.clone(),
            to: data,
        };

        self.send(packet);
        if let Some(ClientPackets::ElementSetElementData(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn element_get_module_data(&self, element_id: &ElementId) -> Result<Values, SessionError> {
        let id = self.generate();
        let packet = ServerPackets::ElementGetModuleData {
            id,
            element_id: element_id.clone(),
        };

        self.send(packet);
        if let Some(ClientPackets::ElementGetModuleData(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn element_set_module_data(
        &self,
        element_id: &ElementId,
        data: Values,
    ) -> Result<(), SessionError> {
        let id = self.generate();
        let packet = ServerPackets::ElementSetModuleData {
            id,
            element_id: element_id.clone(),
            to: data,
        };

        self.send(packet);
        if let Some(ClientPackets::ElementSetModuleData(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn element_get_module(&self, element_id: &ElementId) -> Result<Option<MRef>, SessionError> {
        let id = self.generate();
        let packet = ServerPackets::ElementGetModule {
            id,
            element_id: element_id.clone(),
        };

        self.send(packet);
        if let Some(ClientPackets::ElementGetModule(_, response)) = self.waiting_for(id) {
            match response {
                Ok(ok) => match ok {
                    Some(some) => Ok(Some(self.mref_get_or_add(some))),
                    None => Ok(None),
                },
                Err(err) => Err(err),
            }
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn element_set_module(
        &self,
        element_id: &ElementId,
        module: Option<ModuleId>,
    ) -> Result<(), SessionError> {
        let id = self.generate();
        let packet = ServerPackets::ElementSetModule {
            id,
            element_id: element_id.clone(),
            module,
        };

        self.send(packet);
        if let Some(ClientPackets::ElementSetModule(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn element_get_statuses(&self, element_id: &ElementId) -> Result<Vec<String>, SessionError> {
        let id = self.generate();
        let packet = ServerPackets::ElementGetStatuses {
            id,
            element_id: element_id.clone(),
        };

        self.send(packet);
        if let Some(ClientPackets::ElementGetStatuses(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn element_set_statuses(
        &self,
        element_id: &ElementId,
        statuses: Vec<String>,
    ) -> Result<(), SessionError> {
        let id = self.generate();
        let packet = ServerPackets::ElementSetStatuses {
            id,
            element_id: element_id.clone(),
            to: statuses,
        };

        self.send(packet);
        if let Some(ClientPackets::ElementSetStatuses(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn element_get_status(&self, element_id: &ElementId) -> Result<usize, SessionError> {
        let id = self.generate();
        let packet = ServerPackets::ElementGetStatus {
            id,
            element_id: element_id.clone(),
        };

        self.send(packet);
        if let Some(ClientPackets::ElementGetStatus(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn element_set_status(
        &self,
        element_id: &ElementId,
        status: usize,
    ) -> Result<(), SessionError> {
        let id = self.generate();
        let packet = ServerPackets::ElementSetStatus {
            id,
            element_id: element_id.clone(),
            to: status,
        };

        self.send(packet);
        if let Some(ClientPackets::ElementSetStatus(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn element_get_data(&self, element_id: &ElementId) -> Result<Data, SessionError> {
        let id = self.generate();
        let packet = ServerPackets::ElementGetData {
            id,
            element_id: element_id.clone(),
        };

        self.send(packet);
        if let Some(ClientPackets::ElementGetData(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn element_set_data(&self, element_id: &ElementId, data: Data) -> Result<(), SessionError> {
        let id = self.generate();
        let packet = ServerPackets::ElementSetData {
            id,
            element_id: element_id.clone(),
            to: data,
        };

        self.send(packet);
        if let Some(ClientPackets::ElementSetData(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn element_get_progress(&self, element_id: &ElementId) -> Result<f32, SessionError> {
        let id = self.generate();
        let packet = ServerPackets::ElementGetProgress {
            id,
            element_id: element_id.clone(),
        };

        self.send(packet);
        if let Some(ClientPackets::ElementGetProgress(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn element_set_progress(
        &self,
        element_id: &ElementId,
        progress: f32,
    ) -> Result<(), SessionError> {
        let id = self.generate();
        let packet = ServerPackets::ElementSetProgress {
            id,
            element_id: element_id.clone(),
            to: progress,
        };

        self.send(packet);
        if let Some(ClientPackets::ElementSetProgress(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn element_get_should_save(&self, element_id: &ElementId) -> Result<bool, SessionError> {
        let id = self.generate();
        let packet = ServerPackets::ElementGetShouldSave {
            id,
            element_id: element_id.clone(),
        };

        self.send(packet);
        if let Some(ClientPackets::ElementGetShouldSave(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn element_set_should_save(
        &self,
        element_id: &ElementId,
        should_save: bool,
    ) -> Result<(), SessionError> {
        let id = self.generate();
        let packet = ServerPackets::ElementSetShouldSave {
            id,
            element_id: element_id.clone(),
            to: should_save,
        };

        self.send(packet);
        if let Some(ClientPackets::ElementSetShouldSave(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn element_get_enabled(&self, element_id: &ElementId) -> Result<bool, SessionError> {
        let id = self.generate();
        let packet = ServerPackets::ElementGetEnabled {
            id,
            element_id: element_id.clone(),
        };

        self.send(packet);
        if let Some(ClientPackets::ElementGetEnabled(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn element_set_enabled(
        &self,
        element_id: &ElementId,
        enabled: bool,
        storage: Option<Storage>,
    ) -> Result<(), SessionError> {
        if storage.is_some() {
            panic!(
                "element_set_enable cannot be called with Storage! Storage cannot be transfered!"
            );
        }

        let id = self.generate();
        let packet = ServerPackets::ElementSetEnabled {
            id,
            element_id: element_id.clone(),
            to: enabled,
        };

        self.send(packet);
        if let Some(ClientPackets::ElementSetEnabled(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn element_resolv_module(&self, element_id: &ElementId) -> Result<bool, SessionError> {
        let id = self.generate();
        let packet = ServerPackets::ElementResolvModule {
            id,
            element_id: element_id.clone(),
        };

        self.send(packet);
        if let Some(ClientPackets::ElementResolvModule(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn element_wait(&self, element_id: &ElementId) -> Result<(), SessionError> {
        // TODO: Fix daemon problem in ServerPackets::ElementWait

        let id = self.generate();
        let packet = ServerPackets::ElementWait {
            id,
            element_id: element_id.clone(),
        };

        self.send(packet);
        if let Some(ClientPackets::ElementWait(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn element_get_element_info(
        &self,
        element_id: &ElementId,
    ) -> Result<ElementInfo, SessionError> {
        let id = self.generate();
        let packet = ServerPackets::ElementGetInfo {
            id,
            element_id: element_id.clone(),
        };

        self.send(packet);
        if let Some(ClientPackets::ElementGetInfo(_, response)) = self.waiting_for(id) {
            *response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn element_notify(&self, element_id: &ElementId, event: Event) -> Result<(), SessionError> {
        let id = self.generate();
        let packet = ServerPackets::ElementNotify {
            id,
            element_id: element_id.clone(),
            event,
        };

        self.send(packet);
        if let Some(ClientPackets::ElementNotify(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn element_emit(&self, element_id: &ElementId, event: Event) -> Result<(), SessionError> {
        let id = self.generate();
        let packet = ServerPackets::ElementEmit {
            id,
            element_id: element_id.clone(),
            event,
        };

        self.send(packet);
        if let Some(ClientPackets::ElementEmit(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn element_subscribe(&self, element_id: &ElementId, _ref: ID) -> Result<(), SessionError> {
        let id = self.generate();
        let packet = ServerPackets::ElementSubscribe {
            id,
            element_id: element_id.clone(),
            to: _ref,
        };

        self.send(packet);
        if let Some(ClientPackets::ElementSubscribe(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn element_unsubscribe(&self, element_id: &ElementId, _ref: ID) -> Result<(), SessionError> {
        let id = self.generate();
        let packet = ServerPackets::ElementUnSubscribe {
            id,
            element_id: element_id.clone(),
            to: _ref,
        };

        self.send(packet);
        if let Some(ClientPackets::ElementUnSubscribe(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn create_location(&self, name: &str, location_id: &LocationId) -> Result<LRef, SessionError> {
        let id = self.generate();
        let packet = ServerPackets::CreateLocation {
            id,
            name: name.to_owned(),
            location_id: location_id.clone(),
        };

        self.send(packet);
        if let Some(ClientPackets::CreateLocation(_, response)) = self.waiting_for(id) {
            match response {
                Ok(ok) => Ok(self.lref_get_or_add(ok)),
                Err(err) => Err(err),
            }
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn load_location_info(&self, location_info: LocationInfo) -> Result<LRef, SessionError> {
        let id = self.generate();
        let packet = ServerPackets::LoadLocationInfo { id, location_info };

        self.send(packet);
        if let Some(ClientPackets::LoadLocationInfo(_, response)) = self.waiting_for(id) {
            match response {
                Ok(id) => self.get_location_ref(&id),
                Err(err) => Err(err),
            }
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn get_locations_len(&self, location_id: &LocationId) -> Result<usize, SessionError> {
        let id = self.generate();
        let packet = ServerPackets::GetLocationsLen {
            id,
            location_id: location_id.clone(),
        };

        self.send(packet);
        if let Some(ClientPackets::GetLocationsLen(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn get_locations(
        &self,
        location_id: &LocationId,
        range: std::ops::Range<usize>,
    ) -> Result<Vec<LRef>, SessionError> {
        let id = self.generate();
        let packet = ServerPackets::GetLocations {
            id,
            location_id: location_id.clone(),
            range,
        };

        self.send(packet);
        if let Some(ClientPackets::GetLocations(_, response)) = self.waiting_for(id) {
            match response {
                Ok(ok) => {
                    let mut tmp = Vec::with_capacity(ok.len());

                    for k in ok {
                        tmp.push(self.lref_get_or_add(k))
                    }

                    Ok(tmp)
                }
                Err(err) => Err(err),
            }
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn destroy_location(&self, location_id: LocationId) -> Result<LRow, SessionError> {
        let id = self.generate();
        let packet = ServerPackets::DestroyLocation { id, location_id };

        self.send(packet);
        if let Some(ClientPackets::DestroyLocation(_, response)) = self.waiting_for(id) {
            match response {
                Ok(_) => Err(SessionError::Custom("LRow Cannot be transfered!".into())),
                Err(err) => Err(err),
            }
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn get_default_location(&self) -> Result<LRef, SessionError> {
        let id = self.generate();
        let packet = ServerPackets::GetDefaultLocation { id };
        self.send(packet);
        if let Some(packet) = self.waiting_for(id) {
            if let ClientPackets::GetDefaultLocation(_, response) = packet {
                match response {
                    Ok(ok) => Ok(self.lref_get_or_add(ok)),
                    Err(err) => Err(err),
                }
            } else {
                panic!()
            }
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn move_location(&self, location_id: &LocationId, to: &LocationId) -> Result<(), SessionError> {
        let id = self.generate();
        let packet = ServerPackets::MoveLocation {
            id,
            location_id: location_id.clone(),
            to: to.clone(),
        };
        self.send(packet);
        if let Some(ClientPackets::MoveLocation(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn location_get_name(&self, location_id: &LocationId) -> Result<String, SessionError> {
        let id = self.generate();
        let packet = ServerPackets::LocationGetName {
            id,
            from: location_id.clone(),
        };
        self.send(packet);
        if let Some(ClientPackets::LocationGetName(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn location_set_name(&self, location_id: &LocationId, name: &str) -> Result<(), SessionError> {
        let id = self.generate();
        let packet = ServerPackets::LocationSetName {
            id,
            from: location_id.clone(),
            to: name.to_string(),
        };
        self.send(packet);
        if let Some(ClientPackets::LocationSetName(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn location_get_desc(&self, location_id: &LocationId) -> Result<String, SessionError> {
        let id = self.generate();
        let packet = ServerPackets::LocationGetDesc {
            id,
            from: location_id.clone(),
        };
        self.send(packet);
        if let Some(ClientPackets::LocationGetDesc(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn location_set_desc(&self, location_id: &LocationId, desc: &str) -> Result<(), SessionError> {
        let id = self.generate();
        let packet = ServerPackets::LocationSetDesc {
            id,
            from: location_id.clone(),
            to: desc.to_string(),
        };
        self.send(packet);
        if let Some(ClientPackets::LocationSetDesc(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn location_get_path(&self, location_id: &LocationId) -> Result<PathBuf, SessionError> {
        let id = self.generate();
        let packet = ServerPackets::LocationGetPath {
            id,
            location_id: location_id.clone(),
        };
        self.send(packet);
        if let Some(ClientPackets::LocationGetPath(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn location_set_path(
        &self,
        location_id: &LocationId,
        path: PathBuf,
    ) -> Result<(), SessionError> {
        let id = self.generate();
        let packet = ServerPackets::LocationSetPath {
            id,
            location_id: location_id.clone(),
            to: path,
        };
        self.send(packet);
        if let Some(ClientPackets::LocationSetPath(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn location_get_where_is(
        &self,
        _location_id: &LocationId,
    ) -> Result<WhereIsLocation, SessionError> {
        todo!()
    }

    fn location_set_where_is(
        &self,
        _location_id: &LocationId,
        _where_is: WhereIsLocation,
    ) -> Result<(), SessionError> {
        todo!()
    }

    fn location_get_should_save(&self, location_id: &LocationId) -> Result<bool, SessionError> {
        let id = self.generate();
        let packet = ServerPackets::LocationGetShouldSave {
            id,
            location_id: location_id.clone(),
        };
        self.send(packet);
        if let Some(ClientPackets::LocationGetShouldSave(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn location_set_should_save(
        &self,
        location_id: &LocationId,
        should_save: bool,
    ) -> Result<(), SessionError> {
        let id = self.generate();
        let packet = ServerPackets::LocationSetShouldSave {
            id,
            location_id: location_id.clone(),
            to: should_save,
        };
        self.send(packet);
        if let Some(ClientPackets::LocationSetShouldSave(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn location_get_elements_len(&self, location_id: &LocationId) -> Result<usize, SessionError> {
        let id = self.generate();
        let packet = ServerPackets::LocationGetElementsLen {
            id,
            location_id: location_id.clone(),
        };
        self.send(packet);
        if let Some(ClientPackets::LocationGetElementsLen(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn location_get_elements(
        &self,
        location_id: &LocationId,
        range: std::ops::Range<usize>,
    ) -> Result<Vec<ERef>, SessionError> {
        let id = self.generate();
        let packet = ServerPackets::LocationGetElements {
            id,
            location_id: location_id.clone(),
            range,
        };
        self.send(packet);
        if let Some(ClientPackets::LocationGetElements(_, response)) = self.waiting_for(id) {
            match response {
                Ok(ok) => {
                    let mut tmp = Vec::with_capacity(ok.len());

                    for k in ok {
                        tmp.push(self.eref_get_or_add(k))
                    }

                    Ok(tmp)
                }
                Err(err) => Err(err),
            }
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn location_get_location_info(
        &self,
        location_id: &LocationId,
    ) -> Result<LocationInfo, SessionError> {
        let id = self.generate();
        let packet = ServerPackets::LocationGetInfo {
            id,
            from: location_id.clone(),
        };
        self.send(packet);
        if let Some(ClientPackets::LocationGetInfo(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn location_notify(&self, location_id: &LocationId, event: Event) -> Result<(), SessionError> {
        let id = self.generate();
        let packet = ServerPackets::LocationNotify {
            id,
            location_id: location_id.clone(),
            event,
        };
        self.send(packet);
        if let Some(ClientPackets::LocationNotify(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn location_emit(&self, location_id: &LocationId, event: Event) -> Result<(), SessionError> {
        let id = self.generate();
        let packet = ServerPackets::LocationEmit {
            id,
            location_id: location_id.clone(),
            event,
        };
        self.send(packet);
        if let Some(ClientPackets::LocationEmit(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn location_subscribe(&self, location_id: &LocationId, _ref: ID) -> Result<(), SessionError> {
        let id = self.generate();
        let packet = ServerPackets::LocationSubscribe {
            id,
            location_id: location_id.clone(),
            to: _ref,
        };
        self.send(packet);
        if let Some(ClientPackets::LocationSubscribe(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn location_unsubscribe(&self, location_id: &LocationId, _ref: ID) -> Result<(), SessionError> {
        let id = self.generate();
        let packet = ServerPackets::LocationUnSubscribe {
            id,
            location_id: location_id.clone(),
            to: _ref,
        };
        self.send(packet);
        if let Some(ClientPackets::LocationUnSubscribe(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn get_module_ref(&self, id: &ModuleId) -> Result<MRef, SessionError> {
        Ok(self.mref_get_or_add(*id))
    }

    fn get_element_ref(&self, id: &ElementId) -> Result<ERef, SessionError> {
        Ok(self.eref_get_or_add(id.clone()))
    }

    fn get_location_ref(&self, id: &LocationId) -> Result<LRef, SessionError> {
        Ok(self.lref_get_or_add(id.clone()))
    }

    fn get_version(&self) -> Result<u64, SessionError> {
        let id = self.generate();
        let packet = ServerPackets::GetVersion { id };
        self.send(packet);
        if let Some(ClientPackets::GetVersion(_, res)) = self.waiting_for(id) {
            res
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn get_version_text(&self) -> Result<String, SessionError> {
        let id = self.generate();
        let packet = ServerPackets::GetVersionText { id };
        self.send(packet);
        if let Some(ClientPackets::GetVersionText(_, res)) = self.waiting_for(id) {
            res.map(|version| format!("{version}, DaemonClient: {DAEMON_CLIENT_VERSION}"))
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn c(&self) -> Box<dyn TSession> {
        Box::new(self.cl())
    }
}
