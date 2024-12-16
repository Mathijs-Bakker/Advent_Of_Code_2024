mod part_one;
mod part_two;

fn main() {
    let data = include_str!("../data/input.txt");

    let result = part_one::calc_sum_of_multiplications(data);
    println!("Part one: {:?}", result);

    let result = part_two::Data::new(data).next().unwrap();
    println!("Part two: {:?}", result);
}
