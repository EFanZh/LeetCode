pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn div_ceil(lhs: u32, rhs: u32) -> u32 {
        let result = lhs / rhs;

        if lhs % rhs == 0 {
            result
        } else {
            result + 1
        }
    }

    fn check(piles: &[i32], h: u32, speed: u32) -> bool {
        piles.iter().map(|&pile| Self::div_ceil(pile as _, speed)).sum::<u32>() > h
    }

    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let h = h as u32;
        let mut total = 0;
        let mut max = 0;

        for &pile in &piles {
            let pile = pile as u32;

            total += pile;
            max = max.max(pile);
        }

        let mut low = Self::div_ceil(total, h);
        let mut high = max;

        while low < high {
            let middle = (low + high) / 2;

            if Self::check(&piles, h, middle) {
                low = middle + 1;
            } else {
                high = middle;
            }
        }

        low as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        Self::min_eating_speed(piles, h)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
