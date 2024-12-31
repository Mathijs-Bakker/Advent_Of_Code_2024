mod part_one;

fn main() {
    let data = include_str!("../data/input.txt");
    let mut search = part_one::Search::new(data, "XMAS");

    let result = search.scan();
    println!("Part 1:  {result:?}");
}
