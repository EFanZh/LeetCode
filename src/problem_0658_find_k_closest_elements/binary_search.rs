pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let mut arr = arr;
        let k = k as usize;
        let base = arr.as_ptr() as usize;

        let start = arr[..arr.len() - k].partition_point(|left| {
            let i = (left as *const _ as usize - base) / 4; // TODO: Use `pointer::offset_from`.
            let right = arr[i + k];

            x - left > right - x
        });

        arr.copy_within(start..start + k, 0);
        arr.truncate(k);

        arr
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        Self::find_closest_elements(arr, k, x)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
