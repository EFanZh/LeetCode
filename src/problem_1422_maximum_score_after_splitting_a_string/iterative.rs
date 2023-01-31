pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_score(s: String) -> i32 {
        let (&first, rest) = s.as_bytes().split_first().unwrap();
        let (&last, middle) = rest.split_last().unwrap();
        let mut ones = i32::from(first + last - b'0' * 2);
        let mut gains = i32::from(b'0' + b'1') - i32::from(first) * 2;
        let mut max_gains = gains;

        for &c in middle {
            if c == b'0' {
                gains += 1;
                max_gains = max_gains.max(gains);
            } else {
                gains -= 1;
                ones += 1;
            }
        }

        ones + max_gains
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_score(s: String) -> i32 {
        Self::max_score(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
