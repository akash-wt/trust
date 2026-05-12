use std::io;
use tun_tap::{Iface, Mode};

fn main() -> io::Result<()> {
    let nic = Iface::new("mytun", Mode::Tun)?;
    let mut buf = [0u8; 1504];
    let nbytes = nic.recv(&mut buf[..])?;
    eprintln!("{:?}",buf);
    Ok(()) 
}
