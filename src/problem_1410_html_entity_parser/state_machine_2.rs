pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::str::Bytes;

struct State<'a> {
    result: Vec<u8>,
    iter: Bytes<'a>,
}

impl State<'_> {
    fn not_and(&mut self) {
        match self.iter.next() {
            None => (),
            Some(b'&') => self.and(),
            Some(c) => {
                self.result.push(c);
                self.not_and();
            }
        }
    }

    fn and(&mut self) {
        match self.iter.next() {
            None => self.result.push(b'&'),
            Some(b'&') => {
                self.result.push(b'&');
                self.and();
            }
            Some(b'a') => self.and_a(),
            Some(b'f') => self.and_f(),
            Some(b'g') => self.and_g(),
            Some(b'l') => self.and_l(),
            Some(b'q') => self.and_q(),
            Some(c) => {
                self.result.push(b'&');
                self.result.push(c);
                self.not_and();
            }
        }
    }

    fn and_a(&mut self) {
        match self.iter.next() {
            None => self.result.extend(b"&a"),
            Some(b'&') => {
                self.result.extend(b"&a");
                self.and();
            }
            Some(b'm') => self.and_a_m(),
            Some(b'p') => self.and_a_p(),
            Some(c) => {
                self.result.extend(b"&a");
                self.result.push(c);
                self.not_and();
            }
        }
    }

    fn and_a_m(&mut self) {
        match self.iter.next() {
            None => self.result.extend(b"&am"),
            Some(b'&') => {
                self.result.extend(b"&am");
                self.and();
            }
            Some(b'p') => self.and_a_m_p(),
            Some(c) => {
                self.result.extend(b"&am");
                self.result.push(c);
                self.not_and();
            }
        }
    }

    fn and_a_m_p(&mut self) {
        match self.iter.next() {
            None => self.result.extend(b"&amp"),
            Some(b'&') => {
                self.result.extend(b"&amp");
                self.and();
            }
            Some(b';') => {
                self.result.push(b'&');
                self.not_and();
            }
            Some(c) => {
                self.result.extend(b"&amp");
                self.result.push(c);
                self.not_and();
            }
        }
    }

    fn and_a_p(&mut self) {
        match self.iter.next() {
            None => self.result.extend(b"&ap"),
            Some(b'&') => {
                self.result.extend(b"&ap");
                self.and();
            }
            Some(b'o') => self.and_a_p_o(),
            Some(c) => {
                self.result.extend(b"&ap");
                self.result.push(c);
                self.not_and();
            }
        }
    }

    fn and_a_p_o(&mut self) {
        match self.iter.next() {
            None => self.result.extend(b"&apo"),
            Some(b'&') => {
                self.result.extend(b"&apo");
                self.and();
            }
            Some(b's') => self.and_a_p_o_s(),
            Some(c) => {
                self.result.extend(b"&apo");
                self.result.push(c);
                self.not_and();
            }
        }
    }

    fn and_a_p_o_s(&mut self) {
        match self.iter.next() {
            None => self.result.extend(b"&apos"),
            Some(b'&') => {
                self.result.extend(b"&apos");
                self.and();
            }
            Some(b';') => {
                self.result.push(b'\'');
                self.not_and();
            }
            Some(c) => {
                self.result.extend(b"&apos");
                self.result.push(c);
                self.not_and();
            }
        }
    }

    fn and_f(&mut self) {
        match self.iter.next() {
            None => self.result.extend(b"&f"),
            Some(b'&') => {
                self.result.extend(b"&f");
                self.and();
            }
            Some(b'r') => self.and_f_r(),
            Some(c) => {
                self.result.extend(b"&f");
                self.result.push(c);
                self.not_and();
            }
        }
    }

    fn and_f_r(&mut self) {
        match self.iter.next() {
            None => self.result.extend(b"&fr"),
            Some(b'&') => {
                self.result.extend(b"&fr");
                self.and();
            }
            Some(b'a') => self.and_f_r_a(),
            Some(c) => {
                self.result.extend(b"&fr");
                self.result.push(c);
                self.not_and();
            }
        }
    }

    fn and_f_r_a(&mut self) {
        match self.iter.next() {
            None => self.result.extend(b"&fra"),
            Some(b'&') => {
                self.result.extend(b"&fra");
                self.and();
            }
            Some(b's') => self.and_f_r_a_s(),
            Some(c) => {
                self.result.extend(b"&fra");
                self.result.push(c);
                self.not_and();
            }
        }
    }

    fn and_f_r_a_s(&mut self) {
        match self.iter.next() {
            None => self.result.extend(b"&fras"),
            Some(b'&') => {
                self.result.extend(b"&fras");
                self.and();
            }
            Some(b'l') => self.and_f_r_a_s_l(),
            Some(c) => {
                self.result.extend(b"&fras");
                self.result.push(c);
                self.not_and();
            }
        }
    }

    fn and_f_r_a_s_l(&mut self) {
        match self.iter.next() {
            None => self.result.extend(b"&frasl"),
            Some(b'&') => {
                self.result.extend(b"&frasl");
                self.and();
            }
            Some(b';') => {
                self.result.push(b'/');
                self.not_and();
            }
            Some(c) => {
                self.result.extend(b"&frasl");
                self.result.push(c);
                self.not_and();
            }
        }
    }

    fn and_g(&mut self) {
        match self.iter.next() {
            None => self.result.extend(b"&g"),
            Some(b'&') => {
                self.result.extend(b"&g");
                self.and();
            }
            Some(b't') => self.and_g_t(),
            Some(c) => {
                self.result.extend(b"&g");
                self.result.push(c);
                self.not_and();
            }
        }
    }

    fn and_g_t(&mut self) {
        match self.iter.next() {
            None => self.result.extend(b"&gt"),
            Some(b'&') => {
                self.result.extend(b"&gt");
                self.and();
            }
            Some(b';') => {
                self.result.push(b'>');
                self.not_and();
            }
            Some(c) => {
                self.result.extend(b"&gt");
                self.result.push(c);
                self.not_and();
            }
        }
    }

    fn and_l(&mut self) {
        match self.iter.next() {
            None => self.result.extend(b"&l"),
            Some(b'&') => {
                self.result.extend(b"&l");
                self.and();
            }
            Some(b't') => self.and_l_t(),
            Some(c) => {
                self.result.extend(b"&l");
                self.result.push(c);
                self.not_and();
            }
        }
    }

    fn and_l_t(&mut self) {
        match self.iter.next() {
            None => self.result.extend(b"&lt"),
            Some(b'&') => {
                self.result.extend(b"&lt");
                self.and();
            }
            Some(b';') => {
                self.result.push(b'<');
                self.not_and();
            }
            Some(c) => {
                self.result.extend(b"&lt");
                self.result.push(c);
                self.not_and();
            }
        }
    }

    fn and_q(&mut self) {
        match self.iter.next() {
            None => self.result.extend(b"&q"),
            Some(b'&') => {
                self.result.extend(b"&q");
                self.and();
            }
            Some(b'u') => self.and_q_u(),
            Some(c) => {
                self.result.extend(b"&q");
                self.result.push(c);
                self.not_and();
            }
        }
    }

    fn and_q_u(&mut self) {
        match self.iter.next() {
            None => self.result.extend(b"&qu"),
            Some(b'&') => {
                self.result.extend(b"&qu");
                self.and();
            }
            Some(b'o') => self.and_q_u_o(),
            Some(c) => {
                self.result.extend(b"&qu");
                self.result.push(c);
                self.not_and();
            }
        }
    }

    fn and_q_u_o(&mut self) {
        match self.iter.next() {
            None => self.result.extend(b"&quo"),
            Some(b'&') => {
                self.result.extend(b"&quo");
                self.and();
            }
            Some(b't') => self.and_q_u_o_t(),
            Some(c) => {
                self.result.extend(b"&quo");
                self.result.push(c);
                self.not_and();
            }
        }
    }

    fn and_q_u_o_t(&mut self) {
        match self.iter.next() {
            None => self.result.extend(b"&quot"),
            Some(b'&') => {
                self.result.extend(b"&quot");
                self.and();
            }
            Some(b';') => {
                self.result.push(b'"');
                self.not_and();
            }
            Some(c) => {
                self.result.extend(b"&quot");
                self.result.push(c);
                self.not_and();
            }
        }
    }
}

impl Solution {
    pub fn entity_parser(text: String) -> String {
        let mut state = State {
            result: Vec::new(),
            iter: text.bytes(),
        };

        state.not_and();

        String::from_utf8(state.result).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn entity_parser(text: String) -> String {
        Self::entity_parser(text)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
