pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn find_array(pref: Vec<i32>) -> Vec<i32> {
        let mut prev_xor = 0;
        let mut pref = pref;

        for target in &mut pref {
            let value = *target ^ prev_xor;

            prev_xor = mem::replace(target, value);
        }

        pref
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_array(pref: Vec<i32>) -> Vec<i32> {
        Self::find_array(pref)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
