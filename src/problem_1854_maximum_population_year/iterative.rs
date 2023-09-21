pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::convert::TryInto;

impl Solution {
    pub fn maximum_population(logs: Vec<Vec<i32>>) -> i32 {
        const MIN_YEAR: usize = 1950;

        let mut diffs = [0_i8; 101];

        for log in logs {
            let [birth, death]: [_; 2] = log.try_into().ok().unwrap();

            diffs[birth as u32 as usize - MIN_YEAR] += 1;
            diffs[death as u32 as usize - MIN_YEAR] -= 1;
        }

        let mut max_population = 0;
        let mut max_population_year = 0;
        let mut population = 0;

        for (year, diff) in (MIN_YEAR..).zip(diffs) {
            population += diff;

            if population > max_population {
                max_population = population;
                max_population_year = year;
            }
        }

        max_population_year as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_population(logs: Vec<Vec<i32>>) -> i32 {
        Self::maximum_population(logs)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
