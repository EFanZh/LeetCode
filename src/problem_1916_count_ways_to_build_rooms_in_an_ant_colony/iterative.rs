pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::num::NonZeroU64;

struct Context<'a> {
    nodes: &'a [Vec<u32>],
    factorials: &'a [i32],
}

impl Solution {
    const MODULUS: u64 = 1_000_000_007;

    fn factorials(result: &mut Vec<i32>, size: usize) {
        result.push(1);

        let mut prev = 1;

        result.extend((1..size as u64).map(|i| {
            prev = prev * i % Self::MODULUS;

            prev as i32
        }));
    }

    fn exp_mod(mut base: u64, mut exp: u32, modulus: NonZeroU64) -> u64 {
        let mut result = 1;

        while exp != 0 {
            if exp & 1 != 0 {
                result = (result * base) % modulus;
            }

            exp >>= 1;
            base = base * base % modulus;
        }

        result
    }

    fn mod_inverse(x: u64) -> u64 {
        Self::exp_mod(x, (Self::MODULUS - 2) as _, NonZeroU64::new(Self::MODULUS).unwrap())
    }

    fn dfs(context: &Context, node: u32) -> (u32, u64) {
        let mut nodes = 0;
        let mut ways = 1_u64;

        for &child in context.nodes[node as usize].as_slice() {
            let (child_nodes, child_ways) = Self::dfs(context, child);

            nodes += child_nodes;

            ways = (ways * child_ways) % Self::MODULUS;

            ways =
                (ways * Self::mod_inverse(u64::from(context.factorials[child_nodes as usize] as u32))) % Self::MODULUS;
        }

        ways = ways * u64::from(context.factorials[nodes as usize] as u32) % Self::MODULUS;

        (nodes + 1, ways)
    }

    pub fn ways_to_build_rooms(prev_room: Vec<i32>) -> i32 {
        let mut nodes = vec![Vec::new(); prev_room.len()].into_boxed_slice();

        for (room, &parent) in (0_u32..).zip(&prev_room) {
            if let Some(children) = nodes.get_mut(parent as usize) {
                children.push(room);
            }
        }

        let mut factorials = prev_room;

        factorials.clear();

        Self::factorials(&mut factorials, nodes.len());

        Self::dfs(
            &Context {
                nodes: &nodes,
                factorials: &factorials,
            },
            0,
        )
        .1 as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn ways_to_build_rooms(prev_room: Vec<i32>) -> i32 {
        Self::ways_to_build_rooms(prev_room)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
