pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn smallest_sufficient_team(req_skills: Vec<String>, people: Vec<Vec<String>>) -> Vec<i32> {
        let total_skills = req_skills.len();
        let mut map = HashMap::new();

        for skill in req_skills {
            let id = map.len();

            map.insert(skill, id);
        }

        let people = people
            .into_iter()
            .map(|skills| {
                let mut state = 0_usize;

                for skill in skills {
                    state |= 1 << map[skill.as_str()];
                }

                state
            })
            .collect::<Vec<_>>();

        drop(map);

        let start = (1 << total_skills) - 1;
        let mut visited = vec![false; 1 << total_skills];
        let mut queue = VecDeque::from([(start, 0_u64)]);

        visited[start] = true;

        let mut selection = 'outer: loop {
            for _ in 0..queue.len() {
                let (node, selection) = queue.pop_front().unwrap();
                let mut i = 1;

                for &skills in &people {
                    let next = node & !skills;
                    let next_selection = selection | i;

                    if next == 0 {
                        break 'outer next_selection;
                    }

                    if let visited_value @ false = &mut visited[next] {
                        *visited_value = true;

                        queue.push_back((next, next_selection));
                    }

                    i <<= 1;
                }
            }
        };

        let mut result = Vec::with_capacity(selection.count_ones() as _);

        while selection != 0 {
            result.push(selection.trailing_zeros() as _);
            selection &= selection - 1;
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn smallest_sufficient_team(req_skills: Vec<String>, people: Vec<Vec<String>>) -> Vec<i32> {
        Self::smallest_sufficient_team(req_skills, people)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
