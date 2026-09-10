pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn count_special_integers(nums: Vec<i32>) -> i32 {
        let mut states = [0_u8; 100];

        for (i, num) in (1..).zip(nums) {
            let state = &mut states[num.cast_unsigned() as usize - 1];

            *state = if *state == 0 || *state + 1 == i { i } else { u8::MAX };
        }

        states.iter().filter(|&&state| state != 0 && state != u8::MAX).count() as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_special_integers(nums: Vec<i32>) -> i32 {
        Self::count_special_integers(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
