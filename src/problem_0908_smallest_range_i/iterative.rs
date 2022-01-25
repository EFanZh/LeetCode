pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn min_max(nums: &[i32]) -> (i32, i32) {
        if let [first, second, rest @ ..] = nums {
            let (mut min, mut max) = if second < first {
                (*second, *first)
            } else {
                (*first, *second)
            };

            let mut iter = rest.iter().copied();

            while let Some(left) = iter.next() {
                if let Some(right) = iter.next() {
                    let (new_min, new_max) = if right < left { (right, left) } else { (left, right) };

                    min = min.min(new_min);
                    max = max.max(new_max);
                } else {
                    if left < min {
                        min = left;
                    } else if left > max {
                        max = left;
                    }

                    break;
                }
            }

            (min, max)
        } else {
            let first = nums[0];

            (first, first)
        }
    }

    pub fn smallest_range_i(nums: Vec<i32>, k: i32) -> i32 {
        let (min, max) = Self::min_max(&nums);

        (max - min - k * 2).max(0)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn smallest_range_i(nums: Vec<i32>, k: i32) -> i32 {
        Self::smallest_range_i(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
