use crate::util;

#[aoc(day6, part1)]
pub fn part1(input: &str) -> i64 {
    let (time_row, distance_row) = input.split_once('\n').unwrap();

    get_iter(time_row)
        .zip(get_iter(distance_row))
        .map(|(t, d)| get_roots(t, d + 0.001))
        .map(|[from, to]| to - from + 1.0)
        .product::<f64>() as i64
}

fn get_iter(line: &str) -> impl Iterator<Item = f64> + '_ {
    let (_label, values) = line.split_once(':').unwrap();
    values
        .trim_start()
        .split_ascii_whitespace()
        .map(|val| val.parse::<f64>().unwrap())
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> i64 {
    let (time_row, distance_row) = input.split_once('\n').unwrap();
    let (time, distance) = (get_joined_number(time_row), get_joined_number(distance_row));

    let [from, to] = get_roots(time, distance + 0.001);
    (to - from + 1.0) as i64
}

fn get_joined_number(line: &str) -> f64 {
    let (_label, values) = line.split_once(':').unwrap();

    let bytes = values
        .trim_start()
        .split_ascii_whitespace()
        .flat_map(str::bytes);

    util::parse_number_from_iter(bytes) as f64
}

/// h^2 - Th + S = 0
/// (T +- sqrt(T^2 - 4S)) / 2
fn get_roots(time: f64, distance: f64) -> [f64; 2] {
    let sqrt = (time.powi(2) - distance * 4.0).sqrt();

    let from = ((time - sqrt) / 2.0).ceil();
    let to = ((time + sqrt) / 2.0).floor();

    [from, to]
}

#[cfg(test)]
mod tests {
    const TEST: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_1() {
        assert_eq!(super::part1(TEST), 288);
    }

    #[test]
    fn test_2() {
        assert_eq!(super::part2(TEST), 71503);
    }
}
