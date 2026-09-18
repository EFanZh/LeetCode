pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn count_rotations(s: String, k: i32) -> i32 {
        let mut iter = s.bytes();
        let first = iter.next().unwrap();
        let mut prev = first;
        let mut same = 0;

        for c in iter {
            same += i32::from(c == prev);
            prev = c;
        }

        same += i32::from(first == prev);

        let boundaries = s.len() as i32 - same;

        if k == same - 1 {
            same
        } else if k == same {
            boundaries
        } else {
            0
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_rotations(s: String, k: i32) -> i32 {
        Self::count_rotations(s, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
