pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn best_rotation(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut end_points = vec![0; n + 1];

        for (i, &num) in nums.iter().enumerate() {
            let num = num as usize;

            if num <= i {
                end_points[0] += 1;
                end_points[i - num + 1] -= 1;
                end_points[i + 1] += 1;
            } else {
                end_points[i + 1] += 1;
                end_points[n - (num - i) + 1] -= 1;
            }
        }

        let mut score = 0;
        let mut max_score = 0;
        let mut max_score_index = 0;

        for (i, diff) in (0..).zip(end_points) {
            score += diff;

            if score > max_score {
                max_score = score;
                max_score_index = i;
            }
        }

        max_score_index
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn best_rotation(nums: Vec<i32>) -> i32 {
        Self::best_rotation(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
