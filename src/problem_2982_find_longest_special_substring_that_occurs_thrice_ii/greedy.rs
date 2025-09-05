pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn push(lengths: &mut (u32, u32, u32), length: u32) {
        if length > lengths.1 {
            (lengths.0, lengths.1) = if length > lengths.0 {
                (length, lengths.0)
            } else {
                (lengths.0, length)
            };
        } else if length > lengths.2 {
            lengths.2 = length;
        }
    }

    pub fn maximum_length(s: String) -> i32 {
        let mut max_lengths = [(0_u32, 0_u32, 0_u32); 26];
        let mut prev = b'a';
        let mut length = 0;

        for c in s.bytes() {
            if c == prev {
                length += 1;
            } else {
                Self::push(&mut max_lengths[usize::from(prev) - usize::from(b'a')], length);

                length = 1;
            }

            prev = c;
        }

        Self::push(&mut max_lengths[usize::from(prev) - usize::from(b'a')], length);

        let result = max_lengths.iter().fold(0, |result, &(length_1, length_2, length_3)| {
            result.max(if length_1 == length_2 && length_1 == length_3 {
                length_1
            } else if length_2 + 1 >= length_1 {
                length_1 - 1
            } else {
                length_1 - 2
            })
        });

        if result == 0 { -1 } else { result as _ }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_length(s: String) -> i32 {
        Self::maximum_length(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
