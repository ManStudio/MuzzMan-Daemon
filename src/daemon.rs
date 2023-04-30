use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::Arc,
    time::{Duration, SystemTime},
};

const CLIENT_TIMEOUT: Duration = Duration::new(3, 0);

use async_trait::async_trait;

use crate::{
    packets::{ClientPackets, ServerPackets},
    DAEMON_PORT, DAEMON_VERSION,
};
use bytes_kman::TBytes;
use muzzman_lib::{
    prelude::{TElement, TLocation, TModuleInfo},
    session::TSession,
};
use tokio::{net::UdpSocket, sync::Mutex};

pub struct DaemonInner {
    socket: Arc<UdpSocket>,
    clients: Vec<(SystemTime, SocketAddr)>,
    buffer: [u8; 4096],
}

unsafe impl Send for DaemonInner {}
unsafe impl Sync for DaemonInner {}

pub struct Daemon {
    session: Box<dyn TSession>,
    inner: Arc<Mutex<DaemonInner>>,
    socket: Arc<UdpSocket>,
}

unsafe impl Sync for Daemon {}
unsafe impl Send for Daemon {}

impl Daemon {
    pub async fn new() -> Result<Self, std::io::Error> {
        let socket = UdpSocket::bind(format!("127.0.0.1:{DAEMON_PORT}")).await?;
        let socket = Arc::new(socket);
        let socket_clone = socket.clone();

        let mut session = muzzman_lib::LocalSession::default();

        let inner = Arc::new(Mutex::new(DaemonInner {
            socket,
            clients: Vec::new(),
            buffer: [0; 4096],
        }));

        let inner_clone = inner.clone();
        session.callback = Some(Box::new(move |event| {
            let inner_clone = inner_clone.clone();
            tokio::spawn(async move {
                log::info!("{event:?}");
                let clients = inner_clone.clients().await;
                for client in clients {
                    inner_clone
                        .send(ClientPackets::NewSessionEvent(event.clone()), &client)
                        .await;
                }
            });
        }));
        let session = session.new_session();
        let default_location = session.get_default_location().unwrap();
        default_location
            .set_path(dirs::home_dir().unwrap().join("Downloads"))
            .unwrap();

        Ok(Self {
            session,
            inner,
            socket: socket_clone,
        })
    }

    pub async fn run(mut self) {
        loop {
            let _ = self.socket.readable().await;
            self.respond_to_requests().await;
        }
    }

