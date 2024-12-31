fn main() {
    let data = include_str!("../data/input.txt");
    let mut search = Search::new(data, "XMAS");

    let result = search.scan();
    println!("Part 1:  {result:?}");
}

struct Direction {
    north: i32,
    east: i32,
    south: i32,
    west: i32,
}

impl Direction {
    fn new() -> Self {
        Self {
            north: -1,
            east: 1,
            south: 1,
            west: -1,
        }
    }
}

#[derive(Debug)]
struct Search {
    data: Vec<Vec<char>>,
    word: Vec<char>,
}

impl Search {
    fn new(data: &str, word: &str) -> Self {
        let data: Vec<Vec<char>> = data.lines().map(|l| l.chars().collect()).collect();
        Self {
            data,
            word: word.chars().collect(),
        }
    }

    fn scan(&mut self) -> u32 {
        let data = &self.data;
        let data_len = data.len();
        let search_word = &self.word;

        let mut word_count = 0;

        let dir = Direction::new();

        (0..data_len).for_each(|row_idx| {
            let data = &self.data;

            for col_idx in 0..data_len {
                let current_char = data[row_idx][col_idx];

                let north_margin = row_idx >= search_word.len() - 1;
                let south_margin = row_idx < data_len - search_word.len() + 1;
                let west_margin = col_idx >= search_word.len() - 1;
                let east_margin = col_idx < data_len - search_word.len() + 1;

                if current_char == search_word[0] {
                    word_count +=
                        scan_north(north_margin, data, search_word, row_idx, col_idx, dir.north)
                            + scan_north_east(
                                north_margin,
                                east_margin,
                                data,
                                search_word,
                                row_idx,
                                col_idx,
                                dir.north,
                                dir.east,
                            )
                            + scan_east(east_margin, data, search_word, row_idx, col_idx, dir.east)
                            + scan_south_east(
                                south_margin,
                                east_margin,
                                data,
                                search_word,
                                row_idx,
                                col_idx,
                                dir.south,
                                dir.east,
                            )
                            + scan_south(
                                south_margin,
                                data,
                                search_word,
                                row_idx,
                                col_idx,
                                dir.south,
                            )
                            + scan_south_west(
                                south_margin,
                                west_margin,
                                data,
                                search_word,
                                row_idx,
                                col_idx,
                                dir.south,
                                dir.west,
                            )
                            + scan_west(west_margin, data, search_word, row_idx, col_idx, dir.west)
                            + scan_north_west(
                                north_margin,
                                west_margin,
                                data,
                                search_word,
                                row_idx,
                                col_idx,
                                dir.north,
                                dir.west,
                            );
                }
            }
        });

        word_count
    }
}

fn scan_north_west(
    north_margin: bool,
    west_margin: bool,
    data: &[Vec<char>],
    search_word: &[char],
    row_idx: usize,
    col_idx: usize,
    north: i32,
    west: i32,
) -> u32 {
    if north_margin && west_margin {
        check_direction(data, search_word, row_idx, col_idx, north, west)
    } else {
        0
    }
}

fn scan_west(
    west_margin: bool,
    data: &[Vec<char>],
    search_word: &[char],
    row_idx: usize,
    col_idx: usize,
    west: i32,
) -> u32 {
    if west_margin {
        check_direction(data, search_word, row_idx, col_idx, 0, west)
    } else {
        0
    }
}

fn scan_south_west(
    south_margin: bool,
    west_margin: bool,
    data: &[Vec<char>],
    search_word: &[char],
    row_idx: usize,
    col_idx: usize,
    south: i32,
    west: i32,
) -> u32 {
    if south_margin && west_margin {
        check_direction(data, search_word, row_idx, col_idx, south, west)
    } else {
        0
    }
}

fn scan_south(
    south_margin: bool,
    data: &[Vec<char>],
    search_word: &[char],
    row_idx: usize,
    col_idx: usize,
    south: i32,
) -> u32 {
    match south_margin {
        true => check_direction(data, search_word, row_idx, col_idx, south, 0),
        false => 0,
    }
}

fn scan_south_east(
    south_margin: bool,
    east_margin: bool,
    data: &[Vec<char>],
    search_word: &[char],
    row_idx: usize,
    col_idx: usize,
    south: i32,
    east: i32,
) -> u32 {
    match south_margin && east_margin {
        true => check_direction(data, search_word, row_idx, col_idx, south, east),
        false => 0,
    }
}

fn scan_east(
    east_margin: bool,
    data: &[Vec<char>],
    search_word: &[char],
    row_idx: usize,
    col_idx: usize,
    east: i32,
) -> u32 {
    match east_margin {
        true => check_direction(data, search_word, row_idx, col_idx, 0, east),
        false => 0,
    }
}

fn scan_north_east(
    north_margin: bool,
    east_margin: bool,
    data: &[Vec<char>],
    search_word: &[char],
    row_idx: usize,
    col_idx: usize,
    north: i32,
    east: i32,
) -> u32 {
    match north_margin && east_margin {
        true => check_direction(data, search_word, row_idx, col_idx, north, east),
        false => 0,
    }
}

fn scan_north(
    north_margin: bool,
    data: &[Vec<char>],
    search_word: &[char],
    row_idx: usize,
    col_idx: usize,
    north: i32,
) -> u32 {
    match north_margin {
        true => check_direction(data, search_word, row_idx, col_idx, north, 0),
        false => 0,
    }
}

fn check_direction(
    data: &[Vec<char>],
    search_word: &[char],
    row_idx: usize,
    col_idx: usize,
    row_direction: i32,
    col_direction: i32,
) -> u32 {
    let mut current_col = col_idx as i32 + col_direction;
    let mut current_row = row_idx as i32 + row_direction;
    let mut char_matches = 0;

    for c in search_word.iter().skip(1) {
        match *c == data[current_row as usize][current_col as usize] {
            true => {
                char_matches += 1;
                current_col += col_direction;
                current_row += row_direction;
            }
            _ => break,
        }
    }

    match char_matches == search_word.len() - 1 {
        true => 1,
        _ => 0,
    }
}
