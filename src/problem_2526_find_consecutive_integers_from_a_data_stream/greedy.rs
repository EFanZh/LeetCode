// ------------------------------------------------------ snip ------------------------------------------------------ //

pub struct DataStream {
    value: i32,
    k: u32,
    count: u32,
}

impl DataStream {
    fn new(value: i32, k: i32) -> Self {
        Self {
            value,
            k: k as u32,
            count: 0,
        }
    }

    fn consec(&mut self, num: i32) -> bool {
        if num == self.value {
            self.count += 1;
        } else {
            self.count = 0;
        }

        self.count >= self.k
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::DataStream for DataStream {
    fn new(value: i32, k: i32) -> Self {
        Self::new(value, k)
    }

    fn consec(&mut self, num: i32) -> bool {
        self.consec(num)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::DataStream>();
    }
}
