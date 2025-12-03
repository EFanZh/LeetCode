pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn iter_groups(powers: &[u32], mut f: impl FnMut(u32, u64)) {
        let mut prev_power = 0_u32;
        let mut prev_power_sum = 0_u64;

        for &power in powers {
            if power == prev_power {
                prev_power_sum += u64::from(power);
            } else {
                f(prev_power, prev_power_sum);

                prev_power = power;
                prev_power_sum = u64::from(power);
            }
        }

        f(prev_power, prev_power_sum);
    }

    pub fn maximum_total_damage(power: Vec<i32>) -> i64 {
        let mut power = power.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();

        power.sort_unstable();

        let mut power_queue = [0_u32; 4];
        let mut damage_queue = [0_u64; 4];
        let mut cursor = 0;

        Self::iter_groups(&power, |power, damage| {
            let diff = power - power_queue[cursor & 3];

            let offset = if diff < 3 {
                if diff == 1 && power - power_queue[(cursor + 3) & 3] == 2 {
                    2
                } else {
                    3
                }
            } else {
                0
            };

            let damage = damage_queue[cursor & 3].max(damage_queue[(cursor + offset) & 3] + damage);

            cursor += 1;

            power_queue[cursor & 3] = power;
            damage_queue[cursor & 3] = damage;
        });

        damage_queue[cursor & 3].cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_total_damage(power: Vec<i32>) -> i64 {
        Self::maximum_total_damage(power)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
