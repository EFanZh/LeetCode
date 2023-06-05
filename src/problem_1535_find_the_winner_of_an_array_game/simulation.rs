pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn get_winner(arr: Vec<i32>, k: i32) -> i32 {
        let mut iter = arr.iter().copied();
        let mut max = iter.next().unwrap();
        let mut wins = 0;

        for value in iter {
            if value > max {
                max = value;
                wins = 1;
            } else {
                wins += 1;
            }

            if wins == k {
                break;
            }
        }

        max
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn get_winner(arr: Vec<i32>, k: i32) -> i32 {
        Self::get_winner(arr, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
