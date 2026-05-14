use std::collections::HashMap;
use std::io;
use std::net::Ipv4Addr;
use tun_tap::{Iface, Mode};

struct TcpState{

}

#![derive(Copy,Clone,Debug,Eq,PartialEq)]
struct Quad{
src: (Ipv4Addr,u16),
dst: (Ipv4Addr,u16)
}

fn main() -> io::Result<()> {
    let connections :HashMap<Quad,TcpState> = Default::default();

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
                        // (srcip, srcp, dstip, dstp)
                        let s_port = t.source_port();
                        let d_port = t.destination_port();
                        eprintln!(
                            "{:?} -> {:?} {:?}b of tcp {:?}",
                            s_port,
                            d_port,
                            t.slice().len(),
                            proto
                        );
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
