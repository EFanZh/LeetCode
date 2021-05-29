pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn construct_rectangle(area: i32) -> Vec<i32> {
        (f64::from(area).sqrt().ceil() as _..area)
            .find(|length| area % length == 0)
            .map_or_else(|| [area, 1], |length| [length, area / length])
            .to_vec()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn construct_rectangle(area: i32) -> Vec<i32> {
        Self::construct_rectangle(area)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
