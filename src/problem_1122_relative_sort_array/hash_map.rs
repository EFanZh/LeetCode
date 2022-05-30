pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

impl Solution {
    pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mut arr1 = arr1;
        let ranks = arr2.into_iter().zip(1..).collect::<HashMap<_, u16>>();

        arr1.sort_unstable_by_key(|value| {
            if let Some(&rank) = ranks.get(value) {
                Ok(rank)
            } else {
                Err(*value)
            }
        });

        arr1
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        Self::relative_sort_array(arr1, arr2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
