pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::convert::TryInto;

impl Solution {
    pub fn largest_time_from_digits(arr: Vec<i32>) -> String {
        let [a, b, c, d]: [_; 4] = arr.try_into().unwrap();
        let mut arr = [a as u8, b as u8, c as u8, d as u8];

        arr.sort_unstable();

        let [a, b, c, d] = arr;

        for (h1, rest) in [
            (d, [(c, [[b, a], [a, b]]), (b, [[c, a], [a, c]]), (a, [[c, b], [b, c]])]),
            (c, [(d, [[b, a], [a, b]]), (b, [[d, a], [a, d]]), (a, [[d, b], [b, d]])]),
            (b, [(d, [[c, a], [a, c]]), (c, [[d, a], [a, d]]), (a, [[d, c], [c, d]])]),
            (a, [(d, [[c, b], [b, c]]), (c, [[d, b], [b, d]]), (b, [[d, c], [c, d]])]),
        ] {
            match h1 {
                0 | 1 => {
                    for (h2, rest) in rest {
                        for [m1, m2] in rest {
                            if m1 < 6 {
                                return format!("{h1}{h2}:{m1}{m2}");
                            }
                        }
                    }
                }
                2 => {
                    for (h2, rest) in rest {
                        if h2 < 4 {
                            for [m1, m2] in rest {
                                if m1 < 6 {
                                    return format!("{h1}{h2}:{m1}{m2}");
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        String::new()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn largest_time_from_digits(arr: Vec<i32>) -> String {
        Self::largest_time_from_digits(arr)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
