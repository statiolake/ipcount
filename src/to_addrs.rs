use std::net::Ipv4Addr;

use crate::cidr::Cidr;

pub fn cidrs_to_addrs(cidrs: &[Cidr]) -> Vec<Ipv4Addr> {
    cidrs.iter().flat_map(|cidr| cidr.expand()).collect()
}
