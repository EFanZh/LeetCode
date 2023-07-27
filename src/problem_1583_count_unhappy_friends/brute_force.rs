pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::convert::TryInto;

struct State {
    rank: Box<[u16]>,
    paired_with: u16,
}

impl Solution {
    pub fn unhappy_friends(n: i32, preferences: Vec<Vec<i32>>, pairs: Vec<Vec<i32>>) -> i32 {
        let n = n as u32 as usize;

        let mut states = preferences
            .into_iter()
            .map(|friends| {
                let mut state = State {
                    rank: vec![u16::MAX; n].into_boxed_slice(),
                    paired_with: 0,
                };

                for (i, friend) in (0..).zip(friends) {
                    state.rank[friend as u32 as usize] = i;
                }

                state
            })
            .collect::<Box<_>>();

        for pair in pairs {
            let [x, y]: [_; 2] = pair.as_slice().try_into().ok().unwrap();

            states[x as u32 as usize].paired_with = y as _;
            states[y as u32 as usize].paired_with = x as _;
        }

        let mut result = 0;

        for (x, x_state) in states.iter().enumerate() {
            let y_rank = x_state.rank[usize::from(x_state.paired_with)];

            for (u_state, &u_rank) in states.iter().zip(&*x_state.rank) {
                if u_rank < y_rank && u_state.rank[x] < u_state.rank[usize::from(u_state.paired_with)] {
                    result += 1;

                    break;
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn unhappy_friends(n: i32, preferences: Vec<Vec<i32>>, pairs: Vec<Vec<i32>>) -> i32 {
        Self::unhappy_friends(n, preferences, pairs)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
