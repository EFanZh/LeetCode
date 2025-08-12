pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn find_champion(grid: Vec<Vec<i32>>) -> i32 {
        grid.iter()
            .position(|row| row.iter().filter(|&&cell| cell == 0).count() == 1)
            .unwrap() as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_champion(grid: Vec<Vec<i32>>) -> i32 {
        Self::find_champion(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
