pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::{HashSet, VecDeque};

enum State {
    Empty,
    Infected,
    Contained,
}

#[derive(Default)]
struct Region {
    frontier: HashSet<(usize, usize)>,
    threatens: HashSet<(usize, usize)>,
    walls: u32,
}

struct ObjectPool<T>(Vec<T>);

impl<T: Default> ObjectPool<T> {
    fn allocate(&mut self) -> T {
        self.0.pop().unwrap_or_default()
    }

    fn free(&mut self, value: T) {
        self.0.push(value);
    }
}

impl Solution {
    fn bfs(
        states: &mut [State],
        rows: usize,
        columns: usize,
        queue: &mut VecDeque<(usize, usize)>,
        visited: &mut HashSet<(usize, usize)>,
        regions: &mut Vec<Region>,
        region_pool: &mut ObjectPool<Region>,
    ) -> u32 {
        for y in 0..rows {
            for x in 0..columns {
                if matches!(states[columns * y + x], State::Infected) && visited.insert((y, x)) {
                    let mut current_y = y;
                    let mut current_x = x;
                    let mut region = region_pool.allocate();

                    loop {
                        for &(next_y, next_x) in &[
                            (current_y.wrapping_sub(1), current_x),
                            (current_y, current_x.wrapping_sub(1)),
                            (current_y, current_x + 1),
                            (current_y + 1, current_x),
                        ] {
                            if next_y < rows && next_x < columns {
                                match states[columns * next_y + next_x] {
                                    State::Empty => {
                                        region.frontier.insert((current_y, current_x));
                                        region.threatens.insert((next_y, next_x));
                                        region.walls += 1;
                                    }
                                    State::Infected => {
                                        if visited.insert((next_y, next_x)) {
                                            queue.push_back((next_y, next_x));
                                        }
                                    }
                                    State::Contained => {
                                        region.frontier.insert((current_y, current_x));
                                    }
                                }
                            }
                        }

                        if let Some((next_y, next_x)) = queue.pop_front() {
                            current_y = next_y;
                            current_x = next_x;
                        } else {
                            break;
                        }
                    }

                    if region.walls == 0 {
                        for (y, x) in region.frontier.drain() {
                            states[columns * y + x] = State::Contained;
                        }

                        region_pool.free(region);
                    } else {
                        regions.push(region);
                    }
                }
            }
        }

        if let Some((i, region)) = regions
            .iter()
            .enumerate()
            .max_by_key(|(_, region)| region.threatens.len())
        {
            let result = region.walls;

            // Contain the region that threats most cells.

            for &(y, x) in &region.frontier {
                states[columns * y + x] = State::Contained;
            }

            // Expand the rest infected region.

            for &regions in &[&regions[..i], &regions[i + 1..]] {
                for region in regions {
                    for &(y, x) in &region.threatens {
                        states[columns * y + x] = State::Infected;
                    }
                }
            }

            // Free memory.

            visited.clear();

            for mut region in regions.drain(..) {
                region.frontier.clear();
                region.threatens.clear();
                region.walls = 0;

                region_pool.free(region);
            }

            result
        } else {
            0
        }
    }

    pub fn contain_virus(is_infected: Vec<Vec<i32>>) -> i32 {
        let rows = is_infected.len();
        let columns = is_infected.first().map_or(0, Vec::len);

        let mut states = is_infected
            .into_iter()
            .flatten()
            .map(|value| if value == 0 { State::Empty } else { State::Infected })
            .collect::<Vec<_>>();

        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut regions = Vec::new();
        let mut region_pool = ObjectPool(Vec::new());
        let mut result = 0;

        loop {
            let walls = Self::bfs(
                &mut states,
                rows,
                columns,
                &mut queue,
                &mut visited,
                &mut regions,
                &mut region_pool,
            );

            if walls == 0 {
                break;
            }

            result += walls;
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn contain_virus(is_infected: Vec<Vec<i32>>) -> i32 {
        Self::contain_virus(is_infected)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
