pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn score_difference(nums: Vec<i32>) -> i32 {
        let mut diff = 0;
        let mut second_player_active = false;
        let (chunks, rest) = nums.as_chunks::<6>();
        for chunk in chunks {
            let (&last, left) = chunk.split_last().unwrap();

            for &value in left {
                if value & 1 != 0 {
                    second_player_active = !second_player_active;
                }

                if second_player_active {
                    diff -= value;
                } else {
                    diff += value;
                }
            }

            if last & 1 == 0 {
                second_player_active = !second_player_active;
            }

            if second_player_active {
                diff -= last;
            } else {
                diff += last;
            }
        }

        for value in rest {
            if value & 1 != 0 {
                second_player_active = !second_player_active;
            }

            if second_player_active {
                diff -= value;
            } else {
                diff += value;
            }
        }

        diff
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn score_difference(nums: Vec<i32>) -> i32 {
        Self::score_difference(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
