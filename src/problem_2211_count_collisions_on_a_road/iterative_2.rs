pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn count_collisions(directions: String) -> i32 {
        let directions = directions.into_bytes();
        let left = directions.iter().take_while(|&&c| c == b'L').count();
        let right = directions.iter().rev().take_while(|&&c| c == b'R').count();

        directions[left..directions.len() - right]
            .iter()
            .filter(|&&c| c != b'S')
            .count() as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_collisions(directions: String) -> i32 {
        Self::count_collisions(directions)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
