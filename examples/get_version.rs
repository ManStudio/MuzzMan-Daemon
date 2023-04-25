use muzzman_daemon::prelude::*;

fn main() {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.handle().enter();

    runtime.block_on(async {
        let session = DaemonSession::new()
            .await
            .expect("Cannot connect to daemon")
            .create_session();

        let version = session.get_version().expect("version");
        let version_text = session.get_version_text().expect("version_text");

        println!("Version: {version}");
        println!("Version Text: {version_text}");
    })
}
