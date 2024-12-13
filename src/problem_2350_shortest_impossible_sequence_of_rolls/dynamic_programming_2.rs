pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn shortest_sequence(rolls: Vec<i32>, k: i32) -> i32 {
        let k = k as u32;
        let mut states = vec![false; k as usize].into_boxed_slice();
        let mut true_count = 0;
        let mut result = 1;
        let mut iter = rolls.iter().copied();

        'outer: loop {
            loop {
                if let Some(roll) = iter.next() {
                    if let state @ false = &mut states[roll as u32 as usize - 1] {
                        *state = true;

                        true_count += 1;

                        if true_count == k {
                            result += 1;

                            break;
                        }
                    }
                } else {
                    break 'outer;
                }
            }

            loop {
                if let Some(roll) = iter.next() {
                    if let state @ true = &mut states[roll as u32 as usize - 1] {
                        *state = false;

                        true_count -= 1;

                        if true_count == 0 {
                            result += 1;

                            break;
                        }
                    }
                } else {
                    break 'outer;
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn shortest_sequence(rolls: Vec<i32>, k: i32) -> i32 {
        Self::shortest_sequence(rolls, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
