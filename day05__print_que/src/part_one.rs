pub(crate) fn get_sum_middle_page_numbers(data: &str) -> u32 {
    let mut ordering_rules_data: Vec<(u32, u32)> = Vec::new();
    let mut page_update_data: Vec<Vec<u32>> = Vec::new();

    let _ = data
        .lines()
        .map(|l| {
            if l.contains("|") {
                let split = l.split("|").collect::<Vec<&str>>();
                let left = split[0].parse::<u32>().unwrap();
                let right = split[1].parse::<u32>().unwrap();

                ordering_rules_data.push((left, right));
            } else if !l.is_empty() {
                let mut vec = Vec::new();

                for c in l.split(",") {
                    let n = c.parse::<u32>().unwrap();
                    vec.push(n);
                }

                page_update_data.push(vec);
            }
        })
        .collect::<Vec<_>>();

    let mut valid_ordered_updates = Vec::new();

    for pages in page_update_data {
        let mut all_pages_validated = false;
        let n_pages = pages.len() - 1;
        let mut valid_pages = Vec::new();

        for idx in 0..n_pages {
            let pages = (pages[idx], pages[idx + 1]);

            for rule in &ordering_rules_data {
                if pages == *rule {
                    valid_pages.push(pages.0);
                    break;
                }
            }
            if idx == n_pages - 1 {
                if valid_pages.len() == n_pages {
                    valid_pages.push(pages.1);
                    all_pages_validated = true;
                }
                break;
            }
        }

        if all_pages_validated {
            valid_ordered_updates.push(valid_pages);
        }
    }

    let mut sum = 0;

    for pages in valid_ordered_updates {
        let middle = pages.len() / 2;
        sum += pages[middle];
    }

    sum
}
