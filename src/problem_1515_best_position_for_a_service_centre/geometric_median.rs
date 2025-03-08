pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

// See <https://en.wikipedia.org/wiki/Geometric_median>.

use std::f64::consts;

impl Solution {
    fn iterate(positions: &[(f64, f64)], guess: (f64, f64)) -> Option<(f64, f64)> {
        let mut result = (0.0, 0.0);
        let mut denominator = 0.0;

        for &(x, y) in positions {
            let distance = (guess.0 - x).hypot(guess.1 - y);

            if distance < f64::EPSILON {
                return None;
            }

            result.0 += x / distance;
            result.1 += y / distance;
            denominator += 1.0 / distance;
        }

        result.0 /= denominator;
        result.1 /= denominator;

        Some(result)
    }

    pub fn get_min_dist_sum(positions: Vec<Vec<i32>>) -> f64 {
        const INITIAL_GUESS: (f64, f64) = (consts::E + consts::PI, consts::E / consts::PI);

        let positions = positions
            .into_iter()
            .map(|position| {
                let [x, y] = position.try_into().ok().unwrap();

                (f64::from(x), f64::from(y))
            })
            .collect::<Box<_>>();

        let mut guess = INITIAL_GUESS;

        while let Some(better_guess) = Self::iterate(&positions, guess) {
            let dx = better_guess.0 - guess.0;
            let dy = better_guess.1 - guess.1;

            guess = better_guess;

            if dx.mul_add(dx, dy * dy) < f64::EPSILON {
                break;
            }
        }

        positions.iter().map(|&(x, y)| (guess.0 - x).hypot(guess.1 - y)).sum()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn get_min_dist_sum(positions: Vec<Vec<i32>>) -> f64 {
        Self::get_min_dist_sum(positions)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
