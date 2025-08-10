pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn helper(num_arrows: u32, alice_arrows: &mut [i32; 12]) {
        const CONFIGURATIONS: usize = 1 << 11;

        let mut arrow_sums = [0_u32; CONFIGURATIONS];
        let mut scores = [0_u8; CONFIGURATIONS];
        let mut max_score = 0;
        let mut max_score_configuration = 0;
        let mut max_score_arrows = 0;

        for x in alice_arrows.iter_mut() {
            *x += 1;
        }

        for configuration in 1..CONFIGURATIONS {
            let i = configuration.trailing_zeros() as usize;
            let prev = configuration & (configuration - 1);
            let arrow_sum = arrow_sums[prev] + alice_arrows[i + 1] as u32;

            arrow_sums[configuration] = arrow_sum;

            let score = scores[prev] + i as u8 + 1;

            scores[configuration] = score;

            if arrow_sum <= num_arrows && score > max_score {
                max_score = score;
                max_score_configuration = configuration;
                max_score_arrows = arrow_sum;
            }
        }

        alice_arrows[0] = (num_arrows - max_score_arrows) as _;

        alice_arrows[1..].iter_mut().enumerate().for_each(|(i, count)| {
            if max_score_configuration & (1 << i) == 0 {
                *count = 0;
            }
        });
    }

    pub fn maximum_bob_points(num_arrows: i32, alice_arrows: Vec<i32>) -> Vec<i32> {
        let mut alice_arrows = alice_arrows;

        Self::helper(num_arrows as _, alice_arrows.as_mut_slice().try_into().ok().unwrap());

        alice_arrows
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_bob_points(num_arrows: i32, alice_arrows: Vec<i32>) -> Vec<i32> {
        Self::maximum_bob_points(num_arrows, alice_arrows)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
