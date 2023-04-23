use std::{
    net::UdpSocket,
    ops::{AddAssign, Sub},
    sync::{Arc, RwLock},
    thread::{self, JoinHandle},
    time::{Duration, SystemTime},
};

use bytes_kman::TBytes;
use muzzman_lib::prelude::*;
use packets::{ClientPackets, ServerPackets};

pub const DAEMON_VERSION: u64 = 1;

pub mod common;
pub mod daemon;
pub mod packets;
pub mod row;
pub mod session;

pub const DAEMON_PORT: u16 = 2118;

pub const TIMEOUT: Duration = Duration::new(3, 0);

pub mod prelude {
    pub use crate::common::get_modules;
    pub use crate::DaemonSession;
    pub use muzzman_lib::prelude::*;
}

pub struct DaemonSession {
    pub conn: UdpSocket,
    pub packets: Vec<ClientPackets>,
    pub generator: u128,
    pub locations_refs: Vec<LRef>,
    pub element_refs: Vec<ERef>,
    pub module_refs: Vec<MRef>,
    pub watcher_thread: JoinHandle<()>,
}

unsafe impl Send for DaemonSession {}
unsafe impl Sync for DaemonSession {}

impl DaemonSession {
    pub fn new() -> Result<Self, std::io::Error> {
        let conn = UdpSocket::bind("127.0.0.1:0")?;
        conn.connect(format!("127.0.0.1:{DAEMON_PORT}"))?;
        let _ = conn.set_nonblocking(true);
        let _ = conn.set_read_timeout(Some(TIMEOUT));
        Ok(Self {
            conn,
            packets: Vec::new(),
            generator: 1,
            locations_refs: Vec::new(),
            element_refs: Vec::new(),
            module_refs: Vec::new(),
            watcher_thread: thread::spawn(|| {}),
        })
    }

    pub fn pull_packets(&mut self) {
        let mut buffer = [0; 4096];

        let mut master_buffer = Vec::new();

        while let Ok(len) = self.conn.recv(&mut buffer) {
            master_buffer.append(&mut buffer[0..len].to_vec())
        }

        println!("{}", master_buffer.len());

        while !master_buffer.is_empty() {
            if let Some(packet) = ClientPackets::from_bytes(&mut master_buffer) {
                if let ClientPackets::NewSessionEvent(event) = packet {
                    match event {
                        SessionEvent::DestroyedElement(id) => {
                            self.element_refs.retain(|eref| eref.id() != id);
                        }
                        SessionEvent::DestroyedLocation(id) => {
                            self.locations_refs.retain(|lref| lref.id() != id)
                        }
                        SessionEvent::DestroyedModule(id) => {
                            self.module_refs.retain(|mref| mref.id() != id)
                        }
                        SessionEvent::ElementIdChanged(last, new) => {
                            for eref in self.element_refs.iter_mut() {
                                if eref.id() == last {
                                    eref.write().unwrap().id = new.clone();
                                    break;
                                }
                            }
                        }
                        SessionEvent::LocationIdChanged(last, new) => {
                            for lref in self.locations_refs.iter_mut() {
                                if lref.id() == last {
                                    lref.write().unwrap().id = new;
                                    break;
                                }
                            }
                        }
                        SessionEvent::ModuleIdChanged(last, new) => {
                            for mref in self.module_refs.iter_mut() {
                                if mref.id() == last {
                                    mref.write().unwrap().uid = new;
                                }
                            }
                        }
                        _ => {}
                    }
                } else {
                    self.packets.push(packet)
                }
            }
        }
    }

    pub fn gc_refs(&mut self) {
        self.module_refs.retain(|mref| {
            let count = Arc::strong_count(mref);
            count > 1
        });

        self.locations_refs.retain(|lref| {
            let count = Arc::strong_count(lref);
            count > 1
        });

        self.element_refs.retain(|eref| {
            let count = Arc::strong_count(eref);
            count > 1
        });
    }

    pub fn create_session(self) -> Box<dyn TSession> {
        let s = Arc::new(RwLock::new(self));

        let sc = s.clone();

        s.write().unwrap().watcher_thread = thread::spawn(move || {
            let sc = sc;
            loop {
                thread::sleep(Duration::new(1, 0));
                let count = Arc::strong_count(&sc);
                sc.write().unwrap().gc_refs();
                if count == 1 {
                    break;
                }
                sc.pull_packets();
                let mut bytes = ServerPackets::Tick.to_bytes();
                bytes.reverse();
                let _ = sc.read().unwrap().conn.send(&bytes);
            }
        });

        Box::new(Box::new(s) as Box<dyn TDaemonSession>)
    }
}

pub trait TDaemonSession {
    fn pull_packets(&self);

    fn waiting_for(&self, id: u128) -> Option<ClientPackets>;
    fn send(&self, packet: ServerPackets);
    fn generate(&self) -> u128;

    fn eref_get_or_add(&self, element_id: ElementId) -> ERef;
    fn lref_get_or_add(&self, location_id: LocationId) -> LRef;
    fn mref_get_or_add(&self, module_id: ModuleId) -> MRef;

    fn cl(&self) -> Box<dyn TDaemonSession>;
}

impl TDaemonSession for Arc<RwLock<DaemonSession>> {
    fn pull_packets(&self) {
        self.write().unwrap().pull_packets()
    }

    fn waiting_for(&self, id: u128) -> Option<ClientPackets> {
        let start_time = SystemTime::now();
        let mut index = None;
        'm: loop {
            if start_time.elapsed().unwrap() > TIMEOUT {
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

        for chunk in bytes.chunks(4096) {
            let _ = self.write().unwrap().conn.send(chunk);
        }
    }

    fn generate(&self) -> u128 {
        self.write().unwrap().generator.add_assign(1);
        self.read().unwrap().generator.sub(1)
    }

    fn eref_get_or_add(&self, element_id: ElementId) -> ERef {
        for eref in self.read().unwrap().element_refs.iter() {
            if eref.id() == element_id {
                return eref.clone();
            }
        }

        let eref = Arc::new(RwLock::new(RefElement {
            session: Some(Box::new(self.cl())),
            id: element_id,
        }));

        self.write().unwrap().element_refs.push(eref.clone());
        eref
    }

    fn lref_get_or_add(&self, location_id: LocationId) -> LRef {
        for lref in self.read().unwrap().locations_refs.iter() {
            if lref.id() == location_id {
                return lref.clone();
            }
        }

        let lref = Arc::new(RwLock::new(RefLocation {
            session: Some(Box::new(self.cl())),
            id: location_id,
        }));

        self.write().unwrap().locations_refs.push(lref.clone());
        lref
    }

    fn mref_get_or_add(&self, module_id: ModuleId) -> MRef {
        for mref in self.read().unwrap().module_refs.iter() {
            if mref.id() == module_id {
                return mref.clone();
            }
        }

        let mref = Arc::new(RwLock::new(RefModule {
            session: Some(Box::new(self.cl())),
            uid: module_id,
        }));

        self.write().unwrap().module_refs.push(mref.clone());
        mref
    }

    fn cl(&self) -> Box<dyn TDaemonSession> {
        Box::new(self.clone())
    }
}
