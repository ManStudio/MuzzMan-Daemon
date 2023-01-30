use muzzman_daemon::daemon::Daemon;

fn main() {
    let daemon = Daemon::new().unwrap();
    daemon.run();
}
