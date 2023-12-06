#[cfg(test)]
const WINNING_COUNT: usize = 5;
#[cfg(not(test))]
const WINNING_COUNT: usize = 10;

#[cfg(test)]
const NUMBER_COUNT: usize = 8;
#[cfg(not(test))]
const NUMBER_COUNT: usize = 25;

#[cfg(test)]
const PADDING: usize = 7;
#[cfg(not(test))]
const PADDING: usize = 9;

#[aoc(day4, part1)]
pub fn part1(input: &[u8]) -> usize {
    const HIGHEST_BIT: usize =
        0b_1000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;

    let mut result = 0;

    for mut line in input.split(|b| *b == b'\n') {
        line = &line[PADDING..];

        let mut winning_numbers = [false; 100];
        let mut points: usize = HIGHEST_BIT;

        for _ in 0..WINNING_COUNT {
            let (chunk, rest) = line.split_at(3);
            let num = get_num(chunk);
            winning_numbers[num as usize] = true;

            line = rest;
        }

        line = &line[2..];

        for _ in 0..NUMBER_COUNT {
            let (chunk, rest) = line.split_at(3);
            let num = get_num(chunk);

            points = points.rotate_left(u32::from(winning_numbers[num as usize]));

            line = rest;
        }

        result += points & !HIGHEST_BIT;
    }

    result
}

#[aoc(day4, part2)]
pub fn part2(input: &[u8]) -> usize {
    let mut cards = 0;
    let mut card_multipliers = [1usize; 200];

    for (i, mut line) in input.split(|b| *b == b'\n').enumerate() {
        line = &line[PADDING..];
        let multiplier = card_multipliers[i];
        cards += multiplier;

        let mut winning_numbers = [false; 100];
        let mut winning_count = 0;

        for _ in 0..WINNING_COUNT {
            let (chunk, rest) = line.split_at(3);
            let num = get_num(chunk);
            winning_numbers[num as usize] = true;

            line = rest;
        }

        line = &line[2..];

        for _ in 0..NUMBER_COUNT {
            let (chunk, rest) = line.split_at(3);
            let num = get_num(chunk);

            winning_count += usize::from(winning_numbers[num as usize]);
            line = rest;
        }

        for next in 1..=winning_count {
            card_multipliers[i + next] += multiplier;
        }
    }

    cards
}

fn get_num(slice: &[u8]) -> u8 {
    if slice[1] == b' ' {
        slice[2] - b'0'
    } else {
        (slice[1] - b'0') * 10 + (slice[2] - b'0')
    }
}

#[cfg(test)]
mod tests {
    const TEST: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_1() {
        assert_eq!(super::part1(TEST.as_bytes()), 13);
    }

    #[test]
    fn test_2() {
        assert_eq!(super::part2(TEST.as_bytes()), 30);
    }
}
