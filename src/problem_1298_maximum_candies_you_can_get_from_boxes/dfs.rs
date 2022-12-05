pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

#[derive(Clone)]
enum State {
    NotDiscovered,
    HasBox,
    HasKey,
    Visited,
}

impl Solution {
    pub fn max_candies(
        status: Vec<i32>,
        candies: Vec<i32>,
        keys: Vec<Vec<i32>>,
        contained_boxes: Vec<Vec<i32>>,
        initial_boxes: Vec<i32>,
    ) -> i32 {
        let mut discovery = vec![State::NotDiscovered; status.len()];

        for (state, is_open) in discovery.iter_mut().zip(status) {
            if is_open != 0 {
                *state = State::HasKey;
            }
        }

        let mut new_boxes = initial_boxes.as_slice();
        let mut queue = Vec::new();
        let mut result = 0;

        loop {
            for &new_box in new_boxes {
                let new_box = new_box as usize;
                let state = &mut discovery[new_box];

                if let State::NotDiscovered = state {
                    *state = State::HasBox;
                } else {
                    *state = State::Visited;
                    queue.push(new_box);
                }
            }

            if let Some(current_box) = queue.pop() {
                result += candies[current_box];

                for &key in &keys[current_box] {
                    let key = key as usize;
                    let state = &mut discovery[key];

                    match state {
                        State::NotDiscovered => *state = State::HasKey,
                        State::HasBox => {
                            *state = State::Visited;
                            queue.push(key);
                        }
                        _ => {}
                    }
                }

                new_boxes = &contained_boxes[current_box];
            } else {
                break;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_candies(
        status: Vec<i32>,
        candies: Vec<i32>,
        keys: Vec<Vec<i32>>,
        contained_boxes: Vec<Vec<i32>>,
        initial_boxes: Vec<i32>,
    ) -> i32 {
        Self::max_candies(status, candies, keys, contained_boxes, initial_boxes)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
