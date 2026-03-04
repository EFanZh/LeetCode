pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn check(lengths: &[u16], mut num_ops: u16, max_length: u16) -> bool {
        if max_length == 1 {
            [false, true].iter().copied().any(|mut flip| {
                let mut num_ops = num_ops;

                for &length in lengths {
                    let required = u16::midpoint(u16::from(flip), length);

                    if num_ops < required {
                        return false;
                    }

                    num_ops -= required;
                    flip ^= length % 2 == 0;
                }

                true
            })
        } else {
            for &length in lengths {
                if length > max_length {
                    let required = length / (max_length + 1);

                    if num_ops < required {
                        return false;
                    }

                    num_ops -= required;
                }
            }

            true
        }
    }

    pub fn min_length(s: String, num_ops: i32) -> i32 {
        let num_ops = num_ops as u16;
        let mut lengths = Vec::with_capacity(s.len());
        let mut max_length = 0;
        let mut length = 0;
        let mut prev = 0;

        for c in s.bytes() {
            if c == prev {
                length += 1;
            } else {
                lengths.push(length);
                max_length = u16::max(max_length, length);
                length = 1;
            }

            prev = c;
        }

        lengths.push(length);
        max_length = u16::max(max_length, length);

        let mut left = 1;
        let mut right = max_length;

        while left < right {
            let middle = left.midpoint(right);

            if Self::check(&lengths, num_ops, middle) {
                right = middle;
            } else {
                left = middle + 1;
            }
        }

        i32::from(left)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_length(s: String, num_ops: i32) -> i32 {
        Self::min_length(s, num_ops)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
