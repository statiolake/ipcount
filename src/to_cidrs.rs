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
    // Consider addresses as a u64s (instead of u32s to avoid overflow)
    let addrs = addrs
        .iter()
        .map(|addr| u32::from_be_bytes(addr.octets()) as u64);

    // Group each sequential addresses into an `exclusive` range
    let addr_ranges_exclusive = addrs
        // Add next addresses to make the sequential values exclusive
        .flat_map(|addr| [addr, addr + 1])
        .sorted()
        .dedup()
        .tuple_windows()
        .coalesce(|(x, y), (_, z)| {
            if y + 1 == z {
                // Combine two consecutive addresses into one
                Ok((x, z))
            } else {
                Err(((x, y), (y, z)))
            }
        });

    addr_ranges_exclusive
        .flat_map(|(start, end)| {
            let start = Ipv4Addr::from(
                u32::try_from(start)
                    .expect("bug: start should be 32bit")
                    .to_be_bytes(),
            );
            let end = Ipv4Addr::from(
                u32::try_from(end - 1) // Convert to inclusive
                    .expect("bug: end - 1 should be 32bit")
                    .to_be_bytes(),
            );

            range_to_cidrs(start..=end)
        })
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
