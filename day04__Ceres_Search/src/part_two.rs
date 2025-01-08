#[derive(Debug)]
pub(crate) struct Scanner {
    pub(crate) data: Vec<char>,
    pub(crate) len: usize,
    pub(crate) index: usize,
}

impl Scanner {
    pub(crate) fn new(data: &str, len: usize) -> Self {
        let data = data.chars().collect();
        Self {
            data,
            len,
            index: 0,
        }
    }
}

impl Iterator for Scanner {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let chars = &self.data;

        if chars.len() < self.len * 2 + 3 {
            self.data.clear();
            return None;
        }

        self.index += 1;

        if (self.index + 1) % self.len == 0 {
            self.data.remove(0);
            self.data.remove(0);
            self.index += 1;
        } else {
            let top_left = chars[0];
            let top_right = chars[2];
            let center = chars[self.len + 1];
            let bottom_left = chars[self.len * 2];
            let bottom_right = chars[self.len * 2 + 2];

            self.data.remove(0);

            if center != 'A' {
                return Some(0);
            }

            let mut hits = 0;

            if top_left == 'M' && bottom_right == 'S' {
                hits += 1;
            }
            if top_left == 'S' && bottom_right == 'M' {
                hits += 1;
            }
            if top_right == 'M' && bottom_left == 'S' {
                hits += 1;
            }
            if top_right == 'S' && bottom_left == 'M' {
                hits += 1;
            }

            if hits == 2 {
                return Some(1);
            }
        }
        Some(0)
    }
}
