use std::io;
use tun_tap::{Iface, Mode};

fn main() -> io::Result<()> {
    let nic: Iface = Iface::new("tun0", Mode::Tun)?;
    let mut buf = [0u8; 1504];
    loop {
        let nbytes = nic.recv(&mut buf[..])?;
        let _eht_flags = u16::from_be_bytes([buf[0], buf[1]]);
        let eth_proto = u16::from_be_bytes([buf[2], buf[3]]);
        if eth_proto != 0x0800 {
            //not IPv4
            continue;
        }

        match etherparse::Ipv4HeaderSlice::from_slice(&buf[4..nbytes]) {
            Ok(p) => {
                let src = p.source_addr();
                let dst = p.destination_addr();
                let proto = p.protocol();

                if proto != etherparse::IpNumber(0x06) {
                    //not TCP
                    continue;
                };

                match etherparse::TcpHeaderSlice::from_slice(&buf[4 + p.slice().len()..]) {
                    Ok(t) => {
                        let s_port = t.source_port();
                        let d_port= t.destination_port();

                        eprintln!("{:?} -> {:?} {:?}",s_port,d_port,proto);
                    }
                    Err(e) => {
                        eprintln!("ignoring weired tcp packet {:?}", e);
                    }
                }

            }
            Err(e) => {
                eprintln!("Ipv4HeaderSlice from_slice failed {:?}", e);
            }
        }
    }
}
