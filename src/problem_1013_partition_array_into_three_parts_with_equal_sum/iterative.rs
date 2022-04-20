pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn can_three_parts_equal_sum(arr: Vec<i32>) -> bool {
        let sum = arr.iter().sum::<i32>();

        if sum % 3 == 0 {
            let target = sum / 3;
            let mut iter = arr.into_iter();
            let mut current_sum = 0;

            loop {
                if let Some(value) = iter.next() {
                    current_sum += value;

                    if current_sum == target {
                        break;
                    }
                } else {
                    return false;
                }
            }

            current_sum = 0;

            loop {
                if let Some(value) = iter.next() {
                    current_sum += value;

                    if current_sum == target {
                        break;
                    }
                } else {
                    return false;
                }
            }

            iter.next().is_some()
        } else {
            false
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn can_three_parts_equal_sum(arr: Vec<i32>) -> bool {
        Self::can_three_parts_equal_sum(arr)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
