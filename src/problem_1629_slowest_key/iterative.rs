pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn slowest_key(release_times: Vec<i32>, keys_pressed: String) -> char {
        let mut time = 0;
        let mut state = 0; // High 56 bits for duration, low 8 bits for key.

        for (&release_time, key) in release_times.iter().zip(keys_pressed.bytes()) {
            state = state.max((u64::from((release_time - time) as u32) << 8) | u64::from(key));
            time = release_time;
        }

        char::from(state as u8)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn slowest_key(release_times: Vec<i32>, keys_pressed: String) -> char {
        Self::slowest_key(release_times, keys_pressed)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
