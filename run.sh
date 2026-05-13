#!/bin/sh
cargo b --release
sudo setcap cap_net_admin=eip ~/trust/target/release/trust
~/trust/target/release/trust &
ip addr

sudo ip link set up dev tun0