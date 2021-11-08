pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

// Algorithm:
// <https://leetcode.com/problems/race-car/discuss/124326/Summary-of-the-BFS-and-DP-solutions-with-intuitive-explanation>.

impl Solution {
    pub fn racecar(target: i32) -> i32 {
        let target = target as usize;
        let mut cache = vec![0; target + 1];

        for i in 1..=target {
            if (i + 1).is_power_of_two() {
                cache[i] = (i + 1).trailing_zeros();
            } else {
                let mut min_steps = u32::MAX;
                let mut forward_steps = 1;
                let mut position_1 = 1;

                loop {
                    for backward_steps in 0..forward_steps {
                        let position_2 = position_1 - ((1 << backward_steps) - 1);

                        min_steps = min_steps.min(forward_steps + backward_steps + cache[i - position_2] + 2);
                    }

                    forward_steps += 1;
                    position_1 = (1 << forward_steps) - 1;

                    if position_1 > i {
                        break;
                    }
                }

                min_steps = min_steps.min(forward_steps + cache[position_1 - i] + 1);

                cache[i] = min_steps;
            }
        }

        *cache.last().unwrap() as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn racecar(target: i32) -> i32 {
        Self::racecar(target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
