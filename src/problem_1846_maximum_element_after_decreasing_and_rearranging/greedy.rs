pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn maximum_element_after_decrementing_and_rearranging(arr: Vec<i32>) -> i32 {
        let mut arr = arr;

        arr.sort_unstable();

        let mut prev = 0;

        for value in arr {
            prev = (prev + 1).min(value);
        }

        prev
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_element_after_decrementing_and_rearranging(arr: Vec<i32>) -> i32 {
        Self::maximum_element_after_decrementing_and_rearranging(arr)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
