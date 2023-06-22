pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::iter;

impl Solution {
    pub fn get_smallest_string(n: i32, k: i32) -> String {
        let n = n as u32;
        let k = k as u32;
        let a_count = (26 * n - k) / 25;
        let mut result = Vec::with_capacity(n as _);

        result.extend(iter::repeat(b'a').take(a_count as _));

        if a_count < n {
            let z_count = n - a_count - 1;
            let x = k - a_count - 26 * z_count;

            result.push(b'a' - 1 + (x as u8));
            result.extend(iter::repeat(b'z').take(z_count as _));
        }

        String::from_utf8(result).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn get_smallest_string(n: i32, k: i32) -> String {
        Self::get_smallest_string(n, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
