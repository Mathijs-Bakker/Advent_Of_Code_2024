pub(crate) struct SearchData<'a> {
    pub(crate) input: &'a str,
    pub(crate) consume: &'a str,
    pub(crate) word: &'a str,
}

impl<'a> SearchData<'a> {
    pub(crate) fn new(input: &'a str, search_word: &'a str) -> Self {
        Self {
            input,
            consume: input,
            word: search_word,
        }
    }
}

pub(crate) fn check_hor(word: &str, current_pos: usize, input: &str) -> bool {
    let margin = word.len();

    &input[current_pos..current_pos + word.len()] == word
}

pub(crate) fn check_hor_rev(word_rev: &str, current_pos: usize, input: &str) -> bool {
    let margin = word_rev.len();

    &input[current_pos..current_pos + word_rev.len()] == word_rev
}

pub(crate) fn check_vert_downwards(word: &str, current_pos: usize, input: &str) -> bool {
    let mut temp: String = String::new();

    let mut col = 0;
    for _ in 0..word.len() {
        temp.push(input.chars().nth(current_pos + col).unwrap());
        col += 10;
        println!("temp {:?}", temp);
    }

    temp.as_str() == word
}

pub(crate) fn check_vert_upwards(word_rev: &str, current_pos: usize, input: &str) -> bool {
    let mut temp: String = String::new();

    let mut col = 0;
    for _ in 0..word_rev.len() {
        temp.push(input.chars().nth(current_pos + col).unwrap_or(' '));
        col -= 10;
        println!("temp {:?}", temp);
    }

    temp.as_str() == word_rev
}

pub(crate) fn check_diagonal(word: &str, current_pos: usize, input: &str) -> bool {
    let mut temp: String = String::new();

    let mut col = 0;
    for _ in 0..word.len() {
        temp.push(input.chars().nth(current_pos + col).unwrap());
        col += 11;
        println!("temp {:?}", temp);
    }

    temp.as_str() == word
}

impl Iterator for SearchData<'_> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let search_word = self.word;
        let search_word_rev = search_word.chars().rev().collect::<String>();
        let search_word_rev = search_word_rev.as_str();

        loop {
            let current_pos = self.input.len() - self.consume.len();

            let mut chars = self.consume.chars();
            let current_char = chars.next();
            self.consume = chars.as_str();

            println!("current char {:?} pos {:?}", current_char, current_pos);

            if current_char? != 'X' {
                continue;
            }

            let check_hor = check_hor(search_word, current_pos, self.input);
            println!("{:?}", check_hor);

            let check_hor_rev = check_hor_rev(search_word_rev, current_pos, self.input);
            println!("{:?}", check_hor_rev);

            let check_vert_down = check_vert_downwards(search_word, current_pos, self.input);
            let check_vert_up = check_vert_upwards(search_word_rev, current_pos, self.input);
            println!("check_vert down {:?}", check_vert_down);
            println!("check_vert up {:?}", check_vert_up);

            let check_diagonal = check_diagonal(search_word_rev, current_pos, self.input);
            println!("check_diagonal {:?}", check_diagonal);
        }

        Some(1)
    }
}
