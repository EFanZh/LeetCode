pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
        if nums.len() < 2 {
            nums
        } else {
            nums.sort_unstable();

            let mut max_size = 1;
            let mut max_node = 0;
            let mut sizes = vec![1; nums.len()];
            let mut parents = vec![usize::max_value(); nums.len()];

            for (i, &num) in nums.iter().enumerate().skip(1) {
                let (size, parent) = nums[..i]
                    .iter()
                    .zip(&sizes)
                    .enumerate()
                    .filter(|(_, (&x, _))| num % x == 0)
                    .max_by_key(|(_, (_, &size))| size)
                    .map_or((1, usize::max_value()), |(j, (_, size))| (size + 1, j));

                sizes[i] = size;
                parents[i] = parent;

                if size > max_size {
                    max_size = size;
                    max_node = i;
                }
            }

            let mut result = Vec::with_capacity(max_size);

            result.push(nums[max_node]);

            let mut i = parents[max_node];

            while let Some(&num) = nums.get(i) {
                result.push(num);

                i = parents[i];
            }

            result
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn largest_divisible_subset(nums: Vec<i32>) -> Vec<i32> {
        Self::largest_divisible_subset(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
