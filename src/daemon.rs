use std::{
    net::{SocketAddr, UdpSocket},
    sync::{Arc, Mutex},
    time::{Duration, SystemTime},
};

const CLIENT_TIMEOUT: Duration = Duration::new(3, 0);

use crate::{
    packets::{ClientPackets, ServerPackets},
    row::{IntoRawSock, RawSock},
    DAEMON_PORT, DAEMON_VERSION,
};
use bytes_kman::TBytes;
use muzzman_lib::{
    prelude::{TElement, TLocation, TModuleInfo},
    session::TSession,
};
use polling::Poller;

pub struct DaemonInner {
    socket: UdpSocket,
    clients: Vec<(SystemTime, SocketAddr)>,
    buffer: [u8; 4096],
}

pub struct Daemon {
    session: Box<dyn TSession>,
    socket_fd: RawSock,
    poller: Poller,
    inner: Arc<Mutex<DaemonInner>>,
}

unsafe impl Sync for Daemon {}
unsafe impl Send for Daemon {}

impl Daemon {
    pub fn new() -> Result<Self, std::io::Error> {
        let socket = UdpSocket::bind(format!("127.0.0.1:{DAEMON_PORT}")).unwrap();
        socket.set_nonblocking(true).unwrap();

        let socket_clone = socket.try_clone().unwrap();
        let socket_fd = socket.into_raw();
        let socket = socket_clone;

        let poller = Poller::new()?;
        poller.add(
            socket_fd,
            polling::Event {
                key: 0,
                readable: true,
                writable: false,
            },
        )?;
        let socket = socket;
        let mut session = muzzman_lib::LocalSession::default();

        let inner = Arc::new(Mutex::new(DaemonInner {
            socket,
            clients: Vec::new(),
            buffer: [0; 4096],
        }));

        let inner_clone = inner.clone();
        session.callback = Some(Box::new(move |event| {
            let clients = inner_clone.clients();
            for client in clients {
                inner_clone.send(ClientPackets::NewSessionEvent(event.clone()), &client);
            }
        }));
        let session = session.new_session();
        let default_location = session.get_default_location().unwrap();
        default_location
            .set_path(dirs::home_dir().unwrap().join("Downloads"))
            .unwrap();

        Ok(Self {
            session,
            inner,
            poller,
            socket_fd,
        })
    }

    pub fn run(mut self) {
        let mut events = Vec::new();
        events.push(polling::Event {
            key: 0,
            readable: false,
            writable: false,
        });
        loop {
            self.poller.wait(&mut events, None).unwrap();
            for event in events.iter() {
                if event.key == 0 {
                    self.poller
                        .modify(
                            self.socket_fd,
                            polling::Event {
                                key: 0,
                                readable: true,
                                writable: false,
                            },
                        )
                        .unwrap();
                    self.respond_to_requests();
                }
            }
        }
    }

