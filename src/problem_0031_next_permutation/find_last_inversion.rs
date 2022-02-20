pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn next_permutation(nums: &mut [i32]) {
        if let Some((i, (current, _))) = nums
            .iter()
            .zip(&nums[1..])
            .enumerate()
            .rfind(|(_, (current, next))| current < next)
        {
            let j = nums[i + 2..].partition_point(|x| current < x);

            nums.swap(i, i + 1 + j);
            nums[i + 1..].reverse();
        } else {
            nums.reverse();
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn next_permutation(nums: &mut Vec<i32>) {
        Self::next_permutation(nums);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
