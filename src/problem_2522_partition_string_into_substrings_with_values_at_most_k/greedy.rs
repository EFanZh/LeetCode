pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn minimum_partition(s: String, k: i32) -> i32 {
        let k = k as u32;

        if k < 10 {
            let target = b'0' + k as u8;

            if s.bytes().any(|c| target < c) {
                -1
            } else {
                s.len() as _
            }
        } else {
            let mut num = 0_u32;
            let mut result = 1;

            for c in s.bytes() {
                let c = u32::from(c - b'0');

                num = num
                    .checked_mul(10)
                    .and_then(|value| value.checked_add(c))
                    .unwrap_or(u32::MAX);

                if k < num {
                    num = c;
                    result += 1;
                }
            }

            result
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_partition(s: String, k: i32) -> i32 {
        Self::minimum_partition(s, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
