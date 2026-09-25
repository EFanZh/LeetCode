pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

enum State {
    Zero,
    One(u8),
    Two(u8),
    Three,
    Bad,
}

impl Solution {
    pub fn count_special_integers(nums: Vec<i32>) -> i32 {
        let mut states = [const { State::Zero }; 100];

        (0..).zip(nums).for_each(|(i, num)| {
            let state = &mut states[num.cast_unsigned() as usize - 1];

            *state = match *state {
                State::Zero => State::One(i),
                State::One(prev) => State::Two(i * 2 - prev),
                State::Two(expected) => {
                    if i == expected {
                        State::Three
                    } else {
                        State::Bad
                    }
                }
                State::Three => State::Bad,
                State::Bad => return,
            }
        });

        states.iter().filter(|state| matches!(state, State::Three)).count() as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_special_integers(nums: Vec<i32>) -> i32 {
        Self::count_special_integers(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
