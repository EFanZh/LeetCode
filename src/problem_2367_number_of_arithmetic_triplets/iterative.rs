pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn arithmetic_triplets(nums: Vec<i32>, diff: i32) -> i32 {
        let double_diff = diff + diff;
        let mut seen = [false; 201];
        let mut result = 0;

        for num in nums {
            seen[num as u32 as usize] = true;

            result += i32::from(
                seen.get((num - double_diff) as usize)
                    .map_or(false, |&first| first && seen[(num - diff) as usize]),
            );
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn arithmetic_triplets(nums: Vec<i32>, diff: i32) -> i32 {
        Self::arithmetic_triplets(nums, diff)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
