use anyhow::Context;
use ipcount::{cidr::Cidr, to_addrs};
use std::io::{stdin, Read};

fn main() -> anyhow::Result<()> {
    let stdin = {
        let mut buf = String::new();
        stdin().read_to_string(&mut buf)?;
        buf
    };

    let cidrs = stdin
        .lines()
        .map(|l| l.trim().parse().map_err(Into::into))
        .collect::<anyhow::Result<Vec<Cidr>>>()
        .context("failed to parse cidrs from stdin")?;

    for addr in to_addrs::cidrs_to_addrs(&cidrs) {
        println!("{}", addr);
    }

    Ok(())
}
