pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn can_convert_string(s: String, t: String, k: i32) -> bool {
        s.len() == t.len() && {
            let k = k as u32;

            let mut steps: [u32; 26] = [
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
            ];

            for (from, to) in s.bytes().zip(t.bytes()) {
                if from != to {
                    let mut distance = to.wrapping_sub(from);

                    if to < from {
                        distance = distance.wrapping_add(26);
                    }

                    let step = &mut steps[usize::from(distance)];

                    if *step > k {
                        return false;
                    }

                    *step += 26;
                }
            }

            true
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn can_convert_string(s: String, t: String, k: i32) -> bool {
        Self::can_convert_string(s, t, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
