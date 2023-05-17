pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn get_maximum_generated(n: i32) -> i32 {
        let n = n as u8;

        i32::from(if n < 7 {
            (n + 1) / 2
        } else {
            let mut cache = [0_u8; 101];

            cache[0] = 0;
            cache[1] = 1;
            cache[2] = 1;
            cache[3] = 2;
            cache[4] = 1;
            cache[5] = 3;
            cache[6] = 2;
            cache[7] = 3;

            let cache = &mut cache[..=usize::from(n)];
            let mut i = 8;
            let mut result = 3;

            while i < cache.len() {
                // Even index.

                let value = cache[i / 2];

                cache[i] = value;
                result = result.max(value);

                i += 1;

                if i < cache.len() {
                    // Odd index.

                    let value = cache[i / 2] + cache[i / 2 + 1];

                    cache[i] = value;
                    result = result.max(value);

                    i += 1;
                } else {
                    break;
                }
            }

            result
        })
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn get_maximum_generated(n: i32) -> i32 {
        Self::get_maximum_generated(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
