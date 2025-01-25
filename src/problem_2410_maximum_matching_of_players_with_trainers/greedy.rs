pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn match_players_and_trainers(players: Vec<i32>, trainers: Vec<i32>) -> i32 {
        let mut players = players.into_iter().map(|x| x as u32).collect::<Vec<_>>();
        let mut trainers = trainers.into_iter().map(|x| x as u32).collect::<Vec<_>>();

        players.sort_unstable();
        trainers.sort_unstable();

        let mut trainer_iter = trainers.iter().copied();
        let mut result = 0;

        'outer: for &player in &players {
            loop {
                if let Some(trainer) = trainer_iter.next() {
                    if trainer >= player {
                        break;
                    }
                } else {
                    break 'outer;
                }
            }

            result += 1;
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn match_players_and_trainers(players: Vec<i32>, trainers: Vec<i32>) -> i32 {
        Self::match_players_and_trainers(players, trainers)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
