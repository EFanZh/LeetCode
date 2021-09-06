pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    fn dfs(k: u16, modulus: u16, suffix: u16, visited: &mut [bool], result: &mut String) {
        let base = suffix * k;

        for i in 0..k {
            let edge = base + i;

            if !mem::replace(&mut visited[usize::from(edge)], true) {
                Self::dfs(k, modulus, edge % modulus, visited, result);
            }
        }

        result.push(char::from(b'0' + (suffix % k) as u8));
    }

    pub fn crack_safe(n: i32, k: i32) -> String {
        let n = n as u16;
        let k = k as u16;

        if n == 1 {
            "0123456789"[..usize::from(k)].into()
        } else {
            let total = k.pow(n.into());
            let mut result = String::with_capacity((total + n - 1) as _);
            let mut visited = vec![false; total as _];

            Self::dfs(k, total / k, 0, &mut visited, &mut result);

            for _ in 2..n {
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
