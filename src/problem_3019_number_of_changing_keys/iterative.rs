pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn count_key_changes(s: String) -> i32 {
        let mut iter = s.bytes().map(|c| c & !(1 << 5));

        iter.next()
            .map_or(0, |mut prev| {
                iter.fold(0, |count, c| count + u32::from(c != mem::replace(&mut prev, c)))
            })
            .cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_key_changes(s: String) -> i32 {
        Self::count_key_changes(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
