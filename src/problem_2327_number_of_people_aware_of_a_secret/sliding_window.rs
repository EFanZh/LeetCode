pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    const MODULUS: u32 = 1_000_000_007;

    fn assign_add_sub(target: &mut u32, add: u32, subtract: u32) {
        *target += add;
        *target += Self::MODULUS - subtract;

        if *target >= Self::MODULUS {
            *target -= Self::MODULUS;
        }

        if *target >= Self::MODULUS {
            *target -= Self::MODULUS;
        }
    }

    pub fn people_aware_of_secret(n: i32, delay: i32, forget: i32) -> i32 {
        let n = n as u32 as usize;

        assert!(n <= 1000);

        let delay = delay as u32 as usize;
        let forget = forget as u32 as usize;
        let mut cache = [0_u32; 1000];

        cache[0] = 1;

        let mut sum = 0_u32;
        let new_offset = delay;
        let old_offset = forget;

        for day in 1..n {
            let spreading_count = cache.get(day.wrapping_sub(old_offset)).copied().unwrap_or(0);

            Self::assign_add_sub(
                &mut sum,
                cache.get(day.wrapping_sub(new_offset)).copied().unwrap_or(0),
                spreading_count,
            );

            cache[day] = sum;
        }

        cache[n - forget..n].iter().fold(0_u32, |mut sum, &x| {
            sum += x;

            sum.checked_sub(Self::MODULUS).unwrap_or(sum)
        }) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn people_aware_of_secret(n: i32, delay: i32, forget: i32) -> i32 {
        Self::people_aware_of_secret(n, delay, forget)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
