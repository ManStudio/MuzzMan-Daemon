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

    let new_element = default_location.create_element("TestElement").unwrap();
    println!(
        "Element Info: {:?}",
        new_element.get_element_info().unwrap()
    );

    println!(
        "Default Location Info: {:?}",
        default_location.get_location_info().unwrap()
    );
}
