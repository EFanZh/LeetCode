pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn find_winning_player(skills: Vec<i32>, k: i32) -> i32 {
        let skills = skills.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let mut iter = skills.iter().copied();
        let k = k.cast_unsigned();
        let mut best_index = 0;
        let mut best_skill = iter.next().unwrap();
        let mut best_wins = 0;

        for (index, skill) in (1..).zip(iter) {
            if skill > best_skill {
                best_index = index;
                best_skill = skill;
                best_wins = 1;
            } else {
                best_wins += 1;
            }

            if best_wins >= k {
                break;
            }
        }

        best_index
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_winning_player(skills: Vec<i32>, k: i32) -> i32 {
        Self::find_winning_player(skills, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
