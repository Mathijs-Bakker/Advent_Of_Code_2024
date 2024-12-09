pub(crate) fn count_safe_reports(reports: &str) -> usize {
    reports
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|char| char.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .filter(|numbers| {
            let increasing_nums = numbers[0] < numbers[1];
            let mut prev_num = numbers[0];

            for &cur_num in numbers.iter().skip(1) {
                let not_all_increasing = increasing_nums && cur_num < prev_num;
                let not_all_decreasing = !increasing_nums && cur_num > prev_num;
                let nums_exeeding_max_offset = prev_num.abs_diff(cur_num) > 3;
                let nums_are_the_same = prev_num.abs_diff(cur_num) == 0;

                if not_all_increasing
                    || not_all_decreasing
                    || nums_exeeding_max_offset
                    || nums_are_the_same
                {
                    return false;
                }

                prev_num = cur_num;
            }

            true
        })
        .count()
}
