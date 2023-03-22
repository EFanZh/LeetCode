pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
        let k = k as u32;
        let mut left = 0;
        let mut right = arr.len();

        while left < right {
            let middle = (left + right) / 2;
            let missing = arr[middle] as u32 - (middle as u32 + 1);

            if missing < k {
                left = middle + 1;
            } else {
                right = middle;
            }
        }

        left as i32 + k as i32
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
        Self::find_kth_positive(arr, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
