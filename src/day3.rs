use aoc21::get_input_lines;

fn count_bits(numbers: &[u32], bit: usize) -> (usize, usize) {
    let mask = 1u32 << bit;
    let zeroes = numbers.iter().filter(|&&n| n & mask == 0).count();
    let ones = numbers.len() - zeroes;
    (zeroes, ones)
}

fn main() {
    let lines = get_input_lines();
    let bit_count = lines.first().unwrap().len();
    let numbers = lines.into_iter()
        .map(|l| u32::from_str_radix(&l, 2).unwrap())
        .collect::<Vec<u32>>();

    let part1 = {
        let mut gamma = 0u32;
        let mut epsilon = 0u32;
        for bit in (0..bit_count).rev() {
            gamma <<= 1;
            epsilon <<= 1;
            let (zeroes, ones) = count_bits(&numbers, bit);
            if ones > zeroes {
                gamma |= 1;
            } else {
                epsilon |= 1;
            }
        }

        gamma * epsilon
    };

    let part2 = {
        let oxygen = {
            let mut numbers = numbers.clone();

            for bit in (0..bit_count).rev() {
                let mask = 1u32 << bit;
                let (zeroes, ones) = count_bits(&numbers, bit);
                if ones >= zeroes {
                    numbers.retain(|n| n & mask > 0);
                } else {
                    numbers.retain(|n| n & mask == 0);
                }

                if numbers.len() == 1 {
                    break;
                }
            }
            numbers.first().unwrap().clone()
        };
        let co2 = {
            let mut numbers = numbers;

            for bit in (0..bit_count).rev() {
                let mask = 1u32 << bit;
                let (zeroes, ones) = count_bits(&numbers, bit);
                if zeroes <= ones {
                    numbers.retain(|n| n & mask == 0);
                } else {
                    numbers.retain(|n| n & mask > 0);
                }

                if numbers.len() == 1 {
                    break;
                }
            }
            numbers.first().unwrap().clone()
        };
        oxygen * co2
    };
    println!("Part 1: {}\nPart 2: {}", part1, part2);
}