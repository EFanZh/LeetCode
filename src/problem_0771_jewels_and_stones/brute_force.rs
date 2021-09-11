pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn num_jewels_in_stones(j: String, s: String) -> i32 {
        let mut jewels = [false; (b'z' - b'A' + 1) as _];

        for x in j.bytes() {
            jewels[usize::from(x) - usize::from(b'A')] = true;
        }

        let mut result = 0;

        for x in s.bytes() {
            if jewels[usize::from(x) - usize::from(b'A')] {
                result += 1;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn num_jewels_in_stones(j: String, s: String) -> i32 {
        Self::num_jewels_in_stones(j, s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
