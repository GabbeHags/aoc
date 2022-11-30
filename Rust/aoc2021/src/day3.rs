fn get_common(arr: &[String], pos: usize) -> &str {
    let mut gamma_count = 0;
    let mut epsilon_count = 0;

    for bits in arr {
        if bits.as_bytes()[pos] == b'1' {
            gamma_count += 1;
        } else {
            epsilon_count += 1;
        }
    }

    if gamma_count >= epsilon_count {
        "1"
    } else {
        "0"
    }
}

fn flip(bit: &str) -> &str {
    if bit == "1" {
        return "0";
    }
    "1"
}

fn str_binary_to_int(bits: String) -> i32 {
    i32::from_str_radix(bits.as_str(), 2).unwrap()
}

fn get_rating(arr: &[String], f: bool) -> String {
    let bits_len = arr.first().unwrap().len();
    let mut result: Vec<String> = arr.to_vec();
    let mut common: &str;
    for i in 0..bits_len {
        let cloned_result = &result.clone(); // &result.clone();
        common = get_common(cloned_result, i);
        common = if f { flip(common) } else { common };
        result.retain(|x| x.as_bytes()[i] == common.as_bytes()[0]);
        if result.len() == 1 {
            break;
        }
    }
    result.first().unwrap().to_string()
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines()
    .map(|x| x.to_string())
    .collect::<Vec<_>>()
}

#[aoc(day3, part1)]
fn part_1(input: &[String]) -> i32 {
    let mut gamma = String::new();
    let mut epsilon = String::new();
    let mut common_bit: &str;
    for pos in 0..input.first().unwrap().len() {
        common_bit = get_common(&input, pos);
        gamma += common_bit;
        epsilon += flip(common_bit);
    }
    str_binary_to_int(gamma) * str_binary_to_int(epsilon)
}

#[aoc(day3, part2)]
pub fn part_2(input: &[String]) -> i32 {
    let oxygen = get_rating(input, false);
    let co2 = get_rating(input, true);
    str_binary_to_int(oxygen) * str_binary_to_int(co2)
}
