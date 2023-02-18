use std::net::UdpSocket;

#[cfg(target_os = "windows")]
pub type RawSock = std::os::windows::io::RawSocket;
#[cfg(any(target_os = "linux", target_os = "android"))]
pub type RawSock = std::os::unix::io::RawFd;

pub trait FromRawSock {
    fn from_raw(raw_sock: RawSock) -> Self;
}

pub trait IntoRawSock {
    fn into_raw(self) -> RawSock;
}

impl FromRawSock for UdpSocket {
    fn from_raw(raw_sock: RawSock) -> Self {
        #[cfg(target_os = "windows")]
        use std::os::windows::io::FromRawSocket;

        #[cfg(target_os = "windows")]
        unsafe {
            Self::from_raw_socket(raw_sock)
        }

        #[cfg(any(target_os = "linux", target_os = "android"))]
        use std::os::unix::io::FromRawFd;

        #[cfg(any(target_os = "linux", target_os = "android"))]
        unsafe {
            Self::from_raw_fd(raw_sock)
        }
    }
}

impl IntoRawSock for UdpSocket {
    fn into_raw(self) -> RawSock {
        #[cfg(target_os = "windows")]
        use std::os::windows::io::IntoRawSocket;

        #[cfg(target_os = "windows")]
        unsafe {
            self.into_raw_socket()
        }

        #[cfg(any(target_os = "linux", target_os = "android"))]
        use std::os::unix::io::IntoRawFd;
        #[cfg(any(target_os = "linux", target_os = "android"))]
        self.into_raw_fd()
    }
}
