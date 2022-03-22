pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn str_without3a3b(a: i32, b: i32) -> String {
        let a = a as u32;
        let b = b as u32;

        let (a, b, s1, s2, c) = if b < a {
            (b, a, "aab", "ab", 'a')
        } else {
            (a, b, "bba", "ba", 'b')
        };

        let mut result = String::with_capacity((a + b) as _);

        let extra = b - a;

        let (extra, extra_2) = if let Some(extra_2) = extra.checked_sub(a) {
            (a, extra_2)
        } else {
            (extra, 0)
        };

        for _ in 0..extra {
            result.push_str(s1);
        }

        for _ in 0..(a - extra) {
            result.push_str(s2);
        }

        for _ in 0..extra_2 {
            result.push(c);
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn str_without3a3b(a: i32, b: i32) -> String {
        Self::str_without3a3b(a, b)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
