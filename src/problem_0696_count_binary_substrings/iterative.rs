pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn count_binary_substrings(s: String) -> i32 {
        let mut result = 0;
        let mut iter = s.bytes();
        let mut prev_chunk_length = 0;
        let mut prev = iter.next().unwrap();
        let mut length = 1;

        for c in iter {
            if c == prev {
                length += 1;
            } else {
                result += prev_chunk_length.min(length);

                prev_chunk_length = length;
                prev = c;
                length = 1;
            }
        }

        result + prev_chunk_length.min(length)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_binary_substrings(s: String) -> i32 {
        Self::count_binary_substrings(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
