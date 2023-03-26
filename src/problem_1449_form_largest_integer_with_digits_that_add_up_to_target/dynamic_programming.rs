pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::convert::TryInto;

impl Solution {
    #[allow(clippy::unnecessary_lazy_evaluations)] // Not supported by LeetCode.
    pub fn largest_number(cost: Vec<i32>, target: i32) -> String {
        let cost: &[_; 9] = cost.as_slice().try_into().ok().unwrap();
        let mut target = target as u32 as usize;
        let mut cache = vec![(0_u16, 0_u16); target + 1].into_boxed_slice();

        cache[0].0 = u16::MAX;

        for current_cost in 1..=target {
            cache[current_cost] = cost
                .iter()
                .enumerate()
                .filter_map(|(i, &digit_cost)| {
                    let prev_length = cache.get(current_cost.wrapping_sub(digit_cost as u32 as _))?.1;

                    (prev_length != u16::MAX).then(|| (i, prev_length))
                })
                .max_by_key(|&(_, prev_length)| prev_length)
                .map_or((u16::MAX, u16::MAX), |(i, length)| (i as _, length + 1));
        }

        let mut node = cache[target];

        if node.1 == u16::MAX {
            String::from("0")
        } else {
            let mut result = Vec::with_capacity(usize::from(node.1));

            loop {
                result.push(b'1' + node.0 as u8);

                target -= cost[usize::from(node.0)] as u32 as usize;

                if target == 0 {
                    break;
                }

                node = cache[target];
            }

            String::from_utf8(result).unwrap()
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn largest_number(cost: Vec<i32>, target: i32) -> String {
        Self::largest_number(cost, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
