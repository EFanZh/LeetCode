pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn fold_xor(arr: &[i32]) -> i32 {
        arr.iter().fold(0, |result, x| result ^ x)
    }

    pub fn get_xor_sum(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        Self::fold_xor(&arr1) & Self::fold_xor(&arr2)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn get_xor_sum(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        Self::get_xor_sum(arr1, arr2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
