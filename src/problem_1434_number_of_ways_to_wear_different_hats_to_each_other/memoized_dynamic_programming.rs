pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

impl Solution {
    fn iter_bits(mut value: u16, mut f: impl FnMut(u16)) {
        while value != 0 {
            f(value & !(value - 1));

            value &= value - 1;
        }
    }

    fn helper(hat_to_people: &[u16; 41], cache: &mut HashMap<(u64, u16), u32>, key: (u64, u16)) -> u32 {
        if key.1 == 0 {
            // All people have been assigned a hat.
            1
        } else if key.0.count_ones() < key.1.count_ones() {
            // Not enough hats.
            0
        } else if let Some(&result) = cache.get(&key) {
            // We have already calculated this configuration.
            result
        } else {
            let remaining_hats = key.0 & (key.0 - 1);

            // The hat we need to assign.
            let current_hat = key.0.trailing_zeros();

            // Not assigning this hat.

            let mut result = Self::helper(hat_to_people, cache, (remaining_hats, key.1));

            // Assign this hat to someone.

            Self::iter_bits(hat_to_people[current_hat as usize] & key.1, |mask| {
                result += Self::helper(hat_to_people, cache, (remaining_hats, key.1 ^ mask));
                result = result.checked_sub(1_000_000_007).unwrap_or(result);
            });

            cache.insert(key, result);

            result
        }
    }

    pub fn number_ways(hats: Vec<Vec<i32>>) -> i32 {
        let mut hat_to_people = [0; 41];
        let mut all_hats = 0;
        let all_people = (1 << hats.len()) - 1;

        for (i, hats) in hats.into_iter().enumerate() {
            for hat in hats {
                hat_to_people[hat as u32 as usize] |= 1 << i;
                all_hats |= 1 << hat;
            }
        }

        Self::helper(&hat_to_people, &mut HashMap::new(), (all_hats, all_people)) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn number_ways(hats: Vec<Vec<i32>>) -> i32 {
        Self::number_ways(hats)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
