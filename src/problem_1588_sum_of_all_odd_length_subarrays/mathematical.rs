pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut j = arr.len() as u32 + 1;

        for (i, num) in (1_u32..).zip(arr) {
            result += (num as u32) * ((i / 2) * ((j - 1) / 2) + ((i + 1) / 2) * (j / 2));

            j -= 1;
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
        Self::sum_odd_length_subarrays(arr)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
