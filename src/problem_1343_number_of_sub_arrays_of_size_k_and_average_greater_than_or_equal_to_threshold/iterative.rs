pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>, k: i32, threshold: i32) -> i32 {
        let threshold = threshold * k;
        let (left, right) = arr.split_at(k as _);
        let mut sum = left.iter().sum::<i32>();
        let mut result = i32::from(sum >= threshold);

        for (old, new) in arr.iter().zip(right) {
            sum += new - old;

            if sum >= threshold {
                result += 1;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn num_of_subarrays(arr: Vec<i32>, k: i32, threshold: i32) -> i32 {
        Self::num_of_subarrays(arr, k, threshold)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
