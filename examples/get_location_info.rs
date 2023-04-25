use muzzman_daemon::prelude::*;

fn main() {
    {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.handle().enter();

        runtime.block_on(async {
            let daemon = DaemonSession::new().await.unwrap();
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
        });
    }
}
