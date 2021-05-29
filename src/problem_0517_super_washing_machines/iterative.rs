pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn find_min_moves(machines: Vec<i32>) -> i32 {
        let n = machines.len() as i32;
        let sum = machines.iter().sum::<i32>();

        if sum % n == 0 {
            let expected = sum / n;
            let mut extra = 0;

            machines
                .into_iter()
                .map(|num| {
                    let diff = num - expected;

                    extra += diff;

                    extra.abs().max(diff)
                })
                .max()
                .unwrap()
        } else {
            -1
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_min_moves(machines: Vec<i32>) -> i32 {
        Self::find_min_moves(machines)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
