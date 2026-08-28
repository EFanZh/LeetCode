pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;
use std::collections::hash_map::Entry;

impl Solution {
    fn solve_1(s: &str) -> u32 {
        let mut prev = 0;
        let mut length = 0_u32;
        let mut result = 0_u32;

        for c in s.bytes() {
            if c == prev {
                length += 1;
            } else {
                result = result.max(length);
                length = 1;
                prev = c;
            }
        }

        result.max(length)
    }

    fn solve_2(s: &str) -> u32 {
        fn helper(s: &str, buffer: &mut [u32], keep: u8, drop: u8) -> u32 {
            let n = s.len();
            let mut i = 0;
            let mut diff = 0;
            let mut min_diff = 0;
            let mut max_diff = 0;
            let mut result = 0;

            for c in s.bytes() {
                if c == drop {
                    i = 0;
                    diff = 0;
                    min_diff = 0;
                    max_diff = 0;
                } else {
                    i += 1;

                    if c == keep {
                        diff -= 1;
                    } else {
                        diff += 1;
                    }

                    let state = &mut buffer[n.wrapping_add(diff as usize)];

                    'block: {
                        if diff < min_diff {
                            min_diff = diff;
                        } else if diff > max_diff {
                            max_diff = diff;
                        } else {
                            result = result.max(i - *state);

                            break 'block;
                        }

                        *state = i;
                    }
                }
            }

            result
        }

        let mut buffer = vec![0; s.len() * 2 + 1].into_boxed_slice();

        helper(s, &mut buffer, b'a', b'c')
            .max(helper(s, &mut buffer, b'a', b'b'))
            .max(helper(s, &mut buffer, b'b', b'a'))
    }

    fn solve_3(s: &str) -> u32 {
        let mut first_occurrences = HashMap::from([((0, 0), 0)]);
        let mut a_count = 0;
        let mut b_count = 0;
        let mut c_count = 0;
        let mut result = 0;

        (1..).zip(s.bytes()).for_each(|(i, c)| {
            match c {
                b'a' => a_count += 1,
                b'b' => b_count += 1,
                _ => c_count += 1,
            }

            match first_occurrences.entry((b_count - a_count, c_count - a_count)) {
                Entry::Occupied(occupied_entry) => result = result.max(i - *occupied_entry.get()),
                Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(i);
                }
            }
        });

        result
    }

    pub fn longest_balanced(s: String) -> i32 {
        let s = s.as_str();

        Self::solve_1(s)
            .max(Self::solve_2(s))
            .max(Self::solve_3(s))
            .cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn longest_balanced(s: String) -> i32 {
        Self::longest_balanced(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
