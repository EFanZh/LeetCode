pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn is_reachable_at_time(sx: i32, sy: i32, fx: i32, fy: i32, t: i32) -> bool {
        let (sx, sy, fx, fy, t) = (sx as u32, sy as u32, fx as u32, fy as u32, t as u32);
        let distance = sx.abs_diff(fx).max(sy.abs_diff(fy));

        if distance == 0 { t != 1 } else { t >= distance }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn is_reachable_at_time(sx: i32, sy: i32, fx: i32, fy: i32, t: i32) -> bool {
        Self::is_reachable_at_time(sx, sy, fx, fy, t)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
