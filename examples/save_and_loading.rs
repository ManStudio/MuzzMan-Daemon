use muzzman_daemon::prelude::*;
use serde_json::from_str as from_jstr;
use serde_json::to_string_pretty as to_jstring;

fn main() {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.handle().enter();

    runtime.block_on(async {
        let session = DaemonSession::new()
            .await
            .expect("Daemon is not online")
            .create_session();

        let location = session.get_default_location().unwrap();
        let new_location = location.create_location("Other Location").unwrap();

        let location_info = location.get_location_info().unwrap();
        let location_info_json = to_jstring(&location_info).unwrap();

        // this will return the RawLocation but the RawLocation cannot be transfered so will return an error the error is expected behaviour!
        if let Err(err) = new_location.destroy() {
            match err {
                SessionError::Custom(err) => {
                    // this is a normal error
                    println!("{err}")
                }
                _ => {
                    panic!("{err:?}")
                }
            }
        }

        println!("Json: {location_info_json}");

        let mut location_info: LocationInfo = from_jstr(&location_info_json).expect("Cannot parse");
        location_info.name = "The New Name".to_string();
        let location = session.load_location_info(location_info).unwrap();

        let location_info = location.get_location_info().unwrap();
        let location_info_json = to_jstring(&location_info).unwrap();

        println!("Now loaded json: {location_info_json}");
    })
}
