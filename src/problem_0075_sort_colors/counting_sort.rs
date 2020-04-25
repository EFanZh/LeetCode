pub struct Solution {}

impl Solution {
    fn sort_colors(nums: &mut Vec<i32>) {
        let mut counts = [0; 3];

        for num in &mut *nums {
            counts[*num as usize] += 1;
        }

        let mut iter = nums.iter_mut();

        for (color, count) in counts.iter().copied().enumerate() {
            for _ in 0..count {
                *iter.next().unwrap() = color as i32;
            }
        }
    }
}

impl super::Solution for Solution {
    fn sort_colors(nums: &mut Vec<i32>) {
        Self::sort_colors(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
