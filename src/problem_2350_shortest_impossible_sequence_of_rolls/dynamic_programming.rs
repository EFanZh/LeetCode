pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn shortest_sequence(rolls: Vec<i32>, k: i32) -> i32 {
        let k = k as u32;
        let mut states = vec![false; k as usize].into_boxed_slice();
        let mut current_count_count = k;
        let mut current_count_state = false;
        let mut result = 1;

        for roll in rolls {
            let state = &mut states[roll as u32 as usize - 1];

            if *state == current_count_state {
                *state = !current_count_state;

                if current_count_count == 1 {
                    current_count_count = k;
                    current_count_state = !current_count_state;
                    result += 1;
                } else {
                    current_count_count -= 1;
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn shortest_sequence(rolls: Vec<i32>, k: i32) -> i32 {
        Self::shortest_sequence(rolls, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
