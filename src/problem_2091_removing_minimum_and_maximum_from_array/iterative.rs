pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn min_max(nums: &[i32]) -> (usize, usize) {
        let mut iter = nums.iter().copied().enumerate();
        let mut min = iter.next().unwrap();
        let mut max = min;

        while let Some(left) = iter.next() {
            if let Some(right) = iter.next() {
                let (new_min, new_max) = if right.1 < left.1 { (right, left) } else { (left, right) };

                if new_min.1 < min.1 {
                    min = new_min;
                }

                if new_max.1 > max.1 {
                    max = new_max;
                }
            } else {
                if left.1 < min.1 {
                    min = left;
                } else if left.1 > max.1 {
                    max = left;
                }

                break;
            }
        }

        (min.0, max.0)
    }

    pub fn minimum_deletions(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let (i, j) = Self::min_max(&nums);
        let (i, j) = if j < i { (j, i) } else { (i, j) };
        let candidate_1 = j + 1;
        let candidate_2 = n - i;
        let candidate_3 = i + 1 + (n - j);

        candidate_1.min(candidate_2).min(candidate_3) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_deletions(nums: Vec<i32>) -> i32 {
        Self::minimum_deletions(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
