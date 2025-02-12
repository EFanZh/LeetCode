pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn robot_with_string(s: String) -> String {
        let mut counts = [0_u32; 26];

        for c in s.bytes() {
            counts[usize::from(c) - usize::from(b'a')] += 1;
        }

        let n = s.len();
        let mut right_min = 0;
        let mut result = Vec::with_capacity(n);
        let mut stack = Vec::with_capacity(n);

        for c in s.bytes() {
            while counts[right_min] == 0 {
                right_min += 1;
            }

            let right_min = b'a' + right_min as u8;

            while let Some(&top) = stack.last().filter(|&&top| top <= right_min) {
                stack.pop();
                result.push(top);
            }

            (if c == right_min { &mut result } else { &mut stack }).push(c);

            counts[usize::from(c) - usize::from(b'a')] -= 1;
        }

        drop(s);

        result.extend(stack.iter().rev().copied());

        String::from_utf8(result).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn robot_with_string(s: String) -> String {
        Self::robot_with_string(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
