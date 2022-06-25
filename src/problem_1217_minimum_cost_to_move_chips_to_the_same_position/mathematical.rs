pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_cost_to_move_chips(position: Vec<i32>) -> i32 {
        let mut even = 0;
        let mut odd = 0;

        for position in position {
            if position & 1 == 0 {
                even += 1;
            } else {
                odd += 1;
            }
        }

        even.min(odd)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_cost_to_move_chips(position: Vec<i32>) -> i32 {
        Self::min_cost_to_move_chips(position)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
