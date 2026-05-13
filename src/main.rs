use std::io;
use tun_tap::{Iface, Mode};

fn main() -> io::Result<()> {
    let nic: Iface = Iface::new("tun0", Mode::Tun)?;
    let mut buf = [0u8; 1504];
    loop{
        let _nbytes = nic.recv(&mut buf[..])?;
        eprintln!("{:?}", &buf[..52]);
    }
}