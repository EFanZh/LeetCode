pub struct Solution;

impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        let mut s = s.into_bytes();

        s.chunks_mut(k as usize).step_by(2).for_each(<[_]>::reverse);

        String::from_utf8(s).unwrap()
    }
}

impl super::Solution for Solution {
    fn reverse_str(s: String, k: i32) -> String {
        Self::reverse_str(s, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
