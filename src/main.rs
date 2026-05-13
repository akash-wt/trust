use std::io;
use tun_tap::{Iface, Mode};

fn main() -> io::Result<()> {
    let nic: Iface = Iface::new("tun0", Mode::Tun)?;
    let mut buf = [0u8; 1504];
    loop {
        let nbytes = nic.recv(&mut buf[..])?;
        let flags = u16::from_be_bytes([buf[0], buf[1]]);
        let proto = u16::from_be_bytes([buf[2], buf[3]]);
        eprintln!(
            "flags {:x}, proto {:x} {:?} ",
            flags,
            proto,
            &buf[4..nbytes]
        );
    }
}
