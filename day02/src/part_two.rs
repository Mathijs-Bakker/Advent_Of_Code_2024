pub(crate) fn count_with_toleration(reports: &str) -> usize {
    let n_safe_reports = reports
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|c| c.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|numbers| {
            'outer: for i in 0..numbers.len() {
                let mut numbers = numbers.clone();
                numbers.remove(i);

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
                        continue 'outer;
                    }

                    prev_num = cur_num;
                }

                return true;
            }
            false
        })
        .count();
    n_safe_reports
}
