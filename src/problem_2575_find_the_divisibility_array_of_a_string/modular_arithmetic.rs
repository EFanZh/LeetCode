pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::num::NonZeroU64;

impl Solution {
    pub fn divisibility_array(word: String, m: i32) -> Vec<i32> {
        let mut value = 0;
        let m = NonZeroU64::new(u64::from(m as u32)).unwrap();

        word.into_bytes()
            .into_iter()
            .map(|c| {
                value = (value * 10 + u64::from(c - b'0')) % m;

                i32::from(value == 0)
            })
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn divisibility_array(word: String, m: i32) -> Vec<i32> {
        Self::divisibility_array(word, m)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
