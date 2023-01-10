pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn can_construct(s: String, k: i32) -> bool {
        let n = s.len();
        let k = k as usize;
        let mut counts = [0_u32; 26];

        for c in s.into_bytes() {
            counts[usize::from(c) - usize::from(b'a')] += 1;
        }

        let odd = counts.iter().filter(|&&count| count % 2 != 0).count();

        odd <= k && k <= n
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn can_construct(s: String, k: i32) -> bool {
        Self::can_construct(s, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
