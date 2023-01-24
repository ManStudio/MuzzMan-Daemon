use std::{
    net::{SocketAddr, UdpSocket},
    ops::{AddAssign, Sub},
    path::PathBuf,
    str::FromStr,
    sync::{Arc, RwLock},
    time::{Duration, SystemTime},
};

use bytes_kman::TBytes;
use common::get_muzzman_dir;
use muzzman_lib::prelude::*;
use packets::{ClientPackets, ServerPackets};

pub mod common;
pub mod packets;

pub struct DaemonSession {
    pub conn: UdpSocket,
    pub packets: Vec<ClientPackets>,
    pub generator: u128,
}

impl DaemonSession {
    pub fn new() -> Result<Self, std::io::Error> {
        let conn = UdpSocket::bind(format!("0.0.0.0:{}", rand::random::<u16>()))?;
        conn.connect("0.0.0.0:2118")?;
        conn.set_read_timeout(Some(Duration::new(10, 0)));
        Ok(Self {
            conn,
            packets: vec![],
            generator: 1,
        })
    }

    pub fn pull_packets(&mut self) {
        let mut buffer = [0; 4096];

        if let Ok(len) = self.conn.recv(&mut buffer) {
            let mut buffer = buffer[0..len].to_vec();

            while !buffer.is_empty() {
                if let Some(packet) = ClientPackets::from_bytes(&mut buffer) {
                    println!("Recv {:?}", packet);
                    self.packets.push(packet)
                }
            }
        }
    }

    pub fn create_session(self) -> Box<dyn TSession> {
        Box::new(Box::new(Arc::new(RwLock::new(self))) as Box<dyn TDaemonSession>)
    }
}

impl TDaemonSession for Arc<RwLock<DaemonSession>> {
    fn pull_packets(&self) {
        self.write().unwrap().pull_packets()
    }

    fn waiting_for(&self, id: u128) -> Option<ClientPackets> {
        let start_time = SystemTime::now();
        let mut index = None;
        'm: loop {
            if start_time.elapsed().unwrap() > Duration::from_secs(1) {
                println!("Time Out!");
                break;
            }
            for (i, packet) in self.read().unwrap().packets.iter().enumerate() {
                if packet.id() == id {
                    index = Some(i);
                    break 'm;
                }
            }
            self.pull_packets();
        }

        index.map(|index| self.write().unwrap().packets.remove(index))
    }

    fn send(&self, packet: ServerPackets) {
        let mut bytes = packet.to_bytes();
        bytes.reverse();
        let _ = self.write().unwrap().conn.send(&bytes);
    }

    fn generate(&self) -> u128 {
        self.write().unwrap().generator.add_assign(1);
        self.read().unwrap().generator.sub(1)
    }

    fn cl(&self) -> Box<dyn TDaemonSession> {
        Box::new(self.clone())
    }
}

pub trait TDaemonSession {
    fn pull_packets(&self);

    fn waiting_for(&self, id: u128) -> Option<ClientPackets>;
    fn send(&self, packet: ServerPackets);
    fn generate(&self) -> u128;

    fn cl(&self) -> Box<dyn TDaemonSession>;
}

impl TSession for Box<dyn TDaemonSession> {
    fn load_module(&self, path: PathBuf) -> Result<MRef, SessionError> {
        todo!()
    }

    fn remove_module(&self, id: ModuleId) -> Result<MRow, SessionError> {
        todo!()
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
        todo!()
    }

    fn get_actions_len(&self) -> Result<usize, SessionError> {
        todo!()
    }

    fn run_action(
        &self,
        module_id: &ModuleId,
        name: String,
        data: Vec<Type>,
    ) -> Result<(), SessionError> {
        todo!()
    }

    fn get_modules_len(&self) -> Result<usize, SessionError> {
        todo!()
    }

    fn get_modules(&self, range: std::ops::Range<usize>) -> Result<Vec<MRef>, SessionError> {
        todo!()
    }

    fn get_module_name(&self, module_id: &ModuleId) -> Result<String, SessionError> {
        todo!()
    }

    fn set_module_name(&self, module_id: &ModuleId, name: String) -> Result<(), SessionError> {
        todo!()
    }

    fn default_module_name(&self, module_id: &ModuleId) -> Result<(), SessionError> {
        todo!()
    }

    fn get_module_desc(&self, module_id: &ModuleId) -> Result<String, SessionError> {
        todo!()
    }

