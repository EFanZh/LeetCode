pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::iter;

impl Solution {
    pub fn divide_string(s: String, k: i32, fill: char) -> Vec<String> {
        let n = s.len();
        let k = k as u32 as usize;
        let mut prev = 0;
        let mut result = Vec::with_capacity(s.len().div_ceil(k));

        while let Some(s) = s.get(prev..prev + k) {
            result.push(s.to_string());

            prev += k;
        }

        if n != prev {
            let mut group = s[prev..].to_string();

            group.extend(iter::repeat_n(fill, k - (n - prev)));

            result.push(group);
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn divide_string(s: String, k: i32, fill: char) -> Vec<String> {
        Self::divide_string(s, k, fill)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
