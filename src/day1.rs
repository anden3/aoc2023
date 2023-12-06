#[aoc(day1, part1)]
pub fn part1(input: &[u8]) -> i32 {
    input
        .split(|b| *b == b'\n')
        .map(|row| {
            let first = row.iter().find(|c| c.is_ascii_digit()).unwrap() - b'0';
            let second = row.iter().rfind(|c| c.is_ascii_digit()).unwrap() - b'0';

            first as i32 * 10 + second as i32
        })
        .sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &[u8]) -> usize {
    let mut result = 0;

    for line in input.split(|c| *c == b'\n') {
        let mut index = 0;

        let first = loop {
            match line[index..] {
                [num @ b'0'..=b'9', ..] => {
                    break (num - b'0') as usize;
                }
                [b'o', b'n', b'e', ..] => {
                    break 1;
                }
                [b't', b'w', b'o', ..] => {
                    break 2;
                }
                [b't', b'h', b'r', b'e', b'e', ..] => {
                    break 3;
                }
                [b'f', b'o', b'u', b'r', ..] => {
                    break 4;
                }
                [b'f', b'i', b'v', b'e', ..] => {
                    break 5;
                }
                [b's', b'i', b'x', ..] => {
                    break 6;
                }
                [b's', b'e', b'v', b'e', b'n', ..] => {
                    break 7;
                }
                [b'e', b'i', b'g', b'h', b't', ..] => {
                    break 8;
                }
                [b'n', b'i', b'n', b'e', ..] => {
                    break 9;
                }
                _ => {}
            }

            index += 1;
        };

        index = line.len();

        let second = loop {
            match line[..index] {
                [.., num @ b'0'..=b'9'] => {
                    break (num - b'0') as usize;
                }
                [.., b'o', b'n', b'e'] => {
                    break 1;
                }
                [.., b't', b'w', b'o'] => {
                    break 2;
                }
                [.., b't', b'h', b'r', b'e', b'e'] => {
                    break 3;
                }
                [.., b'f', b'o', b'u', b'r'] => {
                    break 4;
                }
                [.., b'f', b'i', b'v', b'e'] => {
                    break 5;
                }
                [.., b's', b'i', b'x'] => {
                    break 6;
                }
                [.., b's', b'e', b'v', b'e', b'n'] => {
                    break 7;
                }
                [.., b'e', b'i', b'g', b'h', b't'] => {
                    break 8;
                }
                [.., b'n', b'i', b'n', b'e'] => {
                    break 9;
                }
                _ => {}
            }

            index -= 1;
        };

        result += first * 10 + second;
    }

    result
}

#[cfg(test)]
mod tests {
    const TEST_1: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    const TEST_2: &str = "two1nine
eightwothree
abcone2threexyz
jtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn test_1() {
        assert_eq!(super::part1(TEST_1.as_bytes()), 142);
    }

    #[test]
    fn test_2() {
        assert_eq!(super::part2(TEST_2.as_bytes()), 281);
    }
}
