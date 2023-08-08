pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn sort_2(a: &mut u32, b: &mut u32) {
        let (new_a, new_b) = if *b < *a { (*b, *a) } else { (*a, *b) };

        *a = new_a;
        *b = new_b;
    }

    fn sort_3(a: &mut u32, b: &mut u32, c: &mut u32) {
        Self::sort_2(a, b);
        Self::sort_2(b, c);
        Self::sort_2(a, b);
    }

    pub fn maximum_score(a: i32, b: i32, c: i32) -> i32 {
        let mut a = a as u32;
        let mut b = b as u32;
        let mut c = c as u32;

        Self::sort_3(&mut a, &mut b, &mut c);

        let a_plus_b = a + b;

        (if a_plus_b < c { a_plus_b } else { (a_plus_b + c) / 2 }) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_score(a: i32, b: i32, c: i32) -> i32 {
        Self::maximum_score(a, b, c)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
