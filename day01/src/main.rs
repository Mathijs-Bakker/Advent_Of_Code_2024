mod part_one;
mod part_two;

fn main() {
    let data = include_str!("../src/data/input.txt");

    let total_distance = part_one::calc_total_distance(data);
    println!("Part 1: {total_distance}");

    let similarity_score = part_two::calc_similarity_score(data);
    println!("Part 2: {similarity_score}");
}
