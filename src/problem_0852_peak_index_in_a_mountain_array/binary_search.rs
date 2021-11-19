pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        let mut left = 1;
        let mut right = arr.len() - 2;

        while left < right {
            let middle = (left + right) / 2;

            if arr[middle] < arr[middle + 1] {
                left = middle + 1;
            } else {
                right = middle;
            }
        }

        left as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        Self::peak_index_in_mountain_array(arr)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
