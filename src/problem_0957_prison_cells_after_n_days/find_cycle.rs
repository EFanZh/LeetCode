pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::convert::TryFrom;

impl Solution {
    pub fn prison_after_n_days(cells: Vec<i32>, n: i32) -> Vec<i32> {
        let n = n as u32;
        let mut cells = cells;
        let mut states = [0_u8; 256];
        let mut days = [u16::MAX; 256];
        let mut day = 0_u8;

        let mut state = cells
            .iter()
            .enumerate()
            .fold(0_u8, |state, (i, &value)| state | (value as u8) << i);

        days[usize::from(state)] = 0;
        states[0] = state;

        let mut final_state = loop {
            if u32::from(day) == n {
                break state;
            }

            state = (!((state << 1) ^ (state >> 1))) & 0b_0111_1110;
            day += 1;

            let value = &mut days[usize::from(state)];

            if let Ok(cycle_start) = u8::try_from(*value) {
                break states[usize::from(cycle_start)
                    + ((n - u32::from(cycle_start)) % u32::from(day - cycle_start)) as usize];
            }

            *value = u16::from(day);
            states[usize::from(day)] = state;
        };

        for target in &mut cells {
            *target = i32::from(final_state & 1);
            final_state >>= 1;
        }

        cells
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn prison_after_n_days(cells: Vec<i32>, n: i32) -> Vec<i32> {
        Self::prison_after_n_days(cells, n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
