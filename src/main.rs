use muzzman_daemon::Daemon;

fn main() {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(tokio_main());
}

async fn tokio_main() {
    let daemon = Daemon::default();

    std::future::pending::<()>().await
}
