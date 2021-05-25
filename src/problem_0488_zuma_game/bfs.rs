pub struct Solution;

use std::collections::{HashSet, VecDeque};
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy, Eq)]
struct State {
    buffer: [u8; 21],
    sizes: u8,
}

impl State {
    fn normalize_board(board: &mut [u8]) {
        let half = board.len() / 2;

        if board[board.len() - half..].iter().rev().lt(&board[..half]) {
            board.reverse();
        }
    }

    fn normalize_hand(hand: &mut [u8]) {
        hand.sort_unstable();
    }

    fn new(board: &[u8], hand: &[u8]) -> Self {
        let mut buffer = [0; 21];
        let board_buffer = &mut buffer[..board.len()];

        board_buffer.copy_from_slice(board);

        Self::normalize_board(board_buffer);

        let hand_buffer = &mut buffer[board.len()..board.len() + hand.len()];

        hand_buffer.copy_from_slice(hand);

        Self::normalize_hand(hand_buffer);

        Self {
            buffer,
            sizes: ((hand.len() as u8) << 5) | (board.len() as u8),
        }
    }

    fn board_length(&self) -> usize {
        usize::from(self.sizes & 31)
    }

    fn hand_length(&self) -> usize {
        usize::from(self.sizes >> 5)
    }

    fn fields(&self) -> (&[u8], &[u8]) {
        let board_length = self.board_length();

        self.buffer[..board_length + self.hand_length()].split_at(board_length)
    }

    #[allow(clippy::filter_map)]
    fn nexts(&self) -> impl Iterator<Item = Self> + '_ {
        let (board, hand) = self.fields();

        hand.iter()
            .copied()
            .enumerate()
            .filter(move |&(i, insert_color)| hand.get(i.wrapping_sub(1)).copied() != Some(insert_color))
            .flat_map(move |(i, insert_color)| {
                let next_hand = (&hand[..i], &hand[i + 1..]);
                let next_hand_length = hand.len() - 1;

                (0..=board.len()).map(move |insert_index| {
                    let (board_left, board_right) = board.split_at(insert_index);
                    let mut stack = [(0, 0); 21];
                    let mut stack_length = 0_usize;

                    for color in board_left
                        .iter()
                        .copied()
                        .chain(Some(insert_color))
                        .chain(board_right.iter().copied())
                    {
                        loop {
                            if let Some((top_color, top_count)) = stack.get_mut(stack_length.wrapping_sub(1)) {
                                if *top_color == color {
                                    *top_count += 1;

                                    break;
                                } else if *top_count > 2 {
                                    stack_length -= 1;
                                } else {
                                    stack[stack_length] = (color, 1);
                                    stack_length += 1;

                                    break;
                                }
                            } else {
                                stack[stack_length] = (color, 1);
                                stack_length += 1;

                                break;
                            }
                        }
                    }

                    if stack[stack_length - 1].1 > 2 {
                        stack_length -= 1;
                    }

                    let mut buffer = [0; 21];
                    let mut board_length = 0;

                    for &(color, count) in &stack[..stack_length] {
                        buffer[board_length] = color;
                        board_length += 1;

                        if count == 2 {
                            buffer[board_length] = color;
                            board_length += 1;
                        }
                    }

                    let (board_buffer, rest_buffer) = buffer.split_at_mut(board_length);

                    Self::normalize_board(board_buffer);

                    let hand_buffer = rest_buffer[..next_hand_length].split_at_mut(next_hand.0.len());

                    hand_buffer.0.copy_from_slice(next_hand.0);
                    hand_buffer.1.copy_from_slice(next_hand.1);

                    Self {
                        buffer,
                        sizes: (next_hand_length as u8) << 5 | (board_length as u8),
                    }
                })
            })
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        PartialEq::eq(&self.fields(), &other.fields())
    }
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.fields().hash(state);
    }
}

impl Solution {
    pub fn find_min_step(board: String, hand: String) -> i32 {
        let mut depth = 1;
        let node = State::new(board.as_bytes(), hand.as_bytes());
        let mut queue = VecDeque::from(vec![node]);
        let mut visited = Some(node).into_iter().collect::<HashSet<_>>();

        loop {
            for _ in 0..queue.len() {
                let state = queue.pop_front().unwrap();

                for next in state.nexts() {
                    if next.board_length() == 0 {
                        return depth;
                    }

                    if visited.insert(next) {
                        queue.push_back(next);
                    }
                }
            }

            if queue.is_empty() {
                break;
            }

            depth += 1;
        }

        -1
    }
}

impl super::Solution for Solution {
    fn find_min_step(board: String, hand: String) -> i32 {
        Self::find_min_step(board, hand)
    }
}

#[cfg(test)]
mod tests {
    use super::State;

    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }

    #[test]
    fn test_state() {
        let lhs = State::new(b"BGRWY", b"BGRWY");
        let rhs = Clone::clone(&lhs);

        assert!(lhs == rhs);
    }
}
