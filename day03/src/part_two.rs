#[derive(Debug)]
pub(crate) struct Data<'a> {
    pub(crate) remaining_data: &'a str,
    pub(crate) multiply_enabled: bool,
}

impl<'a> Data<'a> {
    pub(crate) fn new(input_data: &'a str) -> Self {
        Self {
            remaining_data: input_data,
            multiply_enabled: true,
        }
    }
}

impl Iterator for Data<'_> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let mut multiplied_nums: Vec<u32> = Vec::new();

        loop {
            if self.remaining_data.is_empty() {
                break;
            }

            let mut chars = self.remaining_data.chars();
            let current_char = chars.next();
            let mut remaining = self.remaining_data;
            self.remaining_data = chars.as_str();
            let enabled = self.multiply_enabled;

            match current_char.unwrap() {
                'm' => {
                    let mul = "mul(";

                    if (&remaining[..mul.len()] == "mul(") && enabled {
                        remaining = &remaining[mul.len()..];
                        let closing_paren_idx = remaining.find(")").unwrap();

                        let numbers = &remaining[..closing_paren_idx]
                            .split(",")
                            .collect::<Vec<&str>>();

                        if numbers.len() != 2
                            || !numbers[0].chars().all(|c| c.is_numeric())
                            || !numbers[1].chars().all(|c| c.is_numeric())
                        {
                            continue;
                        }

                        self.remaining_data = &remaining[closing_paren_idx + 1..];

                        multiplied_nums.push(
                            numbers[0].parse::<u32>().unwrap_or(0)
                                * numbers[1].parse::<u32>().unwrap_or(0),
                        );
                    }
                }
                'd' => {
                    let enable = "do()";
                    let disable = "don't()";

                    if &remaining[..enable.len()] == enable {
                        self.multiply_enabled = true;
                        self.remaining_data = &remaining[enable.len()..];
                    }

                    if &remaining[..disable.len()] == disable {
                        self.multiply_enabled = false;
                        self.remaining_data = &remaining[disable.len()..];
                    }

                    continue;
                }
                _ => {
                    continue;
                }
            };
        }

        Some(multiplied_nums.iter().sum::<u32>())
    }
}
