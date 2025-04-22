pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

impl Solution {
    fn build_map(s: &str, length: u32) -> HashMap<u32, u32> {
        let mut map = HashMap::new();
        let probe = 1_u32 << (length - 1);
        let mask = (probe << 1).wrapping_sub(1);
        let mut queue = 0;

        (0..).zip(s.bytes()).for_each(|(i, c)| {
            queue = (queue << 1) | u32::from(c & 1);

            if queue & probe != 0 {
                map.entry(queue & mask).or_insert(i);
            }
        });

        map
    }

    pub fn substring_xor_queries(s: String, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let zero = s.bytes().position(|c| c == b'0');
        let mut positions = [const { None }; 32];

        queries
            .into_iter()
            .map(|query| {
                Vec::from({
                    let [first, second] = query.try_into().ok().unwrap();
                    let target = (first ^ second) as u32;

                    (if target == 0 {
                        zero.map(|index| [index as _, index as i32])
                    } else {
                        let length = 32 - target.leading_zeros();
                        let map = positions[length as usize - 1].get_or_insert_with(|| Self::build_map(&s, length));

                        map.get(&target).map(|&end| [(end - (length - 1)) as _, end as _])
                    })
                    .unwrap_or([-1, -1])
                })
            })
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn substring_xor_queries(s: String, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        Self::substring_xor_queries(s, queries)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
