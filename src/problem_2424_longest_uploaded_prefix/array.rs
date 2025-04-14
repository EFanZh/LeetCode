// ------------------------------------------------------ snip ------------------------------------------------------ //

pub struct LUPrefix {
    states: Box<[i32]>,
}

impl LUPrefix {
    fn new(n: i32) -> Self {
        Self {
            states: vec![0; n as u32 as usize].into_boxed_slice(),
        }
    }

    fn upload(&mut self, video: i32) {
        let video = video as u32 as usize;

        let (start, end) = match (
            self.states.get(video.wrapping_sub(2)).filter(|&&x| x != 0),
            self.states.get(video).filter(|&&x| x != 0),
        ) {
            (None, None) => (video, video),
            (None, Some(&end)) => (video, end as usize),
            (Some(&start), None) => (start as usize, video),
            (Some(&start), Some(&end)) => (start as usize, end as usize),
        };

        self.states[start - 1] = end as _;
        self.states[end - 1] = start as _;
    }

    fn longest(&self) -> i32 {
        self.states[0]
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::LUPrefix for LUPrefix {
    fn new(n: i32) -> Self {
        Self::new(n)
    }

    fn upload(&mut self, video: i32) {
        self.upload(video);
    }

    fn longest(&self) -> i32 {
        self.longest()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::LUPrefix>();
    }
}
