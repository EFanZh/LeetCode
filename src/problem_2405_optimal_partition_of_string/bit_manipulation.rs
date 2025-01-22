pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn partition_string(s: String) -> i32 {
        let mut iter = s.bytes();
        let mut c = iter.next().unwrap();
        let mut used = 0_u32;
        let mut result = 1;

        loop {
            let probe = 1 << (c - b'a');

            if used & probe == 0 {
                used |= probe;
            } else {
                result += 1;
                used = probe;
            }

            if let Some(next) = iter.next() {
                c = next;
            } else {
                break;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn partition_string(s: String) -> i32 {
        Self::partition_string(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
