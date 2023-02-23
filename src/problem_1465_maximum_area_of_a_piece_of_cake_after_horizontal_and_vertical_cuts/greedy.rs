pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn find_max_interval(mut cuts: Vec<i32>, last: i32) -> u64 {
        cuts.sort_unstable_by_key(|&x| x as u32);

        let mut result = 0;
        let mut prev = 0;

        for cut in cuts {
            let cut = cut as u32;

            result = result.max(cut - prev);
            prev = cut;
        }

        u64::from(result.max(last as u32 - prev))
    }

    pub fn max_area(h: i32, w: i32, horizontal_cuts: Vec<i32>, vertical_cuts: Vec<i32>) -> i32 {
        (Self::find_max_interval(horizontal_cuts, h) * Self::find_max_interval(vertical_cuts, w) % 1_000_000_007) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_area(h: i32, w: i32, horizontal_cuts: Vec<i32>, vertical_cuts: Vec<i32>) -> i32 {
        Self::max_area(h, w, horizontal_cuts, vertical_cuts)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
