pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn fold_max(s: &[u32]) -> u32 {
        s.iter().fold(0, |max, &x| max.max(x))
    }

    pub fn longest_ideal_string(s: String, k: i32) -> i32 {
        let k = k as u32 as usize;
        let mut cache = [0_u32; 26];

        for c in s.into_bytes() {
            let c = usize::from(c - b'a');

            cache[c] = Self::fold_max(&cache[c.saturating_sub(k)..=(c + k).min(25)]) + 1;
        }

        Self::fold_max(&cache) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn longest_ideal_string(s: String, k: i32) -> i32 {
        Self::longest_ideal_string(s, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
