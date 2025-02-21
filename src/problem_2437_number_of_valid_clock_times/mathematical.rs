pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn count_time(time: String) -> i32 {
        let [h0, h1, _, m0, m1]: [_; 5] = time.into_bytes().try_into().ok().unwrap();

        let hours = if h0 == b'?' {
            match h1 {
                b'0'..=b'3' => 3,
                b'4'..=b'9' => 2,
                _ => 24,
            }
        } else if h1 == b'?' {
            if h0 == b'2' { 4 } else { 10 }
        } else {
            1
        };

        let minutes = if m0 == b'?' {
            if m1 == b'?' { 60 } else { 6 }
        } else if m1 == b'?' {
            10
        } else {
            1
        };

        hours * minutes
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_time(time: String) -> i32 {
        Self::count_time(time)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
