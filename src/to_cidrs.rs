use std::{net::Ipv4Addr, ops::RangeInclusive};

use itertools::Itertools;

use crate::cidr::Cidr;

pub fn range_to_cidrs(range: RangeInclusive<Ipv4Addr>) -> Vec<Cidr> {
    // To support 255.255.255.255, we need to use u64 to avoid overflow
    let start = u32::from_be_bytes(range.start().octets()) as u64;
    let end = u32::from_be_bytes(range.end().octets()) as u64 + 1;

    let mut curr = start;
    let mut steps = vec![];
    while curr < end {
        let shift = (0..=curr.trailing_zeros())
            .rfind(|s| curr + (1 << s) <= end)
            .expect("bug: should exist");
        steps.push((curr, shift));
        curr += 1 << shift;
    }

    steps
        .into_iter()
        .map(|(start, shift)| Cidr {
            start: Ipv4Addr::from(
                u32::try_from(start)
                    .expect("bug: start should be 32bit")
                    .to_be_bytes(),
            ),
            mask: 32 - shift,
        })
        .collect_vec()
}

pub fn addrs_to_cidrs(addrs: &[Ipv4Addr]) -> Vec<Cidr> {
    // Consider addresses as a u32s
    let addrs = addrs
        .iter()
        .map(|addr| u32::from_be_bytes(addr.octets()))
        .sorted()
        .dedup();

    let mut seq_addrs_chunks = vec![];
    let mut seq_addrs = vec![];
    for addr in addrs {
        if seq_addrs.last().is_none() || *seq_addrs.last().unwrap() + 1 == addr {
            seq_addrs.push(addr);
        } else {
            seq_addrs_chunks.push(seq_addrs);
            seq_addrs = vec![addr];
        }
    }
    seq_addrs_chunks.push(seq_addrs);

    seq_addrs_chunks
        .into_iter()
        .map(|seq_addrs| {
            let start = seq_addrs.first().expect("bug: should not be empty");
            let end = seq_addrs.last().expect("bug: should not be empty");
            let start = Ipv4Addr::from(start.to_be_bytes());
            let end = Ipv4Addr::from(end.to_be_bytes());

            (start, end)
        })
        .flat_map(|(start, end)| range_to_cidrs(start..=end))
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_grouped() {
        let groups =
            range_to_cidrs(Ipv4Addr::new(192, 168, 24, 90)..=Ipv4Addr::new(192, 168, 24, 99));
        assert_eq!(
            groups,
            vec![
                Cidr {
                    start: Ipv4Addr::new(192, 168, 24, 90),
                    mask: 31
                },
                Cidr {
                    start: Ipv4Addr::new(192, 168, 24, 92),
                    mask: 30
                },
                Cidr {
                    start: Ipv4Addr::new(192, 168, 24, 96),
                    mask: 30
                },
            ]
        );
    }
}
