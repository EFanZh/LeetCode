pub struct Solution {}

impl Solution {
    fn length_of_lis(nums: Vec<i32>) -> i32 {
        // Invariant: The last element of the increasing subsequence of length i with the minimal last element is
        // smaller than the last element of the increasing subsequence of length i + 1 with the minimal last element
        //
        // cache[i] stores the minimal last element of increasing subsequence of length i.

        let mut cache = Vec::with_capacity(nums.len());

        for num in nums {
            if let Err(index) = cache.binary_search(&num) {
                if let Some(value) = cache.get_mut(index) {
                    *value = num;
                } else {
                    cache.push(num);
                }
            }
        }

        cache.len() as _
    }
}

impl super::Solution for Solution {
    fn length_of_lis(nums: Vec<i32>) -> i32 {
        Solution::length_of_lis(nums)
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::run_tests;
    use super::Solution;

    #[test]
    fn test_solution() {
        run_tests::<Solution>();
    }
}
