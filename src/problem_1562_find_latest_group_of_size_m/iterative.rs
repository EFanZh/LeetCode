pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn find_latest_step(arr: Vec<i32>, m: i32) -> i32 {
        let n = arr.len();
        let m = m as u32;

        if n == m as usize {
            m as _
        } else {
            let mut lengths = vec![0; n].into_boxed_slice();
            let lengths = lengths.as_mut();
            let mut result = -1;

            for (step, i) in (0..).zip(arr) {
                let i = i as u32 as usize - 1;
                let left_length = lengths.get(i.wrapping_sub(1)).copied().unwrap_or(0);
                let right_length = lengths.get(i + 1).copied().unwrap_or(0);

                if left_length == m || right_length == m {
                    result = step;
                }

                let new_length = left_length + right_length + 1;

                lengths[i - left_length as usize] = new_length;
                lengths[i + right_length as usize] = new_length;
            }

            result
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_latest_step(arr: Vec<i32>, m: i32) -> i32 {
        Self::find_latest_step(arr, m)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
