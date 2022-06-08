pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn longest_wpi(hours: Vec<i32>) -> i32 {
        let mut sums = hours;
        let mut sum = 0;

        for target in &mut sums {
            sum += if *target <= 8 { -1 } else { 1 };
            *target = sum;
        }

        let mut stack = Vec::new();
        let mut max_sum = i32::MIN;
        let mut result = 0;
        let mut i = sums.len() as i32;

        for &sum in sums.iter().rev() {
            i -= 1;

            if sum > max_sum {
                if sum > 0 {
                    result = i + 1;

                    break;
                }

                max_sum = sum;
                stack.push((i, sum));
            }
        }

        let mut min_sum = i32::MAX;

        'outer: for (i, &sum) in (0..).zip(&sums) {
            if sum < min_sum {
                min_sum = sum;

                loop {
                    if let Some(&(j, top)) = stack.last() {
                        if top > sum {
                            result = result.max(j - i);

                            stack.pop();
                        } else {
                            break;
                        }
                    } else {
                        break 'outer;
                    }
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn longest_wpi(hours: Vec<i32>) -> i32 {
        Self::longest_wpi(hours)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
