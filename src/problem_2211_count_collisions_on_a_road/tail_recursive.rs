pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::str::Bytes;

impl Solution {
    fn all_left(mut iter: Bytes, result: u32) -> u32 {
        for c in iter.by_ref() {
            match c {
                b'L' => {}
                b'R' => return Self::right(iter, result, 1),
                _ => return Self::stop(iter, result),
            }
        }

        result
    }

    #[inline(never)]
    fn right(mut iter: Bytes, result: u32, mut count: u32) -> u32 {
        for c in iter.by_ref() {
            if c == b'R' {
                count += 1;
            } else {
                return Self::stop(iter, result + count + u32::from(c == b'L'));
            }
        }

        result
    }

    #[inline(never)]
    fn stop(mut iter: Bytes, mut result: u32) -> u32 {
        for c in iter.by_ref() {
            if c == b'R' {
                return Self::right(iter, result, 1);
            }

            result += u32::from(c == b'L');
        }

        result
    }

    pub fn count_collisions(directions: String) -> i32 {
        Self::all_left(directions.bytes(), 0) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_collisions(directions: String) -> i32 {
        Self::count_collisions(directions)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
