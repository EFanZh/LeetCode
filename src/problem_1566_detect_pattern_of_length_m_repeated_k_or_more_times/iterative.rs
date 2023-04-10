pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn contains_pattern(arr: Vec<i32>, m: i32, k: i32) -> bool {
        let m = m as u32;
        let k = k as u32;
        let required = m * (k - 1);
        let mut length = 0;

        for (old, new) in arr.iter().zip(&arr[m as usize..]) {
            if old == new {
                length += 1;

                if length >= required {
                    return true;
                }
            } else {
                length = 0;
            }
        }

        false
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn contains_pattern(arr: Vec<i32>, m: i32, k: i32) -> bool {
        Self::contains_pattern(arr, m, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
