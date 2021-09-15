pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut max = 0;

        for (i, num) in (0..).zip(arr) {
            max = max.max(num);

            if max == i {
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
