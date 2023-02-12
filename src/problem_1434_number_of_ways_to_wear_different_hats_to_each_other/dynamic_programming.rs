pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn combinations(n: u8, k: u8, base: u16, f: &mut impl FnMut(u16)) {
        if k == 0 {
            f(base);
        } else {
            for i in (k - 1)..n {
                Self::combinations(i, k - 1, base | (1 << i), f);
            }
        }
    }

    pub fn number_ways(hats: Vec<Vec<i32>>) -> i32 {
        let n = hats.len() as u8;
        let mut hat_to_people = [0; 41];
        let mut all_hats = 0_u64;
        let all_people = (1 << n) - 1;

        // Get a hat-to-people map.

        for (i, hats) in hats.into_iter().enumerate() {
            for hat in hats {
                hat_to_people[hat as u32 as usize] |= 1 << i;
                all_hats |= 1 << hat;
            }
        }

        // Get a list of states that is sorted by bit count.

        let mut states_by_bit_count = [0_u16; 1024];
        let mut i = 0;

        for k in 0..=n {
            Self::combinations(n, k, 0, &mut |state| {
                states_by_bit_count[i] = state;
                i += 1;
            });
        }

        let states_by_bit_count = &states_by_bit_count[..1 << n];

        // Dynamic programming.

        // Stores number of ways to wear hats for a people configuration represented by bits.
        let mut cache = [0_u64; 1024];

        cache[all_people] = 1;

        while all_hats != 0 {
            let current_hat = all_hats.trailing_zeros();

            // List of people we can assign this hat to.
            let candidates = hat_to_people[current_hat as usize];

            for &state in states_by_bit_count {
                let current_count = cache[usize::from(state)];
                let mut candidate = state & candidates;

                while candidate != 0 {
                    let mask = candidate & !(candidate - 1);
                    let target = &mut cache[usize::from(state ^ mask)];

                    *target += current_count;
                    *target = target.checked_sub(1_000_000_007).unwrap_or(*target);

                    candidate &= candidate - 1;
                }
            }

            all_hats &= all_hats - 1;
        }

        cache[0] as _
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
