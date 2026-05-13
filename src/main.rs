use std::io;
use tun_tap::{Iface, Mode};

fn main() -> io::Result<()> {
    let nic: Iface = Iface::new("tun0", Mode::Tun)?;
    let mut buf = [0u8; 1504];
    loop {
        let nbytes = nic.recv(&mut buf[..])?;
        let eht_flags = u16::from_be_bytes([buf[0], buf[1]]);
        let eth_proto = u16::from_be_bytes([buf[2], buf[3]]);
        if eth_proto != 0x0800{
            //not IPv4
            continue;
        }

        match etherparse::IPv4HeaderSlice::from_slice(&[4..nbytes]){
            Ok(p)=>{
                eprintln("{}",p);
            }
            Err(e)=>{

            }
        }
        eprintln!(
            "flags {:x}, proto {:x} {:?} ",
            eht_flags,
            eth_proto,
            &buf[4..nbytes]
        );

    }
}