    fn respond_to_requests(&mut self) {
        let Some((addr, packets)) = self.inner.recv() else{
                return
            };
        for packet in packets {
            match packet {
                ServerPackets::Tick => {}
                ServerPackets::GetDefaultLocation { id } => {
                    let packet = match self.session.get_default_location() {
                        Ok(ok) => ClientPackets::GetDefaultLocation(id, Ok(ok.id())),
                        Err(err) => ClientPackets::GetDefaultLocation(id, Err(err)),
                    };
                    self.inner.send(packet, &addr);
                }
                ServerPackets::LocationGetName { id, from } => {
                    let packet =
                        ClientPackets::LocationGetName(id, self.session.location_get_name(&from));
                    self.inner.send(packet, &addr);
                }
                ServerPackets::LocationSetName { id, from, to } => {
                    let packet = ClientPackets::LocationSetName(
                        id,
                        self.session.location_set_name(&from, &to),
                    );
                    self.inner.send(packet, &addr)
                }
                ServerPackets::LocationGetDesc { id, from } => {
                    let packet =
                        ClientPackets::LocationGetDesc(id, self.session.location_get_desc(&from));

                    self.inner.send(packet, &addr)
                }
                ServerPackets::LocationSetDesc { id, from, to } => {
                    let packet = ClientPackets::LocationSetDesc(
                        id,
                        self.session.location_set_desc(&from, &to),
                    );
                    self.inner.send(packet, &addr)
                }
                ServerPackets::LocationGetInfo { id, from } => {
                    let packet = ClientPackets::LocationGetInfo(
                        id,
                        self.session.location_get_location_info(&from),
                    );
                    self.inner.send(packet, &addr)
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
                    self.inner.send(packet, &addr)
                }
                ServerPackets::ElementGetName { id, element_id } => {
                    let packet = ClientPackets::ElementGetName(
                        id,
                        self.session.element_get_name(&element_id),
                    );
                    self.inner.send(packet, &addr);
                }
                ServerPackets::ElementSetName { id, element_id, to } => {
                    let packet = ClientPackets::ElementSetName(
                        id,
                        self.session.element_set_name(&element_id, &to),
                    );
                    self.inner.send(packet, &addr);
                }
                ServerPackets::ElementGetDesc { id, element_id } => {
                    let packet = ClientPackets::ElementGetDesc(
                        id,
                        self.session.element_get_desc(&element_id),
                    );
                    self.inner.send(packet, &addr);
                }
                ServerPackets::ElementSetDesc { id, element_id, to } => {
                    let packet = ClientPackets::ElementSetDesc(
                        id,
                        self.session.element_set_desc(&element_id, &to),
                    );
                    self.inner.send(packet, &addr);
                }
                ServerPackets::ElementGetMeta { id, element_id } => {
                    let packet = ClientPackets::ElementGetMeta(
                        id,
                        self.session.element_get_meta(&element_id),
                    );
                    self.inner.send(packet, &addr);
                }
                ServerPackets::ElementSetMeta { id, element_id, to } => {
                    let packet = ClientPackets::ElementSetMeta(
                        id,
                        self.session.element_set_meta(&element_id, &to),
                    );
                    self.inner.send(packet, &addr);
                }
                ServerPackets::ElementGetInfo { id, element_id } => {
                    let packet = ClientPackets::ElementGetInfo(
                        id,
                        Box::new(self.session.element_get_element_info(&element_id)),
                    );
                    self.inner.send(packet, &addr);
                }
                ServerPackets::LoadModule { id, path } => {
                    let packet = ClientPackets::LoadModule(
                        id,
                        match self.session.load_module(path) {
                            Ok(ok) => Ok(ok.id()),
                            Err(err) => Err(err),
                        },
                    );
                    self.inner.send(packet, &addr);
                }
                ServerPackets::RemoveModule { id, module_id } => {
                    let packet = ClientPackets::RemoveModule(
                        id,
                        match self.session.remove_module(module_id) {
                            Ok(_) => Ok(()),
                            Err(err) => Err(err),
                        },
                    );
                    self.inner.send(packet, &addr);
                }
                ServerPackets::GetActionsLen { id } => {
                    let packet = ClientPackets::GetActionsLen(id, self.session.get_actions_len());
                    self.inner.send(packet, &addr);
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
                    self.inner.send(packet, &addr);
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
                    self.inner.send(packet, &addr);
                }
                ServerPackets::GetModulesLen { id } => {
                    let packet = ClientPackets::GetModulesLen(id, self.session.get_modules_len());
                    self.inner.send(packet, &addr);
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
                    self.inner.send(packet, &addr);
                }
                ServerPackets::ModuleGetName { id, module_id } => {
                    let packet =
                        ClientPackets::ModuleGetName(id, self.session.module_get_name(&module_id));
                    self.inner.send(packet, &addr)
                }
                ServerPackets::ModuleSetName { id, module_id, to } => {
                    let packet = ClientPackets::ModuleSetName(
                        id,
                        self.session.module_set_name(&module_id, to),
                    );
                    self.inner.send(packet, &addr);
                }
                ServerPackets::ModuleGetDefaultName { id, module_id } => {
                    let packet = ClientPackets::ModuleGetDefaultName(
                        id,
                        self.session.module_get_default_name(&module_id),
                    );
                    self.inner.send(packet, &addr)
                }
                ServerPackets::ModuleGetDesc { id, module_id } => {
                    let packet =
                        ClientPackets::ModuleGetDesc(id, self.session.module_get_desc(&module_id));
                    self.inner.send(packet, &addr)
                }
                ServerPackets::ModuleSetDesc { id, module_id, to } => {
                    let packet = ClientPackets::ModuleSetDesc(
                        id,
                        self.session.module_set_desc(&module_id, to),
                    );
                    self.inner.send(packet, &addr);
                }
                ServerPackets::ModuleGetDefaultDesc { id, module_id } => {
                    let packet = ClientPackets::ModuleGetDefaultDesc(
                        id,
                        self.session.module_get_default_desc(&module_id),
                    );
                    self.inner.send(packet, &addr)
                }
                ServerPackets::ModuleGetProxy { id, module_id } => {
                    let packet = ClientPackets::ModuleGetProxy(
                        id,
                        self.session.module_get_proxy(&module_id),
                    );
                    self.inner.send(packet, &addr)
                }
                ServerPackets::ModuleSetProxy { id, module_id, to } => {
                    let packet = ClientPackets::ModuleSetProxy(
                        id,
                        self.session.module_set_proxy(&module_id, to),
                    );
                    self.inner.send(packet, &addr)
                }
                ServerPackets::ModuleGetSettings { id, module_id } => {
                    let packet = ClientPackets::ModuleGetSettings(
                        id,
                        Box::new(self.session.module_get_settings(&module_id)),
                    );
                    self.inner.send(packet, &addr)
                }
                ServerPackets::ModuleSetSettings { id, module_id, to } => {
                    let packet = ClientPackets::ModuleSetSettings(
                        id,
                        self.session.module_set_settings(&module_id, to),
                    );
                    self.inner.send(packet, &addr)
                }
                ServerPackets::ModuleGetElementSettings { id, module_id } => {
                    let packet = ClientPackets::ModuleGetElementSettings(
                        id,
                        self.session.module_get_element_settings(&module_id),
                    );
                    self.inner.send(packet, &addr);
                }
                ServerPackets::ModuleSetElementSettings { id, module_id, to } => {
                    let packet = ClientPackets::ModuleSetElementSettings(
                        id,
                        self.session.module_set_element_settings(&module_id, to),
                    );
                    self.inner.send(packet, &addr)
                }
                ServerPackets::ModuleInitLocation {
                    id,
                    module_id,
                    location_id,
                    data,
                } => {
                    let packet = ClientPackets::ModuleInitLocation(
                        id,
                        self.session
                            .module_init_location(&module_id, &location_id, data),
                    );
                    self.inner.send(packet, &addr)
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
                    self.inner.send(packet, &addr)
                }
                ServerPackets::ModuleAcceptUrl { id, module_id, url } => {
                    let packet = ClientPackets::ModuleAcceptUrl(
                        id,
                        self.session.module_accept_url(&module_id, url),
                    );
                    self.inner.send(packet, &addr)
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
                    self.inner.send(packet, &addr);
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
                    self.inner.send(packet, &addr)
                }
                ServerPackets::DestroyElement { id, element_id } => {
                    let packet = ClientPackets::DestroyElement(
                        id,
                        match self.session.destroy_element(element_id) {
                            Ok(_) => Ok(()),
                            Err(err) => Err(err),
                        },
                    );
                    self.inner.send(packet, &addr)
                }
                ServerPackets::ElementGetElementData { id, element_id } => {
                    let packet = ClientPackets::ElementGetElementData(
                        id,
                        self.session.element_get_element_data(&element_id),
                    );
                    self.inner.send(packet, &addr)
                }
                ServerPackets::ElementSetElementData { id, element_id, to } => {
                    let packet = ClientPackets::ElementSetElementData(
                        id,
                        self.session.element_set_element_data(&element_id, to),
                    );
                    self.inner.send(packet, &addr)
                }
                ServerPackets::ElementGetModuleData { id, element_id } => {
                    let packet = ClientPackets::ElementGetModuleData(
                        id,
                        self.session.element_get_module_data(&element_id),
                    );
                    self.inner.send(packet, &addr)
                }
                ServerPackets::ElementSetModuleData { id, element_id, to } => {
                    let packet = ClientPackets::ElementSetModuleData(
                        id,
                        self.session.element_set_module_data(&element_id, to),
                    );
                    self.inner.send(packet, &addr)
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
                    self.inner.send(packet, &addr)
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
                    self.inner.send(packet, &addr)
                }
                ServerPackets::ElementGetStatuses { id, element_id } => {
                    let packet = ClientPackets::ElementGetStatuses(
                        id,
                        self.session.element_get_statuses(&element_id),
                    );
                    self.inner.send(packet, &addr)
                }
                ServerPackets::ElementSetStatuses { id, element_id, to } => {
                    let packet = ClientPackets::ElementSetStatuses(
                        id,
                        self.session.element_set_statuses(&element_id, to),
                    );
                    self.inner.send(packet, &addr)
                }
                ServerPackets::ElementGetStatus { id, element_id } => {
                    let packet = ClientPackets::ElementGetStatus(
                        id,
                        self.session.element_get_status(&element_id),
                    );
                    self.inner.send(packet, &addr)
                }
                ServerPackets::ElementSetStatus { id, element_id, to } => {
                    let packet = ClientPackets::ElementSetStatus(
                        id,
                        self.session.element_set_status(&element_id, to),
                    );
                    self.inner.send(packet, &addr)
                }
                ServerPackets::ElementGetData { id, element_id } => {
                    let packet = ClientPackets::ElementGetData(
                        id,
                        self.session.element_get_data(&element_id),
                    );
                    self.inner.send(packet, &addr)
                }
                ServerPackets::ElementSetData { id, element_id, to } => {
                    let packet = ClientPackets::ElementSetData(
                        id,
                        self.session.element_set_data(&element_id, to),
                    );
                    self.inner.send(packet, &addr)
                }
                ServerPackets::ElementGetProgress { id, element_id } => {
                    let packet = ClientPackets::ElementGetProgress(
                        id,
                        self.session.element_get_progress(&element_id),
                    );
                    self.inner.send(packet, &addr)
                }
                ServerPackets::ElementSetProgress { id, element_id, to } => {
                    let packet = ClientPackets::ElementSetProgress(
                        id,
                        self.session.element_set_progress(&element_id, to),
                    );
                    self.inner.send(packet, &addr)
                }
                ServerPackets::ElementGetShouldSave { id, element_id } => {
                    let packet = ClientPackets::ElementGetShouldSave(
                        id,
                        self.session.element_get_should_save(&element_id),
                    );
                    self.inner.send(packet, &addr)
                }
                ServerPackets::ElementSetShouldSave { id, element_id, to } => {
                    let packet = ClientPackets::ElementSetShouldSave(
                        id,
                        self.session.element_set_should_save(&element_id, to),
                    );
                    self.inner.send(packet, &addr)
                }
                ServerPackets::ElementGetEnabled { id, element_id } => {
                    let packet = ClientPackets::ElementGetEnabled(
                        id,
                        self.session.element_get_enabled(&element_id),
                    );
                    self.inner.send(packet, &addr)
                }
                ServerPackets::ElementSetEnabled { id, element_id, to } => {
                    let packet = ClientPackets::ElementSetEnabled(
                        id,
                        self.session.element_set_enabled(&element_id, to, None),
                    );
                    self.inner.send(packet, &addr)
                }
                ServerPackets::ElementResolvModule { id, element_id } => {
                    let packet = ClientPackets::ElementResolvModule(
                        id,
                        self.session.element_resolv_module(&element_id),
                    );
                    self.inner.send(packet, &addr)
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
                    self.inner.send(packet, &addr)
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
                    self.inner.send(packet, &addr)
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
                    self.inner.send(packet, &addr)
                }
                ServerPackets::ElementSubscribe { id, element_id, to } => {
                    let packet = ClientPackets::ElementSubscribe(
                        id,
                        self.session.element_subscribe(&element_id, to),
                    );
                    self.inner.send(packet, &addr)
                }
                ServerPackets::ElementUnSubscribe { id, element_id, to } => {
                    let packet = ClientPackets::ElementUnSubscribe(
                        id,
                        self.session.element_unsubscribe(&element_id, to),
                    );
                    self.inner.send(packet, &addr)
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
                    self.inner.send(packet, &addr)
                }
                ServerPackets::GetLocationsLen { id, location_id } => {
                    let packet = ClientPackets::GetLocationsLen(
                        id,
                        self.session.get_locations_len(&location_id),
                    );
                    self.inner.send(packet, &addr)
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
                    self.inner.send(packet, &addr)
                }
                ServerPackets::DestroyLocation { id, location_id } => {
                    let packet = ClientPackets::DestroyLocation(
                        id,
                        match self.session.destroy_location(location_id) {
                            Ok(_) => Ok(()),
                            Err(err) => Err(err),
                        },
                    );
                    self.inner.send(packet, &addr)
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
                    self.inner.send(packet, &addr)
                }
                ServerPackets::LocationGetPath { id, location_id } => {
                    let packet = ClientPackets::LocationGetPath(
                        id,
                        self.session.location_get_path(&location_id),
                    );
                    self.inner.send(packet, &addr)
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
                    self.inner.send(packet, &addr)
                }
                ServerPackets::LocationGetShouldSave { id, location_id } => {
                    let packet = ClientPackets::LocationGetShouldSave(
                        id,
                        self.session.location_get_should_save(&location_id),
                    );
                    self.inner.send(packet, &addr)
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
                    self.inner.send(packet, &addr)
                }
                ServerPackets::LocationGetElementsLen { id, location_id } => {
                    let packet = ClientPackets::LocationGetElementsLen(
                        id,
                        self.session.location_get_elements_len(&location_id),
                    );
                    self.inner.send(packet, &addr)
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
                    self.inner.send(packet, &addr)
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
                    self.inner.send(packet, &addr)
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
                    self.inner.send(packet, &addr)
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
                    self.inner.send(packet, &addr)
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
                    self.inner.send(packet, &addr)
                }
                ServerPackets::ModuleAcceptedProtocols { id, module_id } => {
                    let packet = ClientPackets::ModuleAcceptedProtocols(
                        id,
                        self.session.module_accepted_protocols(&module_id),
                    );
                    self.inner.send(packet, &addr)
                }
                ServerPackets::ElementGetUrl { id, element_id } => {
                    let packet =
                        ClientPackets::ElementGetUrl(id, self.session.element_get_url(&element_id));
                    self.inner.send(packet, &addr)
                }
                ServerPackets::ElementSetUrl { id, element_id, to } => {
                    let packet = ClientPackets::ElementSetUrl(
                        id,
                        self.session.element_set_url(&element_id, to),
                    );
                    self.inner.send(packet, &addr)
                }
                ServerPackets::LoadModuleInfo { id, module_info } => {
                    let packet = ClientPackets::LoadModuleInfo(
                        id,
                        self.session
                            .load_module_info(module_info)
                            .map(|_ref| _ref.id()),
                    );
                    self.inner.send(packet, &addr)
                }
                ServerPackets::FindModule { id, module_info } => {
                    let packet = ClientPackets::FindModule(
                        id,
                        self.session.find_module(module_info).map(|_ref| _ref.id()),
                    );
                    self.inner.send(packet, &addr)
                }
                ServerPackets::ModuleGetUid { id, module_id } => {
                    self.inner.send(
                        ClientPackets::ModuleGetUid(id, self.session.module_get_uid(&module_id)),
                        &addr,
                    );
                }
                ServerPackets::ModuleGetVersion { id, module_id } => {
                    self.inner.send(
                        ClientPackets::ModuleGetVersion(
                            id,
                            self.session.module_get_version(&module_id),
                        ),
                        &addr,
                    );
                }
                ServerPackets::ModuleSupportedVersions { id, module_id } => {
                    self.inner.send(
                        ClientPackets::ModuleSupportedVersions(
                            id,
                            self.session.module_supported_versions(&module_id),
                        ),
                        &addr,
                    );
                }
                ServerPackets::ModuleAcceptedExtensions { id, module_id } => {
                    self.inner.send(
                        ClientPackets::ModuleAcceptedExtensions(
                            id,
                            self.session.module_accepted_extensions(&module_id),
                        ),
                        &addr,
                    );
                }
                ServerPackets::LoadElementInfo { id, element_info } => self.inner.send(
                    ClientPackets::LoadElementInfo(
                        id,
                        self.session
                            .load_element_info(element_info)
                            .map(|_ref| _ref.id()),
                    ),
                    &addr,
                ),
                ServerPackets::LoadLocationInfo { id, location_info } => self.inner.send(
                    ClientPackets::LoadLocationInfo(
                        id,
                        self.session
                            .load_location_info(location_info)
                            .map(|_ref| _ref.id()),
                    ),
                    &addr,
                ),
                ServerPackets::GetVersion { id } => self.inner.send(
                    ClientPackets::GetVersion(id, self.session.get_version()),
                    &addr,
                ),
                ServerPackets::GetVersionText { id } => self.inner.send(
                    ClientPackets::GetVersionText(
                        id,
                        self.session
                            .get_version_text()
                            .map(|version| format!("{version}, Daemon: {DAEMON_VERSION}")),
                    ),
                    &addr,
                ),
            }
        }
    }
}

