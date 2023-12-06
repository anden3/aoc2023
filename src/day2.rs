#[aoc(day2, part1)]
pub fn part1(input: &str) -> usize {
    let mut result = 0;

    'games: for (i, line) in input.split('\n').enumerate() {
        let i = i + 1;
        let prefix_width = match i {
            1..=9 => 8,
            10..=99 => 9,
            100..=1000 => 10,
            _ => unreachable!(),
        };

        let line = &line[prefix_width..];

        for subset in line.split(';') {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            for cube_selection in subset.split(',') {
                let (count, color) = cube_selection.trim().split_once(' ').unwrap();
                let count = count.parse::<usize>().unwrap();

                let (color, threshold) = match color {
                    "red" => (&mut red, 12),
                    "green" => (&mut green, 13),
                    "blue" => (&mut blue, 14),
                    _ => unreachable!(),
                };

                *color += count;

                if *color > threshold {
                    continue 'games;
                }
            }
        }

        result += i;
    }

    result
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> usize {
    let mut result = 0;

    for (i, line) in input.split('\n').enumerate() {
        let i = i + 1;
        let prefix_width = match i {
            1..=9 => 8,
            10..=99 => 9,
            100..=1000 => 10,
            _ => unreachable!(),
        };

        let line = &line[prefix_width..];

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for subset in line.split(';') {
            for cube_selection in subset.split(',') {
                let (count, color) = cube_selection.trim().split_once(' ').unwrap();
                let count = count.parse::<usize>().unwrap();

                let color = match color {
                    "red" => &mut red,
                    "green" => &mut green,
                    "blue" => &mut blue,
                    _ => unreachable!(),
                };

                *color = usize::max(*color, count);
            }
        }

        result += red * green * blue;
    }

    result
}

#[cfg(test)]
mod tests {
    const TEST: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_1() {
        assert_eq!(super::part1(TEST), 8);
    }

    #[test]
    fn test_2() {
        assert_eq!(super::part2(TEST), 2286);
    }
}
