use muzzman_daemon::DaemonSession;
use muzzman_lib::prelude::*;

fn main() {
    let daemon = DaemonSession::new().unwrap();
    let session = daemon.create_session();

    let default_location = session.get_default_location().unwrap();

    println!(
        "Default Location Info: {:?}",
        default_location.get_location_info().unwrap()
    );
}
