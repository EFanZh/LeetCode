pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn into_array(value: Vec<i32>) -> (u32, u32) {
        let [x, y]: [_; 2] = value.try_into().ok().unwrap();

        (x as _, y as _)
    }

    fn add_to_stack(stack: &mut Vec<(u32, u32)>, room: (u32, u32)) {
        while let Some(&(top_size, _)) = stack.last() {
            if room.1 < top_size {
                break;
            }

            stack.pop();
        }

        stack.push((room.1, room.0));
    }

    fn search_stack(stack: &mut [(u32, u32)], min_size: u32) -> u32 {
        let i = stack.partition_point(|&(size, _)| size >= min_size);

        stack.get(i.wrapping_sub(1)).map_or(u32::MAX, |&(_, id)| id)
    }

    pub fn closest_room(rooms: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut rooms = rooms.into_iter().map(Self::into_array).collect::<Box<_>>();
        let mut result = vec![0_i32; queries.len()];

        let mut queries = queries
            .into_iter()
            .zip(&mut result)
            .map(|(query, target)| {
                let (preferred, min_size) = Self::into_array(query);

                (preferred, min_size, target)
            })
            .collect::<Box<_>>();

        rooms.sort_unstable_by_key(|&(id, _)| id);
        queries.sort_unstable_by_key(|&(preferred, _, _)| preferred);

        let mut stack = Vec::with_capacity(rooms.len());

        // Left to right.

        {
            let mut room_iter = rooms.iter().copied();
            let mut room = room_iter.next().unwrap();
            let mut query_iter = queries.iter_mut().map(|query| (query.0, query.1, &mut *query.2));

            'outer: while let Some((preferred, min_size, target)) = query_iter.next() {
                while room.0 <= preferred {
                    Self::add_to_stack(&mut stack, room);

                    if let Some(next_room) = room_iter.next() {
                        room = next_room;
                    } else {
                        *target = Self::search_stack(&mut stack, min_size) as _;

                        for (_, min_size, target) in query_iter {
                            *target = Self::search_stack(&mut stack, min_size) as _;
                        }

                        break 'outer;
                    }
                }

                *target = Self::search_stack(&mut stack, min_size) as _;
            }

            stack.clear();
        }

        // Right to left.

        {
            let mut room_iter = rooms.iter().copied();
            let mut room = room_iter.next_back().unwrap();
            let mut query_iter = queries.iter_mut().map(|query| (query.0, query.1, &mut *query.2));

            let update_target = |target: &mut _, stack: &mut _, preferred, min_size| {
                let candidate = Self::search_stack(stack, min_size);

                if *target == -1 || candidate - preferred < preferred - *target as u32 {
                    *target = candidate as _;
                }
            };

            'outer: while let Some((preferred, min_size, target)) = query_iter.next_back() {
                while room.0 > preferred {
                    Self::add_to_stack(&mut stack, room);

                    if let Some(next_room) = room_iter.next_back() {
                        room = next_room;
                    } else {
                        update_target(target, &mut stack, preferred, min_size);

                        for (_, min_size, target) in query_iter.rev() {
                            update_target(target, &mut stack, preferred, min_size);
                        }

                        break 'outer;
                    }
                }

                update_target(target, &mut stack, preferred, min_size);
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn closest_room(rooms: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        Self::closest_room(rooms, queries)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
