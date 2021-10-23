pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

// https://git.sagemath.org/sage.git/tree/src/sage/combinat/debruijn_sequence.pyx#n69.

impl Solution {
    fn helper(t: usize, p: usize, digits: u8, length: usize, a: &mut [u8], result: &mut String) {
        if t > length {
            if length % p == 0 {
                result.extend(a[1..=p].iter().map(|&c| char::from(c)));
            }
        } else {
            a[t] = a[t - p];

            Self::helper(t + 1, p, digits, length, a, result);

            for c in a[t - p] + 1..b'0' + digits {
                a[t] = c;

                Self::helper(t + 1, t, digits, length, a, result);
            }
        }
    }

    pub fn crack_safe(n: i32, k: i32) -> String {
        let n = n as usize;
        let k = k as u8;

        if n == 1 {
            "0123456789"[..usize::from(k)].into()
        } else if k == 1 {
            "0".repeat(n)
        } else {
            let mut a = vec![b'0'; usize::from(k) * n];
            let mut result = String::with_capacity(usize::from(k).pow(n as _) + n - 1);

            Self::helper(1, 1, k, n, &mut a, &mut result);

            for _ in 1..n {
                result.push('0');
            }

            result
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
