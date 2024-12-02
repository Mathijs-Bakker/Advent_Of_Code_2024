pub(crate) fn calc_total_distance(location_id_data: &str) -> u32 {
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    location_id_data
        .lines()
        .map(|line| line.split_once("   ").unwrap())
        .for_each(|(r, l)| {
            left_list.push(l.parse::<u32>().unwrap());
            right_list.push(r.parse::<u32>().unwrap());
        });

    left_list.sort();
    right_list.sort();

    let total_distance = left_list
        .into_iter()
        .zip(right_list)
        .fold(0, |acc, (l, r)| acc + l.abs_diff(r));

    total_distance
}
