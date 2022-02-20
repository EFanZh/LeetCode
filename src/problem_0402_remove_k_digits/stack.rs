pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn remove_kdigits(num: String, k: i32) -> String {
        let mut num = num;
        let n = num.len();
        let k = k as usize;

        if k == n {
            num.replace_range(.., "0");
        } else {
            let length = n - k;
            let mut stack = Vec::with_capacity(length);
            let mut i = 0;

            while i - stack.len() != k {
                let start = i.saturating_sub(k);
                let digit = num.as_bytes()[i];
                let insertion_point = start + stack[start..].partition_point(|&d| d <= digit);

                if let Some(target) = stack.get_mut(insertion_point) {
                    *target = digit;

                    stack.truncate(insertion_point + 1);
                } else if insertion_point != length {
                    stack.push(digit);
                }

                i += 1;
            }

            stack.extend(&num.as_bytes()[i..]);

            if let Some(i) = stack.iter().position(|&d| d != b'0') {
                stack.drain(..i);
            } else {
                stack.truncate(1);
            }

            num = String::from_utf8(stack).unwrap();
        }

        num
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn remove_kdigits(num: String, k: i32) -> String {
        Self::remove_kdigits(num, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
