pub struct Solution;

impl Solution {
    #[allow(clippy::many_single_char_names, clippy::too_many_arguments)]
    pub fn compute_area(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32, h: i32) -> i32 {
        let left = a.max(e);
        let right = c.min(g);

        if left < right {
            let bottom = b.max(f);
            let top = d.min(h);

            if bottom < top {
                (c - a) * (d - b) + (g - e) * (h - f) - (right - left) * (top - bottom)
            } else {
                (c - a) * (d - b) + (g - e) * (h - f)
            }
        } else {
            (c - a) * (d - b) + (g - e) * (h - f)
        }
    }
}

impl super::Solution for Solution {
    #[allow(clippy::many_single_char_names)]
    fn compute_area(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32, h: i32) -> i32 {
        Self::compute_area(a, b, c, d, e, f, g, h)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
