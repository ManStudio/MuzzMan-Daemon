use std::{
    path::PathBuf,
    sync::{Arc, RwLock},
};

use crate::{
    packets::{ClientPackets, ServerPackets},
    TDaemonSession,
};
use muzzman_lib::prelude::*;

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

    fn register_action(
        &self,
        module_id: &ModuleId,
        name: String,
        values: Vec<(String, Value)>,
        callback: fn(MRef, values: Vec<Type>),
    ) -> Result<(), SessionError> {
        todo!()
    }

    fn remove_action(&self, module_id: &ModuleId, name: String) -> Result<(), SessionError> {
        todo!()
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

    fn get_module_name(&self, module_id: &ModuleId) -> Result<String, SessionError> {
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

    fn set_module_name(&self, module_id: &ModuleId, name: String) -> Result<(), SessionError> {
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

    fn default_module_name(&self, module_id: &ModuleId) -> Result<(), SessionError> {
        let id = self.generate();
        let packet = ServerPackets::ModuleGetDefaultName {
            id,
            module_id: *module_id,
        };

        self.send(packet);
        if let Some(ClientPackets::ModuleDefaultName(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn get_module_desc(&self, module_id: &ModuleId) -> Result<String, SessionError> {
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

    fn set_module_desc(&self, module_id: &ModuleId, desc: String) -> Result<(), SessionError> {
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

    fn default_module_desc(&self, module_id: &ModuleId) -> Result<(), SessionError> {
        let id = self.generate();
        let packet = ServerPackets::ModuleGetDefaultDesc {
            id,
            module_id: *module_id,
        };

        self.send(packet);
        if let Some(ClientPackets::ModuleDefaultDesc(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn get_module_proxy(&self, module_id: &ModuleId) -> Result<usize, SessionError> {
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

    fn set_module_proxy(&self, module_id: &ModuleId, proxy: usize) -> Result<(), SessionError> {
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

    fn get_module_settings(&self, module_id: &ModuleId) -> Result<Data, SessionError> {
        let id = self.generate();
        let packet = ServerPackets::ModuleGetSettings {
            id,
            module_id: *module_id,
        };

        self.send(packet);
        if let Some(ClientPackets::ModuleGetSettings(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn set_module_settings(&self, module_id: &ModuleId, data: Data) -> Result<(), SessionError> {
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

    fn get_module_element_settings(&self, module_id: &ModuleId) -> Result<Data, SessionError> {
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

    fn set_module_element_settings(
        &self,
        module_id: &ModuleId,
        data: Data,
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
        data: FileOrData,
    ) -> Result<(), SessionError> {
        let id = self.generate();
        let packet = ServerPackets::ModuleInitLocation {
            id,
            module_id: *module_id,
            location_id: location_id.clone(),
            data,
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

    fn moduie_accept_url(&self, module_id: &ModuleId, url: Url) -> Result<bool, SessionError> {
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

    fn module_step_element(
        &self,
        module_id: &ModuleId,
        element_id: &ElementId,
        control_flow: &mut ControlFlow,
        storage: &mut Storage,
    ) -> Result<(), SessionError> {
        todo!()
    }

    fn module_step_location(
        &self,
        module_id: &ModuleId,
        location_id: &LocationId,
        control_flow: &mut ControlFlow,
        storage: &mut Storage,
    ) -> Result<(), SessionError> {
        todo!()
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

    fn move_element(
        &self,
        element: &ElementId,
        location_id: &LocationId,
    ) -> Result<(), SessionError> {
        todo!()
    }

    fn destroy_element(&self, element_id: ElementId) -> Result<ERow, SessionError> {
        todo!()
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

    fn element_get_element_data(&self, element_id: &ElementId) -> Result<Data, SessionError> {
        todo!()
    }

    fn element_set_element_data(
        &self,
        element_id: &ElementId,
        data: Data,
    ) -> Result<(), SessionError> {
        todo!()
    }

    fn element_get_module_data(&self, element_id: &ElementId) -> Result<Data, SessionError> {
        todo!()
    }

    fn element_set_module_data(
        &self,
        element_id: &ElementId,
        data: Data,
    ) -> Result<(), SessionError> {
        todo!()
    }

    fn element_get_module(&self, element_id: &ElementId) -> Result<Option<MRef>, SessionError> {
        todo!()
    }

    fn element_set_module(
        &self,
        element: &ElementId,
        module: Option<ModuleId>,
    ) -> Result<(), SessionError> {
        todo!()
    }

    fn element_get_statuses(&self, element_id: &ElementId) -> Result<Vec<String>, SessionError> {
        todo!()
    }

    fn element_set_statuses(
        &self,
        element: &ElementId,
        statuses: Vec<String>,
    ) -> Result<(), SessionError> {
        todo!()
    }

    fn element_get_status(&self, element_id: &ElementId) -> Result<usize, SessionError> {
        todo!()
    }

    fn element_set_status(
        &self,
        element_id: &ElementId,
        status: usize,
    ) -> Result<(), SessionError> {
        todo!()
    }

    fn element_get_data(&self, element_id: &ElementId) -> Result<FileOrData, SessionError> {
        todo!()
    }

    fn element_set_data(
        &self,
        element_id: &ElementId,
        data: FileOrData,
    ) -> Result<(), SessionError> {
        todo!()
    }

    fn element_get_progress(&self, element_id: &ElementId) -> Result<f32, SessionError> {
        todo!()
    }

    fn element_set_progress(
        &self,
        element_id: &ElementId,
        progress: f32,
    ) -> Result<(), SessionError> {
        todo!()
    }

    fn element_get_should_save(&self, element_id: &ElementId) -> Result<bool, SessionError> {
        todo!()
    }

    fn element_set_should_save(
        &self,
        element: &ElementId,
        should_save: bool,
    ) -> Result<(), SessionError> {
        todo!()
    }

    fn element_get_enabled(&self, element_id: &ElementId) -> Result<bool, SessionError> {
        todo!()
    }

    fn element_set_enabled(
        &self,
        element_id: &ElementId,
        enabled: bool,
        storage: Option<Storage>,
    ) -> Result<(), SessionError> {
        todo!()
    }

    fn element_resolv_module(&self, element_id: &ElementId) -> Result<bool, SessionError> {
        todo!()
    }

    fn element_wait(&self, element_id: &ElementId) -> Result<(), SessionError> {
        todo!()
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
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn element_notify(&self, element_id: &ElementId, event: Event) -> Result<(), SessionError> {
        todo!()
    }

    fn element_emit(&self, element_id: &ElementId, event: Event) -> Result<(), SessionError> {
        todo!()
    }

    fn element_subscribe(&self, element_id: &ElementId, _ref: ID) -> Result<(), SessionError> {
        todo!()
    }

    fn element_unsubscribe(&self, element_id: &ElementId, _ref: ID) -> Result<(), SessionError> {
        todo!()
    }

    fn create_location(&self, name: &str, location_id: &LocationId) -> Result<LRef, SessionError> {
        todo!()
    }

    fn get_locations_len(&self, location_id: &LocationId) -> Result<usize, SessionError> {
        todo!()
    }

    fn get_locations(
        &self,
        location_id: &LocationId,
        range: std::ops::Range<usize>,
    ) -> Result<Vec<LRef>, SessionError> {
        todo!()
    }

    fn destroy_location(&self, location_id: LocationId) -> Result<LRow, SessionError> {
        todo!()
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
        todo!()
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
        todo!()
    }

    fn location_set_path(
        &self,
        location_id: &LocationId,
        path: PathBuf,
    ) -> Result<(), SessionError> {
        todo!()
    }

    fn location_get_where_is(
        &self,
        location_id: &LocationId,
    ) -> Result<WhereIsLocation, SessionError> {
        todo!()
    }

    fn location_set_where_is(
        &self,
        location_id: &LocationId,
        where_is: WhereIsLocation,
    ) -> Result<(), SessionError> {
        todo!()
    }

    fn location_get_should_save(&self, location_id: &LocationId) -> Result<bool, SessionError> {
        todo!()
    }

    fn location_set_should_save(
        &self,
        location_id: &LocationId,
        should_save: bool,
    ) -> Result<(), SessionError> {
        todo!()
    }

    fn location_get_elements_len(&self, location_id: &LocationId) -> Result<usize, SessionError> {
        todo!()
    }

    fn location_get_elements(
        &self,
        location_id: &LocationId,
        range: std::ops::Range<usize>,
    ) -> Result<Vec<ERef>, SessionError> {
        todo!()
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
        todo!()
    }

    fn location_emit(&self, location_id: &LocationId, event: Event) -> Result<(), SessionError> {
        todo!()
    }

    fn location_subscribe(&self, location_id: &LocationId, _ref: ID) -> Result<(), SessionError> {
        todo!()
    }

    fn location_unsubscribe(&self, location_id: &LocationId, _ref: ID) -> Result<(), SessionError> {
        todo!()
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

    fn c(&self) -> Box<dyn TSession> {
        Box::new(self.cl())
    }
}
