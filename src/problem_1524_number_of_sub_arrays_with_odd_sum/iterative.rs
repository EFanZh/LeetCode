pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
        let mut counts = [1_u32, 0_u32];
        let mut state = 0;
        let mut result = 0_u32;

        for num in arr {
            state ^= num as usize & 1;

            counts[state] += 1;
            result += counts[state ^ 1];
            result = result.checked_sub(1_000_000_007).unwrap_or(result);
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn num_of_subarrays(arr: Vec<i32>) -> i32 {
        Self::num_of_subarrays(arr)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
