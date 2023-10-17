pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn two_egg_drop(n: i32) -> i32 {
        let n = n as u32 as usize + 1;
        let mut cache = vec![0_u16; n].into_boxed_slice();

        for i in 1..n {
            cache[i] = (0..)
                .zip(cache[..i].iter().rev())
                .map(|(x, &y)| x.max(y))
                .min()
                .unwrap()
                + 1;
        }

        i32::from(*cache.last().unwrap())
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn two_egg_drop(n: i32) -> i32 {
        Self::two_egg_drop(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
