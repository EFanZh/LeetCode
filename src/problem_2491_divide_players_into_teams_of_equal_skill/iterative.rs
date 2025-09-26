pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn helper(n: usize, total_skill: u32, min_skill: u32, counts: &[u32; 2048]) -> Option<u64> {
        let pairs = n as u32 / 2;

        if !total_skill.is_multiple_of(pairs) {
            return None;
        }

        let pair_skill_sum = total_skill / pairs;
        let half_pair_skill_sum = pair_skill_sum.div_ceil(2);
        let left = &counts[min_skill as usize..half_pair_skill_sum as usize];

        let right =
            &counts[(pair_skill_sum + 1 - half_pair_skill_sum) as usize..=(pair_skill_sum - min_skill) as usize];

        let mut chemistry = (min_skill..).zip(left.iter().zip(right.iter().rev())).try_fold(
            0_u64,
            |chemistry, (left_skill, (&left_count, &right_count))| {
                (left_count == right_count).then(|| {
                    chemistry + u64::from(left_skill) * u64::from(pair_skill_sum - left_skill) * u64::from(left_count)
                })
            },
        )?;

        if pair_skill_sum.is_multiple_of(2) {
            chemistry += u64::from(half_pair_skill_sum).pow(2) * u64::from(counts[half_pair_skill_sum as usize] / 2);
        }

        Some(chemistry)
    }

    pub fn divide_players(skill: Vec<i32>) -> i64 {
        let n = skill.len();
        let mut total_skill = 0;
        let mut min_skill = u32::MAX;
        let mut counts = [0_u32; 2048];

        for skill in skill {
            let skill = skill as u32;

            total_skill += skill;
            min_skill = min_skill.min(skill);
            counts[skill as usize & (counts.len() - 1)] += 1;
        }

        Self::helper(n, total_skill, min_skill, &counts).unwrap_or(u64::MAX) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn divide_players(skill: Vec<i32>) -> i64 {
        Self::divide_players(skill)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
