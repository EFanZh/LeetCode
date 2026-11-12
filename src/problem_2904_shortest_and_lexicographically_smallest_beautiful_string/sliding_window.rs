pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::fmt::Write as _;

impl Solution {
    pub fn shortest_beautiful_substring(s: String, k: i32) -> String {
        let k = k.cast_unsigned();
        let mut window = 0_u128;
        let mut min_window = u128::MAX;
        let mut ones = 0;

        for c in s.bytes() {
            window <<= 1;

            if c == b'1' {
                window |= 1;

                if ones == k {
                    window &= !(1 << window.ilog2());
                } else {
                    ones += 1;

                    if ones < k {
                        continue;
                    }
                }

                min_window = min_window.min(window);
            }
        }

        let mut result = s;

        result.clear();

        if min_window != u128::MAX {
            write!(result, "{min_window:b}").ok();
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn shortest_beautiful_substring(s: String, k: i32) -> String {
        Self::shortest_beautiful_substring(s, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
