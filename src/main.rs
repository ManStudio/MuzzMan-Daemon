use std::{net::SocketAddr, str::FromStr};

use bytes_kman::TBytes;
use muzzman_daemon::packets::{ClientPackets, ServerPackets};
use muzzman_lib::{prelude::TLocation, session::TSession};

fn main() {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(tokio_main());
}

pub struct Daemon {
    session: Box<dyn TSession>,
}

unsafe impl Sync for Daemon {}
unsafe impl Send for Daemon {}

impl Default for Daemon {
    fn default() -> Self {
        let session = muzzman_lib::local_session::session::LocalSession::new_session();
        let default_location = session.get_default_location().unwrap();
        default_location
            .set_path(dirs::home_dir().unwrap().join("Downloads"))
            .unwrap();
        Self { session }
    }
}

impl Daemon {
    pub async fn run(self) {
        let socket_addr = SocketAddr::from_str("0.0.0.0:2118").unwrap();
        let socket = tokio::net::UdpSocket::bind(socket_addr).await.unwrap();

        let mut buffer = vec![0; 4096];

        loop {
            let (len, addr) = socket.recv_from(&mut buffer).await.unwrap();
            let mut buffer = buffer[0..len].to_vec();
            while !buffer.is_empty() {
                let Some(packet) = ServerPackets::from_bytes(&mut buffer)else{continue};
                println!("Recived Packet: {:?}", packet);
                match packet {
                    ServerPackets::GetDefaultLocation { id } => {
                        let packet = match self.session.get_default_location() {
                            Ok(ok) => ClientPackets::GetDefaultLocation(id, Ok(ok.id())),
                            Err(err) => ClientPackets::GetDefaultLocation(id, Err(err)),
                        };
                        let mut bytes = packet.to_bytes();
                        bytes.reverse();
                        let _ = socket.send_to(&bytes, addr).await;
                    }
                    ServerPackets::GetLocationName { id, from } => {
                        let packet = match self.session.location_get_name(&from) {
                            Ok(ok) => ClientPackets::GetLocationName(id, Ok(ok)),
                            Err(err) => ClientPackets::GetLocationName(id, Err(err)),
                        };
                        let mut bytes = packet.to_bytes();
                        bytes.reverse();
                        let _ = socket.send_to(&bytes, addr).await;
                    }
                    ServerPackets::SetLocationName { id, from, to } => {
                        let packet = match self.session.location_set_name(&from, &to) {
                            Ok(ok) => ClientPackets::SetLocationName(id, Ok(ok)),
                            Err(err) => ClientPackets::SetLocationName(id, Err(err)),
                        };
                        let mut bytes = packet.to_bytes();
                        bytes.reverse();
                        let _ = socket.send_to(&bytes, addr).await;
                    }
                    ServerPackets::GetLocationDesc { id, from } => {
                        let packet = match self.session.location_get_desc(&from) {
                            Ok(ok) => ClientPackets::GetLocationDesc(id, Ok(ok)),
                            Err(err) => ClientPackets::GetLocationDesc(id, Err(err)),
                        };
                        let mut bytes = packet.to_bytes();
                        bytes.reverse();
                        let _ = socket.send_to(&bytes, addr).await;
                    }
                    ServerPackets::SetLocationDesc { id, from, to } => {
                        let packet = match self.session.location_set_desc(&from, &to) {
                            Ok(ok) => ClientPackets::SetLocationDesc(id, Ok(ok)),
                            Err(err) => ClientPackets::SetLocationDesc(id, Err(err)),
                        };
                        let mut bytes = packet.to_bytes();
                        bytes.reverse();
                        let _ = socket.send_to(&bytes, addr).await;
                    }
                    ServerPackets::GetLocationInfo { id, from } => {
                        let packet = match self.session.location_get_location_info(&from) {
                            Ok(ok) => ClientPackets::GetLocationInfo(id, Ok(ok)),
                            Err(err) => ClientPackets::GetLocationInfo(id, Err(err)),
                        };
                        let mut bytes = packet.to_bytes();
                        bytes.reverse();
                        let _ = socket.send_to(&bytes, addr).await;
                    }
                }
            }
        }
    }
}

async fn tokio_main() {
    let daemon = Daemon::default();
    daemon.run().await;
}
