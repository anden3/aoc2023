use std::ops::Range;

use rangemap::RangeSet;

#[aoc(day5, part1)]
pub fn part1(input: &str) -> i64 {
    let mut seeds = Vec::<i64>::with_capacity(32);

    for section in input.split("\n\n") {
        let (label, values) = section.split_once(':').unwrap();

        if label == "seeds" {
            seeds = values
                .trim()
                .split_ascii_whitespace()
                .map(|v| v.parse().unwrap())
                .collect();
            continue;
        }

        let mut mapped_seeds = [false; 64];

        for line in values.trim().lines() {
            let mut line_values = line.split_ascii_whitespace().map(|v| v.parse().unwrap());
            let [dest, source, length]: [i64; 3] =
                std::array::from_fn(|_| line_values.next().unwrap());

            let diff = dest - source;
            let source_range = source..(source + length + 1);

            for (i, seed) in seeds.iter_mut().enumerate() {
                if !mapped_seeds[i] && source_range.contains(seed) {
                    *seed += diff;
                    mapped_seeds[i] = true;
                }
            }
        }
    }

    seeds.into_iter().min().unwrap()
}

trait RangeExt {
    fn add(&self, offset: i64) -> Self;
}

impl RangeExt for Range<i64> {
    fn add(&self, offset: i64) -> Self {
        (self.start + offset)..(self.end + offset)
    }
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> i64 {
    let mut seeds = RangeSet::<i64>::new();

    for section in input.split("\n\n") {
        let (label, values) = section.split_once(':').unwrap();

        if label == "seeds" {
            let seed_vec = values
                .trim()
                .split_ascii_whitespace()
                .map(|v| v.parse::<i64>().unwrap())
                .collect::<Vec<_>>();

            for pair in seed_vec.chunks_exact(2) {
                let (start, length) = (pair[0], pair[1]);
                seeds.insert(start..(start + length));
            }
            continue;
        }

        let mut next_seeds = RangeSet::<i64>::new();

        for line in values.trim().lines() {
            let mut line_values = line.split_ascii_whitespace().map(|v| v.parse().unwrap());
            let [dest, source, length]: [i64; 3] =
                std::array::from_fn(|_| line_values.next().unwrap());

            let diff = dest - source;
            let source_range = source..(source + length);
            let mut to_remove = Vec::new();

            for overlapping in seeds.overlapping(&source_range) {
                let range = match (
                    source_range.contains(&overlapping.start),
                    source_range.contains(&overlapping.end),
                ) {
                    (true, true) => {
                        overlapping.clone()
                    }
                    (true, false) => {
                        overlapping.start..source_range.end
                    }
                    (false, true) => {
                        source_range.start..overlapping.end
                    }
                    (false, false) => {
                        source_range.clone()
                    }
                };
                
                next_seeds.insert(range.add(diff));
                to_remove.push(range);
            }

            for range in to_remove {
                seeds.remove(range);
            }
        }

        next_seeds.extend(seeds);
        seeds = next_seeds;
    }

    seeds.into_iter().next().unwrap().start
}

#[cfg(test)]
mod tests {
    const TEST: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_1() {
        assert_eq!(super::part1(TEST), 35);
    }

    #[test]
    fn test_2() {
        assert_eq!(super::part2(TEST), 46);
    }
}
