pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn flip(n: i32) -> i32 {
        let start = 1 << (31 - n.leading_zeros());
        let end = (start << 1) - 1;

        end - (n - start)
    }

    pub fn path_in_zig_zag_tree(label: i32) -> Vec<i32> {
        let digits = 32 - label.leading_zeros();
        let label = if digits & 1 == 0 { Self::flip(label) } else { label };
        let mut result = Vec::with_capacity(digits as _);
        let mut probe = 1 << (digits - 1);

        probe >>= 1;
        result.push(1);

        let mut prev = 1_i32;

        while probe != 0 {
            prev *= 2;

            if label & probe != 0 {
                prev += 1;
            }

            probe >>= 1;
            result.push(Self::flip(prev));

            if probe == 0 {
                break;
            }

            prev *= 2;

            if label & probe != 0 {
                prev += 1;
            }

            probe >>= 1;
            result.push(prev);
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn path_in_zig_zag_tree(label: i32) -> Vec<i32> {
        Self::path_in_zig_zag_tree(label)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
