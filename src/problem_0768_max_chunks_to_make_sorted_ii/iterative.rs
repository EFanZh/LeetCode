pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        let mut max_values = Vec::with_capacity(arr.len());
        let mut max_value = i32::MIN;

        max_values.push(max_value);

        for &num in &arr[..arr.len() - 1] {
            max_value = max_value.max(num);
            max_values.push(max_value);
        }

        let mut result = 0;
        let mut min_value = i32::MAX;

        for (&num, &left_max) in arr.iter().zip(&max_values).rev() {
            min_value = min_value.min(num);

            if min_value >= left_max {
                result += 1;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        Self::max_chunks_to_sorted(arr)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
