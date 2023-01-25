use std::{
    net::{SocketAddr, UdpSocket},
    os::fd::AsRawFd,
    str::FromStr,
    sync::{Arc, Mutex},
    time::{Duration, SystemTime},
};

use bytes_kman::TBytes;
use muzzman_daemon::packets::{ClientPackets, ServerPackets};
use muzzman_lib::{
    prelude::{SessionEvent, TElement, TLocation, TModuleInfo},
    session::TSession,
};
use polling::Poller;

fn main() {
    let daemon = Daemon::new().unwrap();
    daemon.run();
}

pub struct DaemonInner {
    socket: UdpSocket,
    clients: Vec<(SystemTime, SocketAddr)>,
    buffer: [u8; 4096],
}

pub struct Daemon {
    session: Box<dyn TSession>,
    socket_fd: i32,
    poller: Poller,
    inner: Arc<Mutex<DaemonInner>>,
}

unsafe impl Sync for Daemon {}
unsafe impl Send for Daemon {}

impl Daemon {
    pub fn new() -> Result<Self, std::io::Error> {
        let socket_addr = SocketAddr::from_str("0.0.0.0:2118").unwrap();
        let socket = UdpSocket::bind(socket_addr).unwrap();
        socket.set_nonblocking(true).unwrap();

        let socket_fd = socket.as_raw_fd();

        let mut poller = Poller::new()?;
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
            println!("Recived Event from localsession: {:?}", event);
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
                        self.session.element_get_element_info(&element_id),
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
                ServerPackets::Tick => {}
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
            packets.push(packet)
        }

        Some((from, packets))
    }

    fn gc_clients(&self) {
        self.lock()
            .unwrap()
            .clients
            .retain(|(time, _)| time.elapsed().unwrap() > Duration::new(5, 0));
    }

    fn clients(&self) -> Vec<SocketAddr> {
        self.gc_clients();
        self.lock()
            .unwrap()
            .clients
            .iter()
            .map(|(_, addr)| addr.clone())
            .collect::<Vec<SocketAddr>>()
    }
}
