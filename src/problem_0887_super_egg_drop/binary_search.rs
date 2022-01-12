pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn max_eggs_required(floors: u32) -> u32 {
        31 - (floors + 1).leading_zeros()
    }

    fn min_moves_required(floors: u32) -> u32 {
        32 - floors.leading_zeros()
    }

    fn check_moves(eggs: u32, moves: u32, floors: u32) -> bool {
        // See <https://oeis.org/A008949>.

        let mut ability = moves;
        let mut combinations = moves;
        let mut i = 1;

        loop {
            if ability >= floors {
                return true;
            } else if i == eggs {
                return false;
            }

            combinations *= moves - i;
            i += 1;
            combinations /= i;
            ability += combinations;
        }
    }

    pub fn super_egg_drop(k: i32, n: i32) -> i32 {
        let k = k as u32;
        let n = n as u32;

        if k < Self::max_eggs_required(n) {
            let mut low = 3;
            let mut high = n;

            while low < high {
                let middle = (low + high) / 2;

                if Self::check_moves(k, middle, n) {
                    high = middle;
                } else {
                    low = middle + 1;
                }
            }

            low as _
        } else {
            Self::min_moves_required(n) as _
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn super_egg_drop(k: i32, n: i32) -> i32 {
        Self::super_egg_drop(k, n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