trait TDaemonInner {
    fn send(&self, packet: ClientPackets, to: &SocketAddr);
    fn recv(&self) -> Option<(SocketAddr, Vec<ServerPackets>)>;
    // garbage collect clients
    fn gc_clients(&self);
    fn clients(&self) -> Vec<SocketAddr>;
}

impl DaemonInner {
    fn get_inner(&mut self) -> (&mut [u8], &mut UdpSocket) {
        (&mut self.buffer, &mut self.socket)
    }
}

impl TDaemonInner for Arc<Mutex<DaemonInner>> {
    fn send(&self, packet: ClientPackets, to: &SocketAddr) {
        log::trace!("Send: {}, Packet: {:?}", to, packet);
        let mut bytes = packet.to_bytes();
        bytes.reverse();
        let _ = self.lock().unwrap().socket.send_to(&bytes, to);
    }

    fn recv(&self) -> Option<(SocketAddr, Vec<ServerPackets>)> {
        let mut inner = self.lock().unwrap();

        let (buffer, socket) = inner.get_inner();

        let Ok((len, from)) = socket.recv_from(buffer) else{return None};

        {
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
        }

        let mut buffer = inner.buffer[0..len].to_vec();
        let mut packets = Vec::new();
        while !buffer.is_empty() {
            let Some(packet) = ServerPackets::from_bytes(&mut buffer) else{continue};
            log::trace!("From: {}, Packet: {:?}", from, packet);
            packets.push(packet)
        }

        Some((from, packets))
    }

    fn gc_clients(&self) {
        self.lock()
            .unwrap()
            .clients
            .retain(|(time, _)| time.elapsed().unwrap() < CLIENT_TIMEOUT);
    }

    fn clients(&self) -> Vec<SocketAddr> {
        self.gc_clients();

        log::trace!("Clients: {:?}", self.lock().unwrap().clients);
        self.lock()
            .unwrap()
            .clients
            .iter()
            .map(|(_, addr)| *addr)
            .collect::<Vec<SocketAddr>>()
    }
}
