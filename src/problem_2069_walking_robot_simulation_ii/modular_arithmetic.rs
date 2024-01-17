// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::num::{NonZeroU16, NonZeroU32};

pub struct Robot {
    width_minus_1: u16,
    cycle_length: NonZeroU16,
    steps: u16,
}

impl Robot {
    fn new(width: i32, height: i32) -> Self {
        let width_minus_1 = width as u16 - 1;
        let cycle_length = NonZeroU16::new((width as u16 + height as u16) * 2 - 4).unwrap();

        Self {
            width_minus_1,
            cycle_length,
            steps: 0x_8000,
        }
    }

    fn step(&mut self, num: i32) {
        let num = num as u32;
        let steps = u32::from(self.steps & 0x_7fff);

        self.steps = ((steps + num) % NonZeroU32::from(self.cycle_length)) as _;
    }

    fn get_pos(&self) -> Vec<i32> {
        let steps = self.steps & 0x_7fff;
        let half_cycle = self.cycle_length.get() / 2;

        #[allow(clippy::option_if_let_else)] // Suggestion is too ugly.
        let (x, y) = if let Some(second_half) = steps.checked_sub(half_cycle) {
            if let Some(x) = self.width_minus_1.checked_sub(second_half) {
                (x, half_cycle - self.width_minus_1)
            } else {
                (0, half_cycle - second_half)
            }
        } else if let Some(y) = steps.checked_sub(self.width_minus_1) {
            (self.width_minus_1, y)
        } else {
            (steps, 0)
        };

        vec![i32::from(x), i32::from(y)]
    }

    fn get_dir(&self) -> String {
        let direction = if self.steps < 0x_8000 {
            if self.steps == 0 {
                "South"
            } else {
                let steps = self.steps - 1;
                let half_cycle = self.cycle_length.get() / 2;

                #[allow(clippy::option_if_let_else)] // Suggestion is too ugly.
                if let Some(second_half) = steps.checked_sub(half_cycle) {
                    if second_half < self.width_minus_1 {
                        "West"
                    } else {
                        "South"
                    }
                } else if steps < self.width_minus_1 {
                    "East"
                } else {
                    "North"
                }
            }
        } else {
            "East"
        };

        direction.to_string()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Robot for Robot {
    fn new(width: i32, height: i32) -> Self {
        Self::new(width, height)
    }

    fn step(&mut self, num: i32) {
        self.step(num);
    }

    fn get_pos(&self) -> Vec<i32> {
        self.get_pos()
    }

    fn get_dir(&self) -> String {
        self.get_dir()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Robot>();
    }
}
