pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn new21_game(n: i32, k: i32, max_pts: i32) -> f64 {
        let (n, k, max_pts) = (n as u16, k as u16, max_pts as u16);

        if k == 0 || max_pts <= n - k + 1 {
            1.0
        } else {
            let mut cache = vec![0.0; usize::from(k)];
            let split = usize::from(k.saturating_sub(max_pts));

            for i in (split..usize::from(k)).rev() {
                let sum = cache[i + 1..].iter().sum::<f64>() + f64::from((max_pts + 1 + i as u16 - k).min(n - k + 1));

                cache[i] = sum / f64::from(max_pts);
            }

            for i in (0..split).rev() {
                let sum = cache[i + 1..=i + usize::from(max_pts)].iter().sum::<f64>();

                cache[i] = sum / f64::from(max_pts);
            }

            cache[0]
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn new21_game(n: i32, k: i32, max_pts: i32) -> f64 {
        Self::new21_game(n, k, max_pts)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
