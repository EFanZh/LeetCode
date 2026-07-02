pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn toggle_light_bulbs(bulbs: Vec<i32>) -> Vec<i32> {
        let mut bulbs = bulbs;
        let mut states = [false; 100];

        for bulb in &mut bulbs {
            let state = &mut states[bulb.cast_unsigned() as usize - 1];

            *state = !*state;
        }

        bulbs.clear();
        bulbs.extend((1..).zip(states).filter_map(|(i, on)| on.then_some(i)));

        bulbs
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn toggle_light_bulbs(bulbs: Vec<i32>) -> Vec<i32> {
        Self::toggle_light_bulbs(bulbs)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
