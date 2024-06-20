use anyhow::Context;
use ipcount::to_cidrs;
use std::{
    io::{stdin, Read},
    net::Ipv4Addr,
};

fn main() -> anyhow::Result<()> {
    let stdin = {
        let mut buf = String::new();
        stdin().read_to_string(&mut buf)?;
        buf
    };

    let addrs = stdin
        .lines()
        .map(|l| l.trim().parse().map_err(Into::into))
        .collect::<anyhow::Result<Vec<Ipv4Addr>>>()
        .context("failed to parse addresses from stdin")?;

    for cidr in to_cidrs::addrs_to_cidrs(&addrs) {
        println!("{}", cidr);
    }

    Ok(())
}
