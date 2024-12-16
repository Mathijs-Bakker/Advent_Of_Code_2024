pub(crate) fn calc_sum_of_multiplications(data: &str) -> u32 {
    data.split(")")
        .map(|s| s.split("mul(").collect::<Vec<&str>>())
        .map(|e| e.last().unwrap().split(",").collect::<Vec<_>>())
        .map(|e| {
            if e.len() == 2 {
                let a = e[0].parse::<u32>().unwrap_or(0);
                let b = e[1].parse::<u32>().unwrap_or(0);
                a * b
            } else {
                0
            }
        })
        .sum::<u32>()
}
