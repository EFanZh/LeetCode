pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn find_k_distant_indices(nums: Vec<i32>, key: i32, k: i32) -> Vec<i32> {
        let some_key = Some(key);
        let k = k as u32 as usize;
        let mut count = nums[..k].iter().fold(0, |count, &x| count + u16::from(x == key));
        let mut result = Vec::with_capacity(nums.len());

        for i in 0..nums.len() {
            count -= u16::from(nums.get(i.wrapping_sub(k + 1)).copied() == some_key);
            count += u16::from(nums.get(i + k).copied() == some_key);

            if count != 0 {
                result.push(i as _);
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_k_distant_indices(nums: Vec<i32>, key: i32, k: i32) -> Vec<i32> {
        Self::find_k_distant_indices(nums, key, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
