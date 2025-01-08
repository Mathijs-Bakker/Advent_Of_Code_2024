mod part_one;
mod part_two;

fn main() {
    let data = include_str!("../data/input.txt");

    // part_one
    let mut search = part_one::Search::new(data, "XMAS");

    let result = search.scan();
    println!("Part 1:  {result:?}");

    // part_two
    let len = data.find("\n").unwrap();
    let data = data.replace("\n", "");
    let data = data.as_str();
    let result = part_two::Scanner::new(data, len);

    let sum = result.sum::<u32>();
    println!("Part 2:  {:?}", sum);
}
