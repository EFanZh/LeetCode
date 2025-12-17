pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn get_smallest_string(s: String) -> String {
        let mut s = s.into_bytes();
        let mut iter = s.iter_mut();
        let mut zero = 0;
        let mut prev = iter.next().unwrap_or(&mut zero);

        for value in iter {
            if *value < *prev && (*prev ^ *value) & 1 == 0 {
                mem::swap(prev, value);

                break;
            }

            prev = value;
        }

        String::from_utf8(s).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn get_smallest_string(s: String) -> String {
        Self::get_smallest_string(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
