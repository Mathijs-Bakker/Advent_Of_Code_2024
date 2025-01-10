mod part_one;

fn main() {
    let data = include_str!("../data/input.txt");

    let result = part_one::get_sum_middle_page_numbers(data);
    println!("{:?}", result);
}
