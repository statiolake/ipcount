use std::{fmt, net::Ipv4Addr, str::FromStr};

use anyhow::Context;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Cidr {
    pub start: Ipv4Addr,
    pub mask: u32,
}

impl Cidr {
    pub fn new(start: Ipv4Addr, mask: u32) -> Self {
        assert!(mask <= 32);
        Self { start, mask }
    }

    pub fn expand(self) -> Vec<Ipv4Addr> {
        let start = u32::from_be_bytes(self.start.octets()) as u64; // To avoid overflow
        let end = start + (1 << (32 - self.mask));

        (start..end)
            .map(|addr| {
                let octets = u32::try_from(addr)
                    .expect("bug: should be 32bit")
                    .to_be_bytes();
                Ipv4Addr::from(octets)
            })
            .collect()
    }
}

impl fmt::Display for Cidr {
    fn fmt(&self, b: &mut fmt::Formatter) -> fmt::Result {
        write!(b, "{}/{}", self.start, self.mask)
    }
}

impl FromStr for Cidr {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.splitn(2, '/');
        let start = parts.next().context("missing start")?.parse()?;
        let mask = parts.next().context("missing mask")?.parse()?;
        Ok(Self::new(start, mask))
    }
}
