pub fn diagonstic1(binary_strs: &Vec<&str>) -> i32 {
    if binary_strs.len() == 0 {
        return 0;
    }

    let count_threshold: i32 = (binary_strs.len() / 2).try_into().unwrap();
    let mut ones_counter = vec![0; binary_strs[0].len()];
    for binary_str in binary_strs {
        let binary_str: Vec<char> = binary_str.chars().collect();
        for i in 0..binary_str.len() {
            ones_counter[i] += if binary_str[i] == '1' { 1 } else { 0 };
        }
    }

    let mut gamma_rate: String = "".to_string();
    let mut epsilon_rate: String = "".to_string();

    for count in &ones_counter {
        gamma_rate += if count < &count_threshold { "0" } else { "1" };
        epsilon_rate += if count < &count_threshold { "1" } else { "0" };
    }

    let gamma_dec = isize::from_str_radix(&gamma_rate, 2).unwrap();
    let epsilon_dec = isize::from_str_radix(&epsilon_rate, 2).unwrap();
    (gamma_dec * epsilon_dec).try_into().unwrap()
}

fn calculate_o2_generator_rating(binary_strs: &Vec<&str>, i: usize) -> i32 {
    if binary_strs.len() == 1 {
        return isize::from_str_radix(&binary_strs[0], 2)
            .unwrap()
            .try_into()
            .unwrap();
    }

    let mut prefix_one_strs: Vec<&str> = Vec::new();
    let mut prefix_zero_strs: Vec<&str> = Vec::new();

    for binary_str in binary_strs {
        let binary_vec: Vec<char> = binary_str.chars().collect();
        if binary_vec[i] == '1' {
            prefix_one_strs.push(binary_str);
        } else {
            prefix_zero_strs.push(binary_str);
        }
    }

    if prefix_one_strs.len() < prefix_zero_strs.len() {
        return calculate_o2_generator_rating(&prefix_zero_strs, i + 1);
    } else {
        return calculate_o2_generator_rating(&prefix_one_strs, i + 1);
    }
}

fn calculate_co2_scrubber_rating(binary_strs: &Vec<&str>, i: usize) -> i32 {
    if binary_strs.len() == 1 {
        return isize::from_str_radix(&binary_strs[0], 2)
            .unwrap()
            .try_into()
            .unwrap();
    }

    let mut prefix_one_strs: Vec<&str> = Vec::new();
    let mut prefix_zero_strs: Vec<&str> = Vec::new();

    for binary_str in binary_strs {
        let binary_vec: Vec<char> = binary_str.chars().collect();
        if binary_vec[i] == '1' {
            prefix_one_strs.push(binary_str);
        } else {
            prefix_zero_strs.push(binary_str);
        }
    }

    if prefix_one_strs.len() >= prefix_zero_strs.len() {
        return calculate_co2_scrubber_rating(&prefix_zero_strs, i + 1);
    } else {
        return calculate_co2_scrubber_rating(&prefix_one_strs, i + 1);
    }
}

pub fn diagonstic2(binary_strs: &Vec<&str>) -> i32 {
    if binary_strs.len() == 0 {
        return 0;
    }
    calculate_o2_generator_rating(&binary_strs, 0) * calculate_co2_scrubber_rating(&binary_strs, 0)
}