    fn set_module_desc(&self, module_id: &ModuleId, desc: String) -> Result<(), SessionError> {
        todo!()
    }

    fn default_module_desc(&self, module_id: &ModuleId) -> Result<(), SessionError> {
        todo!()
    }

    fn get_module_proxy(&self, module_id: &ModuleId) -> Result<usize, SessionError> {
        todo!()
    }

    fn set_module_proxy(&self, module_id: &ModuleId, proxy: usize) -> Result<(), SessionError> {
        todo!()
    }

    fn get_module_settings(&self, module_id: &ModuleId) -> Result<Data, SessionError> {
        todo!()
    }

    fn set_module_settings(&self, module_id: &ModuleId, data: Data) -> Result<(), SessionError> {
        todo!()
    }

    fn get_module_element_settings(&self, module_id: &ModuleId) -> Result<Data, SessionError> {
        todo!()
    }

    fn set_module_element_settings(
        &self,
        module_id: &ModuleId,
        data: Data,
    ) -> Result<(), SessionError> {
        todo!()
    }

    fn module_init_location(
        &self,
        module_id: &ModuleId,
        location_id: &LocationId,
        data: FileOrData,
    ) -> Result<(), SessionError> {
        todo!()
    }

    fn module_init_element(
        &self,
        module_id: &ModuleId,
        element_id: &ElementId,
    ) -> Result<(), SessionError> {
        todo!()
    }

    fn moduie_accept_url(&self, module_id: &ModuleId, url: Url) -> Result<bool, SessionError> {
        todo!()
    }

    fn module_accept_extension(
        &self,
        module_id: &ModuleId,
        filename: &str,
    ) -> Result<bool, SessionError> {
        todo!()
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
        todo!()
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
        todo!()
    }

    fn element_set_name(&self, element_id: &ElementId, name: &str) -> Result<(), SessionError> {
        todo!()
    }

    fn element_get_desc(&self, element_id: &ElementId) -> Result<String, SessionError> {
        todo!()
    }

    fn element_set_desc(&self, element_id: &ElementId, desc: &str) -> Result<(), SessionError> {
        todo!()
    }

    fn element_get_meta(&self, element_id: &ElementId) -> Result<String, SessionError> {
        todo!()
    }

    fn element_set_meta(&self, element_id: &ElementId, meta: &str) -> Result<(), SessionError> {
        todo!()
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
        todo!()
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
                    Ok(ok) => Ok(Arc::new(RwLock::new(RefLocation {
                        session: Some(self.c()),
                        id: ok,
                    }))),
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
        let packet = ServerPackets::GetLocationName {
            id,
            from: location_id.clone(),
        };
        self.send(packet);
        if let Some(ClientPackets::GetLocationName(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn location_set_name(&self, location_id: &LocationId, name: &str) -> Result<(), SessionError> {
        let id = self.generate();
        let packet = ServerPackets::SetLocationName {
            id,
            from: location_id.clone(),
            to: name.to_string(),
        };
        self.send(packet);
        if let Some(ClientPackets::SetLocationName(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn location_get_desc(&self, location_id: &LocationId) -> Result<String, SessionError> {
        let id = self.generate();
        let packet = ServerPackets::GetLocationDesc {
            id,
            from: location_id.clone(),
        };
        self.send(packet);
        if let Some(ClientPackets::GetLocationDesc(_, response)) = self.waiting_for(id) {
            response
        } else {
            Err(SessionError::ServerTimeOut)
        }
    }

    fn location_set_desc(&self, location_id: &LocationId, desc: &str) -> Result<(), SessionError> {
        let id = self.generate();
        let packet = ServerPackets::SetLocationDesc {
            id,
            from: location_id.clone(),
            to: desc.to_string(),
        };
        self.send(packet);
        if let Some(ClientPackets::SetLocationDesc(_, response)) = self.waiting_for(id) {
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
        let packet = ServerPackets::GetLocationInfo {
            id,
            from: location_id.clone(),
        };
        self.send(packet);
        if let Some(ClientPackets::GetLocationInfo(_, response)) = self.waiting_for(id) {
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
        todo!()
    }

    fn get_element_ref(&self, id: &ElementId) -> Result<ERef, SessionError> {
        todo!()
    }

    fn get_location_ref(&self, id: &LocationId) -> Result<LRef, SessionError> {
        todo!()
    }

    fn c(&self) -> Box<dyn TSession> {
        Box::new(self.cl())
    }
}
