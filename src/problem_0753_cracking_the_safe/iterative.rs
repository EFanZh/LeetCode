pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn crack_safe(n: i32, k: i32) -> String {
        let n = n as u16;
        let k = k as u16;

        if n == 1 {
            "0123456789"[..usize::from(k)].into()
        } else {
            let total = k.pow(n.into());
            let mut result = String::with_capacity((total + n - 1) as _);
            let mut visited = vec![false; total as _];

            for _ in 1..n {
                result.push('0');
            }

            let modulus = total / k;
            let mut suffix = 0;

            'outer: loop {
                let base = suffix * k;

                // Although this algorithm passes all tests, is it really correct? Why is the reversion required?

                for i in (0..k).rev() {
                    let next = base + i;

                    if !mem::replace(&mut visited[usize::from(next)], true) {
                        result.push(char::from(b'0' + (i as u8)));
                        suffix = next % modulus;

                        continue 'outer;
                    }
                }

                return result;
            }
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn crack_safe(n: i32, k: i32) -> String {
        Self::crack_safe(n, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