    async fn respond_to_requests(&mut self) {
        for (addr, packets) in self.inner.recv().await {
            for packet in packets {
                match packet {
                    ServerPackets::Tick => {}
                    ServerPackets::GetDefaultLocation { id } => {
                        let packet = match self.session.get_default_location() {
                            Ok(ok) => ClientPackets::GetDefaultLocation(id, Ok(ok.id())),
                            Err(err) => ClientPackets::GetDefaultLocation(id, Err(err)),
                        };
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::LocationGetName { id, from } => {
                        let packet = ClientPackets::LocationGetName(
                            id,
                            self.session.location_get_name(&from),
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::LocationSetName { id, from, to } => {
                        let packet = ClientPackets::LocationSetName(
                            id,
                            self.session.location_set_name(&from, &to),
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::LocationGetDesc { id, from } => {
                        let packet = ClientPackets::LocationGetDesc(
                            id,
                            self.session.location_get_desc(&from),
                        );

                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::LocationSetDesc { id, from, to } => {
                        let packet = ClientPackets::LocationSetDesc(
                            id,
                            self.session.location_set_desc(&from, &to),
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::LocationGetInfo { id, from } => {
                        let packet = ClientPackets::LocationGetInfo(
                            id,
                            self.session.location_get_location_info(&from),
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::CreateElement {
                        id,
                        location_id,
                        name,
                    } => {
                        let packet = match self.session.create_element(&name, &location_id) {
                            Ok(ok) => ClientPackets::CreateElement(id, Ok(ok.id())),
                            Err(err) => ClientPackets::CreateElement(id, Err(err)),
                        };
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::ElementGetName { id, element_id } => {
                        let packet = ClientPackets::ElementGetName(
                            id,
                            self.session.element_get_name(&element_id),
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::ElementSetName { id, element_id, to } => {
                        let packet = ClientPackets::ElementSetName(
                            id,
                            self.session.element_set_name(&element_id, &to),
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::ElementGetDesc { id, element_id } => {
                        let packet = ClientPackets::ElementGetDesc(
                            id,
                            self.session.element_get_desc(&element_id),
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::ElementSetDesc { id, element_id, to } => {
                        let packet = ClientPackets::ElementSetDesc(
                            id,
                            self.session.element_set_desc(&element_id, &to),
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::ElementGetMeta { id, element_id } => {
                        let packet = ClientPackets::ElementGetMeta(
                            id,
                            self.session.element_get_meta(&element_id),
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::ElementSetMeta { id, element_id, to } => {
                        let packet = ClientPackets::ElementSetMeta(
                            id,
                            self.session.element_set_meta(&element_id, &to),
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::ElementGetInfo { id, element_id } => {
                        let packet = ClientPackets::ElementGetInfo(
                            id,
                            Box::new(self.session.element_get_element_info(&element_id)),
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::LoadModule { id, path } => {
                        let packet = ClientPackets::LoadModule(
                            id,
                            match self.session.load_module(path) {
                                Ok(ok) => Ok(ok.id()),
                                Err(err) => Err(err),
                            },
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::RemoveModule { id, module_id } => {
                        let packet = ClientPackets::RemoveModule(
                            id,
                            match self.session.remove_module(module_id) {
                                Ok(_) => Ok(()),
                                Err(err) => Err(err),
                            },
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::GetActionsLen { id } => {
                        let packet =
                            ClientPackets::GetActionsLen(id, self.session.get_actions_len());
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::GetActions { id, range } => {
                        let packet = ClientPackets::GetActions(
                            id,
                            match self.session.get_actions(range) {
                                Ok(ok) => {
                                    let mut tmp = Vec::new();
                                    for k in ok {
                                        tmp.push((k.0, k.1.id(), k.2));
                                    }
                                    Ok(tmp)
                                }
                                Err(err) => Err(err),
                            },
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::RunAction {
                        id,
                        module_id,
                        name,
                        data,
                    } => {
                        let packet = ClientPackets::RunAction(
                            id,
                            self.session.run_action(&module_id, name, data),
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::GetModulesLen { id } => {
                        let packet =
                            ClientPackets::GetModulesLen(id, self.session.get_modules_len());
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::GetModules { id, range } => {
                        let packet = ClientPackets::GetModules(
                            id,
                            match self.session.get_modules(range) {
                                Ok(ok) => {
                                    let mut tmp = Vec::with_capacity(ok.len());
                                    for k in ok {
                                        tmp.push(k.id())
                                    }
                                    Ok(tmp)
                                }
                                Err(err) => Err(err),
                            },
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::ModuleGetName { id, module_id } => {
                        let packet = ClientPackets::ModuleGetName(
                            id,
                            self.session.module_get_name(&module_id),
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::ModuleSetName { id, module_id, to } => {
                        let packet = ClientPackets::ModuleSetName(
                            id,
                            self.session.module_set_name(&module_id, to),
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::ModuleGetDefaultName { id, module_id } => {
                        let packet = ClientPackets::ModuleGetDefaultName(
                            id,
                            self.session.module_get_default_name(&module_id),
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::ModuleGetDesc { id, module_id } => {
                        let packet = ClientPackets::ModuleGetDesc(
                            id,
                            self.session.module_get_desc(&module_id),
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::ModuleSetDesc { id, module_id, to } => {
                        let packet = ClientPackets::ModuleSetDesc(
                            id,
                            self.session.module_set_desc(&module_id, to),
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::ModuleGetDefaultDesc { id, module_id } => {
                        let packet = ClientPackets::ModuleGetDefaultDesc(
                            id,
                            self.session.module_get_default_desc(&module_id),
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::ModuleGetProxy { id, module_id } => {
                        let packet = ClientPackets::ModuleGetProxy(
                            id,
                            self.session.module_get_proxy(&module_id),
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::ModuleSetProxy { id, module_id, to } => {
                        let packet = ClientPackets::ModuleSetProxy(
                            id,
                            self.session.module_set_proxy(&module_id, to),
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::ModuleGetSettings { id, module_id } => {
                        let packet = ClientPackets::ModuleGetSettings(
                            id,
                            Box::new(self.session.module_get_settings(&module_id)),
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::ModuleSetSettings { id, module_id, to } => {
                        let packet = ClientPackets::ModuleSetSettings(
                            id,
                            self.session.module_set_settings(&module_id, to),
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::ModuleGetElementSettings { id, module_id } => {
                        let packet = ClientPackets::ModuleGetElementSettings(
                            id,
                            self.session.module_get_element_settings(&module_id),
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::ModuleSetElementSettings { id, module_id, to } => {
                        let packet = ClientPackets::ModuleSetElementSettings(
                            id,
                            self.session.module_set_element_settings(&module_id, to),
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::ModuleInitLocation {
                        id,
                        module_id,
                        location_id,
                    } => {
                        let packet = ClientPackets::ModuleInitLocation(
                            id,
                            self.session.module_init_location(&module_id, &location_id),
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::ModuleInitElement {
                        id,
                        module_id,
                        element_id,
                    } => {
                        let packet = ClientPackets::ModuleInitElement(
                            id,
                            self.session.module_init_element(&module_id, &element_id),
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::ModuleAcceptUrl { id, module_id, url } => {
                        let packet = ClientPackets::ModuleAcceptUrl(
                            id,
                            self.session.module_accept_url(&module_id, url),
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::ModuleAcceptExtension {
                        id,
                        module_id,
                        filename,
                    } => {
                        let packet = ClientPackets::ModuleAcceptExtension(
                            id,
                            self.session.module_accept_extension(&module_id, &filename),
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::MoveElement {
                        id,
                        element_id,
                        location_id,
                    } => {
                        let packet = ClientPackets::MoveElement(
                            id,
                            self.session.move_element(&element_id, &location_id),
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::DestroyElement { id, element_id } => {
                        let packet = ClientPackets::DestroyElement(
                            id,
                            match self.session.destroy_element(element_id) {
                                Ok(_) => Ok(()),
                                Err(err) => Err(err),
                            },
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::ElementGetElementData { id, element_id } => {
                        let packet = ClientPackets::ElementGetElementData(
                            id,
                            self.session.element_get_element_data(&element_id),
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::ElementSetElementData { id, element_id, to } => {
                        let packet = ClientPackets::ElementSetElementData(
                            id,
                            self.session.element_set_element_data(&element_id, to),
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::ElementGetModuleData { id, element_id } => {
                        let packet = ClientPackets::ElementGetModuleData(
                            id,
                            self.session.element_get_module_data(&element_id),
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::ElementSetModuleData { id, element_id, to } => {
                        let packet = ClientPackets::ElementSetModuleData(
                            id,
                            self.session.element_set_module_data(&element_id, to),
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::ElementGetModule { id, element_id } => {
                        let packet = ClientPackets::ElementGetModule(
                            id,
                            match self.session.element_get_module(&element_id) {
                                Ok(ok) => match ok {
                                    Some(some) => Ok(Some(some.id())),
                                    None => Ok(None),
                                },
                                Err(err) => Err(err),
                            },
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::ElementSetModule {
                        id,
                        element_id,
                        module,
                    } => {
                        let packet = ClientPackets::ElementSetModule(
                            id,
                            self.session.element_set_module(&element_id, module),
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::ElementGetStatuses { id, element_id } => {
                        let packet = ClientPackets::ElementGetStatuses(
                            id,
                            self.session.element_get_statuses(&element_id),
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::ElementSetStatuses { id, element_id, to } => {
                        let packet = ClientPackets::ElementSetStatuses(
                            id,
                            self.session.element_set_statuses(&element_id, to),
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::ElementGetStatus { id, element_id } => {
                        let packet = ClientPackets::ElementGetStatus(
                            id,
                            self.session.element_get_status(&element_id),
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::ElementSetStatus { id, element_id, to } => {
                        let packet = ClientPackets::ElementSetStatus(
                            id,
                            self.session.element_set_status(&element_id, to),
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::ElementGetData { id, element_id } => {
                        let packet = ClientPackets::ElementGetData(
                            id,
                            self.session.element_get_data(&element_id),
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::ElementSetData { id, element_id, to } => {
                        let packet = ClientPackets::ElementSetData(
                            id,
                            self.session.element_set_data(&element_id, to),
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::ElementGetProgress { id, element_id } => {
                        let packet = ClientPackets::ElementGetProgress(
                            id,
                            self.session.element_get_progress(&element_id),
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::ElementSetProgress { id, element_id, to } => {
                        let packet = ClientPackets::ElementSetProgress(
                            id,
                            self.session.element_set_progress(&element_id, to),
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::ElementGetShouldSave { id, element_id } => {
                        let packet = ClientPackets::ElementGetShouldSave(
                            id,
                            self.session.element_get_should_save(&element_id),
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::ElementSetShouldSave { id, element_id, to } => {
                        let packet = ClientPackets::ElementSetShouldSave(
                            id,
                            self.session.element_set_should_save(&element_id, to),
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::ElementGetEnabled { id, element_id } => {
                        let packet = ClientPackets::ElementGetEnabled(
                            id,
                            self.session.element_get_enabled(&element_id),
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::ElementSetEnabled { id, element_id, to } => {
                        let packet = ClientPackets::ElementSetEnabled(
                            id,
                            self.session.element_set_enabled(&element_id, to, None),
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::ElementResolvModule { id, element_id } => {
                        let packet = ClientPackets::ElementResolvModule(
                            id,
                            self.session.element_resolv_module(&element_id),
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::ElementWait { id, element_id } => {
                        // TODO: Implement this better
                        // This will block the daemon until the element is done
                        // This is really bad because any request from other sessions will be ignored
                        // and result into timeout
                        // this is here because i'am lazy to makeit better with some event handler
                        // if some one want to fix this go end do it
                        //
                        // Problem from 1/29/2023
                        let packet =
                            ClientPackets::ElementWait(id, self.session.element_wait(&element_id));
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::ElementNotify {
                        id,
                        element_id,
                        event,
                    } => {
                        let packet = ClientPackets::ElementNotify(
                            id,
                            self.session.element_notify(&element_id, event),
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::ElementEmit {
                        id,
                        element_id,
                        event,
                    } => {
                        let packet = ClientPackets::ElementEmit(
                            id,
                            self.session.element_emit(&element_id, event),
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::ElementSubscribe { id, element_id, to } => {
                        let packet = ClientPackets::ElementSubscribe(
                            id,
                            self.session.element_subscribe(&element_id, to),
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::ElementUnSubscribe { id, element_id, to } => {
                        let packet = ClientPackets::ElementUnSubscribe(
                            id,
                            self.session.element_unsubscribe(&element_id, to),
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::CreateLocation {
                        id,
                        name,
                        location_id,
                    } => {
                        let packet = ClientPackets::CreateLocation(
                            id,
                            match self.session.create_location(&name, &location_id) {
                                Ok(ok) => Ok(ok.id()),
                                Err(err) => Err(err),
                            },
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::GetLocationsLen { id, location_id } => {
                        let packet = ClientPackets::GetLocationsLen(
                            id,
                            self.session.get_locations_len(&location_id),
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::GetLocations {
                        id,
                        location_id,
                        range,
                    } => {
                        let packet = ClientPackets::GetLocations(
                            id,
                            match self.session.get_locations(&location_id, range) {
                                Ok(ok) => {
                                    let mut tmp = Vec::with_capacity(ok.len());

                                    for k in ok {
                                        tmp.push(k.id())
                                    }

                                    Ok(tmp)
                                }
                                Err(err) => Err(err),
                            },
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::DestroyLocation { id, location_id } => {
                        let packet = ClientPackets::DestroyLocation(
                            id,
                            match self.session.destroy_location(location_id) {
                                Ok(_) => Ok(()),
                                Err(err) => Err(err),
                            },
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::MoveLocation {
                        id,
                        location_id,
                        to,
                    } => {
                        let packet = ClientPackets::MoveLocation(
                            id,
                            self.session.move_location(&location_id, &to),
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::LocationGetPath { id, location_id } => {
                        let packet = ClientPackets::LocationGetPath(
                            id,
                            self.session.location_get_path(&location_id),
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::LocationSetPath {
                        id,
                        location_id,
                        to,
                    } => {
                        let packet = ClientPackets::LocationSetPath(
                            id,
                            self.session.location_set_path(&location_id, to),
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::LocationGetShouldSave { id, location_id } => {
                        let packet = ClientPackets::LocationGetShouldSave(
                            id,
                            self.session.location_get_should_save(&location_id),
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::LocationSetShouldSave {
                        id,
                        location_id,
                        to,
                    } => {
                        let packet = ClientPackets::LocationSetShouldSave(
                            id,
                            self.session.location_set_should_save(&location_id, to),
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::LocationGetElementsLen { id, location_id } => {
                        let packet = ClientPackets::LocationGetElementsLen(
                            id,
                            self.session.location_get_elements_len(&location_id),
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::LocationGetElements {
                        id,
                        location_id,
                        range,
                    } => {
                        let packet = ClientPackets::LocationGetElements(
                            id,
                            match self.session.location_get_elements(&location_id, range) {
                                Ok(ok) => {
                                    let mut tmp = Vec::with_capacity(ok.len());

                                    for k in ok {
                                        tmp.push(k.id())
                                    }

                                    Ok(tmp)
                                }
                                Err(err) => Err(err),
                            },
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::LocationNotify {
                        id,
                        location_id,
                        event,
                    } => {
                        let packet = ClientPackets::LocationNotify(
                            id,
                            self.session.location_notify(&location_id, event),
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::LocationEmit {
                        id,
                        location_id,
                        event,
                    } => {
                        let packet = ClientPackets::LocationEmit(
                            id,
                            self.session.location_emit(&location_id, event),
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::LocationSubscribe {
                        id,
                        location_id,
                        to,
                    } => {
                        let packet = ClientPackets::LocationSubscribe(
                            id,
                            self.session.location_subscribe(&location_id, to),
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::LocationUnSubscribe {
                        id,
                        location_id,
                        to,
                    } => {
                        let packet = ClientPackets::LocationUnSubscribe(
                            id,
                            self.session.location_unsubscribe(&location_id, to),
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::ModuleAcceptedProtocols { id, module_id } => {
                        let packet = ClientPackets::ModuleAcceptedProtocols(
                            id,
                            self.session.module_accepted_protocols(&module_id),
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::ElementGetUrl { id, element_id } => {
                        let packet = ClientPackets::ElementGetUrl(
                            id,
                            self.session.element_get_url(&element_id),
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::ElementSetUrl { id, element_id, to } => {
                        let packet = ClientPackets::ElementSetUrl(
                            id,
                            self.session.element_set_url(&element_id, to),
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::LoadModuleInfo { id, module_info } => {
                        let packet = ClientPackets::LoadModuleInfo(
                            id,
                            self.session
                                .load_module_info(module_info)
                                .map(|_ref| _ref.id()),
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::FindModule { id, module_info } => {
                        let packet = ClientPackets::FindModule(
                            id,
                            self.session.find_module(module_info).map(|_ref| _ref.id()),
                        );
                        self.inner.send(packet, &addr).await
                    }
                    ServerPackets::ModuleGetUid { id, module_id } => {
                        self.inner
                            .send(
                                ClientPackets::ModuleGetUid(
                                    id,
                                    self.session.module_get_uid(&module_id),
                                ),
                                &addr,
                            )
                            .await
                    }
                    ServerPackets::ModuleGetVersion { id, module_id } => {
                        self.inner
                            .send(
                                ClientPackets::ModuleGetVersion(
                                    id,
                                    self.session.module_get_version(&module_id),
                                ),
                                &addr,
                            )
                            .await
                    }
                    ServerPackets::ModuleSupportedVersions { id, module_id } => {
                        self.inner
                            .send(
                                ClientPackets::ModuleSupportedVersions(
                                    id,
                                    self.session.module_supported_versions(&module_id),
                                ),
                                &addr,
                            )
                            .await
                    }
                    ServerPackets::ModuleAcceptedExtensions { id, module_id } => {
                        self.inner
                            .send(
                                ClientPackets::ModuleAcceptedExtensions(
                                    id,
                                    self.session.module_accepted_extensions(&module_id),
                                ),
                                &addr,
                            )
                            .await
                    }
                    ServerPackets::LoadElementInfo { id, element_info } => {
                        self.inner
                            .send(
                                ClientPackets::LoadElementInfo(
                                    id,
                                    self.session
                                        .load_element_info(element_info)
                                        .map(|_ref| _ref.id()),
                                ),
                                &addr,
                            )
                            .await
                    }
                    ServerPackets::LoadLocationInfo { id, location_info } => {
                        self.inner
                            .send(
                                ClientPackets::LoadLocationInfo(
                                    id,
                                    self.session
                                        .load_location_info(location_info)
                                        .map(|_ref| _ref.id()),
                                ),
                                &addr,
                            )
                            .await
                    }
                    ServerPackets::GetVersion { id } => {
                        self.inner
                            .send(
                                ClientPackets::GetVersion(id, self.session.get_version()),
                                &addr,
                            )
                            .await
                    }
                    ServerPackets::GetVersionText { id } => {
                        self.inner
                            .send(
                                ClientPackets::GetVersionText(
                                    id,
                                    self.session.get_version_text().map(|version| {
                                        format!("{version}, Daemon: {DAEMON_VERSION}")
                                    }),
                                ),
                                &addr,
                            )
                            .await
                    }
                    ServerPackets::ModuleGetLocationSettings { id, module_id } => {
                        self.inner
                            .send(
                                ClientPackets::ModuleGetLocationSettings(
                                    id,
                                    self.session.module_get_location_settings(&module_id),
                                ),
                                &addr,
                            )
                            .await
                    }
                    ServerPackets::ModuleSetLocationSettings { id, module_id, to } => {
                        self.inner
                            .send(
                                ClientPackets::ModuleSetLocationSettings(
                                    id,
                                    self.session.module_set_location_settings(&module_id, to),
                                ),
                                &addr,
                            )
                            .await
                    }
                    ServerPackets::ElementIsError { id, element_id } => {
                        self.inner
                            .send(
                                ClientPackets::ElementIsError(
                                    id,
                                    self.session.element_is_error(&element_id),
                                ),
                                &addr,
                            )
                            .await
                    }
                    ServerPackets::LocationGetModule { id, location_id } => {
                        self.inner
                            .send(
                                ClientPackets::LocationGetModule(
                                    id,
                                    self.session.location_get_module(&location_id).map(
                                        |option_module_ref| {
                                            option_module_ref.map(|module_ref| module_ref.id())
                                        },
                                    ),
                                ),
                                &addr,
                            )
                            .await
                    }
                    ServerPackets::LocationSetModule {
                        id,
                        location_id,
                        module_id,
                    } => {
                        self.inner
                            .send(
                                ClientPackets::LocationSetModule(
                                    id,
                                    self.session.location_set_module(&location_id, module_id),
                                ),
                                &addr,
                            )
                            .await
                    }
                    ServerPackets::LocationGetSettings { id, location_id } => {
                        self.inner
                            .send(
                                ClientPackets::LocationGetSettings(
                                    id,
                                    self.session.location_get_settings(&location_id),
                                ),
                                &addr,
                            )
                            .await
                    }
                    ServerPackets::LocationSetSettings {
                        id,
                        location_id,
                        to,
                    } => {
                        self.inner
                            .send(
                                ClientPackets::LocationSetSettings(
                                    id,
                                    self.session.location_set_settings(&location_id, to),
                                ),
                                &addr,
                            )
                            .await
                    }
                    ServerPackets::LocationGetModuleSettings { id, location_id } => {
                        self.inner
                            .send(
                                ClientPackets::LocationGetModuleSettings(
                                    id,
                                    self.session.location_get_module_settings(&location_id),
                                ),
                                &addr,
                            )
                            .await
                    }
                    ServerPackets::LocationSetModuleSettings {
                        id,
                        location_id,
                        to,
                    } => {
                        self.inner
                            .send(
                                ClientPackets::LocationSetModuleSettings(
                                    id,
                                    self.session.location_set_module_settings(&location_id, to),
                                ),
                                &addr,
                            )
                            .await
                    }
                    ServerPackets::LocationGetStatuses { id, location_id } => {
                        self.inner
                            .send(
                                ClientPackets::LocationGetStatuses(
                                    id,
                                    self.session.location_get_statuses(&location_id),
                                ),
                                &addr,
                            )
                            .await
                    }
                    ServerPackets::LocationSetStatuses {
                        id,
                        location_id,
                        statuses,
                    } => {
                        self.inner
                            .send(
                                ClientPackets::LocationSetStatuses(
                                    id,
                                    self.session.location_set_statuses(&location_id, statuses),
                                ),
                                &addr,
                            )
                            .await
                    }
                    ServerPackets::LocationGetStatus { id, location_id } => {
                        self.inner
                            .send(
                                ClientPackets::LocationGetStatus(
                                    id,
                                    self.session.location_get_status(&location_id),
                                ),
                                &addr,
                            )
                            .await
                    }
                    ServerPackets::LocationSetStatus {
                        id,
                        location_id,
                        to,
                    } => {
                        self.inner
                            .send(
                                ClientPackets::LocationSetStatus(
                                    id,
                                    self.session.location_set_status(&location_id, to),
                                ),
                                &addr,
                            )
                            .await
                    }
                    ServerPackets::LocationGetProgress { id, location_id } => {
                        self.inner
                            .send(
                                ClientPackets::LocationGetProgress(
                                    id,
                                    self.session.location_get_progress(&location_id),
                                ),
                                &addr,
                            )
                            .await
                    }
                    ServerPackets::LocationSetProgress {
                        id,
                        location_id,
                        to,
                    } => {
                        self.inner
                            .send(
                                ClientPackets::LocationSetProgress(
                                    id,
                                    self.session.location_set_progress(&location_id, to),
                                ),
                                &addr,
                            )
                            .await
                    }
                    ServerPackets::LocationIsEnabled { id, location_id } => {
                        self.inner
                            .send(
                                ClientPackets::LocationIsEnabled(
                                    id,
                                    self.session.location_is_enabled(&location_id),
                                ),
                                &addr,
                            )
                            .await
                    }
                    ServerPackets::LocationSetEnabled {
                        id,
                        location_id,
                        to,
                    } => {
                        self.inner
                            .send(
                                ClientPackets::LocationSetEnabled(
                                    id,
                                    self.session.location_set_enabled(&location_id, to, None),
                                ),
                                &addr,
                            )
                            .await
                    }
                    ServerPackets::LocationIsError { id, location_id } => {
                        self.inner
                            .send(
                                ClientPackets::LocationIsError(
                                    id,
                                    self.session.location_is_error(&location_id),
                                ),
                                &addr,
                            )
                            .await
                    }
                }
            }
        }
    }
}

#[async_trait]
trait TDaemonInner {
    async fn send(&self, packet: ClientPackets, to: &SocketAddr);
    async fn recv(&self) -> Vec<(SocketAddr, Vec<ServerPackets>)>;
    // garbage collect clients
    async fn gc_clients(&self);
    async fn clients(&self) -> Vec<SocketAddr>;
}

impl DaemonInner {
    fn get_inner(&mut self) -> (&mut [u8], &UdpSocket) {
        (&mut self.buffer, &self.socket)
    }
}

#[async_trait]
impl TDaemonInner for Arc<Mutex<DaemonInner>> {
    async fn send(&self, packet: ClientPackets, to: &SocketAddr) {
        log::trace!("Send: {}, Packet: {:?}", to, packet);
        let mut bytes = packet.to_bytes();
        bytes.reverse();

        let inner = self.lock().await;
        let socket = inner.socket.clone();

        for chunk in bytes.chunks(4096) {
            let _ = socket.send_to(chunk, to).await;
        }
    }

    async fn recv(&self) -> Vec<(SocketAddr, Vec<ServerPackets>)> {
        let mut inner = self.lock().await;

        let mut master_buffer: HashMap<SocketAddr, Vec<u8>> = HashMap::new();
        let (buffer, socket) = inner.get_inner();

        while let Ok((len, from)) = socket.try_recv_from(buffer) {
            let buf = master_buffer.get_mut(&from);
            match buf {
                Some(buf) => buf.append(&mut buffer[0..len].to_vec()),
                None => {
                    master_buffer.insert(from, buffer[0..len].to_vec());
                }
            }
        }

        master_buffer
            .drain()
            .map(|(from, mut buffer)| {
                let mut packets = Vec::new();

                let mut finded = false;
                for client in inner.clients.iter_mut() {
                    if client.1 == from {
                        client.0 = SystemTime::now();
                        finded = true;
                        break;
                    }
                }

                if !finded {
                    inner.clients.push((SystemTime::now(), from))
                }
                while !buffer.is_empty() {
                    let Some(packet) = ServerPackets::from_bytes(&mut buffer) else{continue};
                    log::trace!("From: {}, Packet: {:?}", from, packet);
                    packets.push(packet)
                }
                (from, packets)
            })
            .collect()
    }

    async fn gc_clients(&self) {
        self.lock()
            .await
            .clients
            .retain(|(time, _)| time.elapsed().unwrap() < CLIENT_TIMEOUT);
    }

    async fn clients(&self) -> Vec<SocketAddr> {
        self.gc_clients().await;

        log::trace!("Clients: {:?}", self.lock().await.clients);
        self.lock()
            .await
            .clients
            .iter()
            .map(|(_, addr)| *addr)
            .collect::<Vec<SocketAddr>>()
    }
}
