pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Reverse;

impl Solution {
    fn helper(slice: &mut [u8], merge_buffer: &mut [u8], sort_buffer: &mut Vec<(usize, usize)>) {
        let saved_sort_buffer_length = sort_buffer.len();
        let mut stack = 0;
        let mut start = 0;

        for i in 0..slice.len() {
            if slice[i] == b'0' {
                stack -= 1;

                if stack == 0 {
                    sort_buffer.push((start, i + 1));
                    Self::helper(&mut slice[start + 1..i], merge_buffer, sort_buffer);
                    start = i + 1;
                }
            } else {
                stack += 1;
            }
        }

        let frame = &mut sort_buffer[saved_sort_buffer_length..];

        frame.sort_unstable_by_key(|&(left, right)| Reverse(&slice[left..right]));

        let mut i = 0;

        for &(left, right) in &*frame {
            let length = right - left;

            merge_buffer[i..i + length].copy_from_slice(&slice[left..right]);
            i += length;
        }

        slice.copy_from_slice(&merge_buffer[..slice.len()]);

        sort_buffer.truncate(saved_sort_buffer_length);
    }

    pub fn make_largest_special(s: String) -> String {
        let mut s = s.into_bytes();
        let n = s.len();

        Self::helper(&mut s, &mut vec![0; n], &mut Vec::new());

        String::from_utf8(s).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn make_largest_special(s: String) -> String {
        Self::make_largest_special(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
