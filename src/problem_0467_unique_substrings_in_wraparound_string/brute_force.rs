pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn update_length(lengths: &mut [i32; 26], key: u8, length: i32) {
        let target = &mut lengths[usize::from(key - b'a')];

        *target = (*target).max(length);
    }

    pub fn find_substring_in_wrapround_string(p: String) -> i32 {
        p.as_bytes().split_first().map_or(0, |(&first, rest)| {
            let mut max_lengths = [0; 26];
            let mut prev = first;
            let mut length = 1;

            for &c in rest {
                Self::update_length(&mut max_lengths, prev, length);

                let diff = c.wrapping_sub(prev);

                if diff == 1 || diff == b'a'.wrapping_sub(b'z') {
                    length += 1;
                } else {
                    length = 1;
                }

                prev = c;
            }

            Self::update_length(&mut max_lengths, prev, length);

            max_lengths.iter().sum()
        })
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_substring_in_wrapround_string(p: String) -> i32 {
        Self::find_substring_in_wrapround_string(p)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
