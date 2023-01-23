use std::{net::SocketAddr, str::FromStr};

use bytes_kman::TBytes;
use muzzman_daemon::packets::ServerPackets;
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
    pub async fn run() {
        let socket_addr = SocketAddr::from_str("localhost:2118").unwrap();
        let socket = tokio::net::UdpSocket::bind(socket_addr).await.unwrap();

        let mut buffer = vec![0; 4096];

        loop {
            let len = socket.recv(&mut buffer).await.unwrap();
            let mut buffer = buffer[0..len].to_vec();
            while !buffer.is_empty() {
                let Some(packet) = ServerPackets::from_bytes(&mut buffer)else{continue};
                match packet {
                    ServerPackets::GetDefaultLocation => {}
                    ServerPackets::GetLocationName { from } => todo!(),
                    ServerPackets::SetLocationName { from, to } => todo!(),
                    ServerPackets::GetLocationDesc { from } => todo!(),
                    ServerPackets::SetLocationDesc { from, to } => todo!(),
                    ServerPackets::GetLocationInfo { from } => todo!(),
                }
            }
        }
    }
}

async fn tokio_main() {
    let daemon = Daemon::default();

    std::future::pending::<()>().await
}
