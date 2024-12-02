use std::collections::HashMap;

pub(crate) fn calc_similarity_score(location_id_data: &str) -> u32 {
    let mut left_list: Vec<u32> = Vec::new();
    let mut right_list: HashMap<u32, u32> = HashMap::new();

    location_id_data
        .lines()
        .map(|line| line.split_once("   ").unwrap())
        .for_each(|(l, r)| {
            left_list.push(l.parse().unwrap());
            right_list
                .entry(r.parse().unwrap())
                .and_modify(|counter: &mut u32| *counter += 1)
                .or_insert(1);
        });

    let similarity_score = left_list.iter().fold(0u32, |acc, element| {
        acc + right_list.get(element).copied().unwrap_or_default() * element
    });

    similarity_score
}
