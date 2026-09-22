pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn prefix_sums(values: Vec<i32>) -> (Box<[u64]>, u64) {
        let mut sum = 0;

        (
            values
                .into_iter()
                .map(|skill| {
                    sum += u64::from(skill.cast_unsigned());
                    sum
                })
                .collect::<Box<_>>(),
            sum,
        )
    }

    pub fn min_time(skill: Vec<i32>, mana: Vec<i32>) -> i64 {
        let (skill, total_skill) = Self::prefix_sums(skill);
        let mut prev_mana = 0;
        let mut start = 0_u64;

        for mana in mana.into_iter().map(i32::cast_unsigned).map(u64::from) {
            let mut offset = 0;
            let mut prev = 0;

            for &skill in &skill {
                let min_start = prev * mana;
                let prev_end = skill * prev_mana;

                if min_start < prev_end {
                    offset = u64::max(offset, prev_end - min_start);
                }

                prev = skill;
            }

            prev_mana = mana;
            start += offset;
        }

        (start + total_skill * prev_mana).cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_time(skill: Vec<i32>, mana: Vec<i32>) -> i64 {
        Self::min_time(skill, mana)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
