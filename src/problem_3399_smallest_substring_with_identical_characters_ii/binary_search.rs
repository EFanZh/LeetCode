pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn check(lengths: &[u32], mut num_ops: u32, max_length: u32) -> bool {
        if max_length == 1 {
            [false, true].iter().copied().any(|mut flip| {
                let mut num_ops = num_ops;

                for &length in lengths {
                    let required = u32::midpoint(u32::from(flip), length);

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
        let num_ops = num_ops.cast_unsigned();
        let mut lengths = Vec::with_capacity(s.len());
        let mut max_length = 0;
        let mut length = 0;
        let mut prev = 0;

        for c in s.bytes() {
            if c == prev {
                length += 1;
            } else {
                lengths.push(length);
                max_length = u32::max(max_length, length);
                length = 1;
            }

            prev = c;
        }

        lengths.push(length);
        max_length = u32::max(max_length, length);

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

        left.cast_signed()
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
