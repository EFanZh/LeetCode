pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn query(nodes: &[(u32, u32)], mut probe: u32, x: u32) -> u32 {
        let mut result = x & !((probe << 1) - 1);
        let mut node = nodes[0];

        while probe != 0 {
            let (primary, secondary) = if x & probe == 0 { (node.1, node.0) } else { node };

            node = if let Some(&next_node) = nodes.get(primary as usize) {
                result |= probe;

                next_node
            } else {
                nodes[secondary as usize]
            };

            probe >>= 1;
        }

        // Process last bit.

        result |= u32::from(if x & 1 == 0 { node.1 } else { node.0 } == 0);

        result
    }

    fn trie_insert(nodes: &mut Vec<(u32, u32)>, mut probe: u32, value: u32) {
        let mut total_nodes = nodes.len() as u32;
        let mut node = &mut nodes[0];

        while probe > 1 {
            let next = if value & probe == 0 { &mut node.0 } else { &mut node.1 };

            node = if *next < total_nodes {
                let next = *next as usize;

                &mut nodes[next]
            } else {
                *next = total_nodes;

                total_nodes += 1;

                nodes.push((u32::MAX, u32::MAX));

                nodes.last_mut().unwrap()
            };

            probe >>= 1;
        }

        // We use 0 to represent presence of value.

        if value & 1 == 0 {
            node.0 = 0;
        } else {
            node.1 = 0;
        }
    }

    pub fn maximize_xor(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut nums = nums;

        nums.sort_unstable();
        nums.dedup();

        let mut result = vec![0; queries.len()];

        let mut queue = result
            .iter_mut()
            .zip(queries)
            .map(|(target, query)| {
                let [x, limit]: [_; 2] = query.try_into().ok().unwrap();

                (limit as u32, x as u32, target)
            })
            .collect::<Vec<_>>();

        queue.sort_unstable_by_key(|&(limit, ..)| limit);

        let min_num = nums[0] as u32;
        let probe = 1 << u32::saturating_sub(31, nums.last().unwrap().leading_zeros());
        let mut nodes = vec![(u32::MAX, u32::MAX)];
        let mut iter = nums.iter();

        for (limit, x, target) in queue {
            while let Some(&peek) = iter.as_slice().first() {
                let peek = peek as u32;

                if peek <= limit {
                    Self::trie_insert(&mut nodes, probe, peek);

                    iter.next();
                } else {
                    break;
                }
            }

            *target = if limit < min_num {
                -1
            } else {
                Self::query(&nodes, probe, x) as _
            };
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximize_xor(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        Self::maximize_xor(nums, queries)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
