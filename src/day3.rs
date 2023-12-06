#![allow(dead_code)]

use std::marker::PhantomData;

#[cfg(test)]
const WIDTH: usize = 10;
#[cfg(not(test))]
const WIDTH: usize = 140;

struct PreviousLine;
struct CurrentLine;

#[derive(Debug)]
struct LineState<T> {
    symbols: [bool; WIDTH],
    numbers: [usize; WIDTH],
    number_indices: [Option<usize>; WIDTH],
    _marker: PhantomData<T>,
}

impl LineState<CurrentLine> {
    pub fn new() -> Self {
        Self {
            symbols: [false; WIDTH],
            numbers: [0; WIDTH],
            number_indices: [None; WIDTH],
            _marker: PhantomData,
        }
    }

    pub fn register_symbol(&mut self, index: usize) {
        self.symbols[index] = true;
    }

    pub fn register_number(&mut self, start: usize, end: usize, value: usize) {
        self.numbers[start] = value;

        for i in start..=end {
            self.number_indices[i] = Some(start);
        }
    }

    pub fn finish(self) -> LineState<PreviousLine> {
        LineState::<PreviousLine> {
            symbols: self.symbols,
            numbers: self.numbers,
            number_indices: self.number_indices,
            _marker: PhantomData,
        }
    }
}

impl LineState<PreviousLine> {
    pub fn has_symbol(&self, index: usize) -> bool {
        self.symbols[index]
    }

    pub fn get_number(&mut self, index: usize) -> Option<usize> {
        let range = index.saturating_sub(1)..=usize::min(index + 1, WIDTH - 1);

        let mut sum = 0;

        for i in range {
            let Some(idx) = self.number_indices[i].take() else { continue };
            sum += self.numbers[idx];
            self.numbers[idx] = 0;
        }

        if sum > 0 {
            Some(sum)
        } else {
            None
        }
    }

    pub fn reset(self) -> LineState<CurrentLine> {
        LineState::<CurrentLine>::new()
    }
}

#[derive(Debug)]
enum State {
    Dot,
    Number {
        start: usize,
        end: usize,
        value: usize,
        is_eligible: bool,
    },
    Symbol,
}

#[aoc(day3, part1)]
pub fn part1(input: &[u8]) -> usize {
    let mut result = 0;

    let mut current_line = LineState::<CurrentLine>::new();
    let mut previous_line = LineState::<CurrentLine>::new().finish();

    for chunk in input.split(|b| *b == b'\n') {
        let mut state = State::Dot;

        for (index, byte) in chunk.iter().chain(std::iter::once(&b'.')).enumerate() {
            let next_state = match *byte {
                b'.' => State::Dot,
                b'0'..=b'9' => State::Number {
                    start: index,
                    end: index,
                    value: (*byte - b'0') as usize,
                    is_eligible: false,
                },
                _ => State::Symbol,
            };

            state = match (state, next_state) {
                (State::Dot, State::Number { value, start, .. }) => State::Number {
                    value,
                    start,
                    end: start,
                    is_eligible: previous_line.has_symbol(index)
                        || previous_line.has_symbol(index.saturating_sub(1)),
                },
                (State::Symbol, State::Number { value, start, .. }) => State::Number {
                    value,
                    start,
                    end: start,
                    is_eligible: true,
                },
                (
                    State::Number {
                        value: value_a,
                        start,
                        is_eligible,
                        ..
                    },
                    State::Number {
                        value: value_b,
                        end,
                        ..
                    },
                ) => State::Number {
                    value: value_a * 10 + value_b,
                    start,
                    end,
                    is_eligible: is_eligible || previous_line.has_symbol(index),
                },
                (
                    State::Number {
                        start,
                        end,
                        value,
                        is_eligible,
                    },
                    State::Dot,
                ) => {
                    if is_eligible || previous_line.has_symbol(usize::min(index, WIDTH - 1)) {
                        result += value;
                    } else {
                        current_line.register_number(start, end, value);
                    }

                    State::Dot
                }
                (State::Number { value, .. }, State::Symbol) => {
                    result += value;
                    current_line.register_symbol(index);

                    if let Some(value) = previous_line.get_number(index) {
                        result += value;
                    }

                    State::Symbol
                }
                (State::Dot | State::Symbol, State::Symbol) => {
                    current_line.register_symbol(index);

                    if let Some(value) = previous_line.get_number(index) {
                        result += value;
                    }

                    State::Symbol
                }
                (State::Dot | State::Symbol, State::Dot) => State::Dot,
            };
        }

        previous_line = current_line.finish();
        current_line = LineState::<CurrentLine>::new();
    }

    result
}

#[derive(Debug)]
enum StateGears {
    Other,
    Number {
        start: usize,
        end: usize,
        value: usize,
        is_eligible: bool,
    },
    Gear {
        count: u8,
    },
}

#[aoc(day3, part2)]
pub fn part2(input: &[u8]) -> usize {
    let mut result = 0;

    let mut current_line = LineState::<CurrentLine>::new();
    let mut previous_line = LineState::<CurrentLine>::new().finish();

    for chunk in input.split(|b| *b == b'\n') {
        let mut state = StateGears::Other;

        for (index, byte) in chunk.iter().chain(std::iter::once(&b'.')).enumerate() {
            let next_state = match *byte {
                b'*' => StateGears::Gear { count: 0 },
                b'0'..=b'9' => StateGears::Number {
                    start: index,
                    end: index,
                    value: (*byte - b'0') as usize,
                    is_eligible: false,
                },
                _ => StateGears::Other,
            };

            state = match (state, next_state) {
                (StateGears::Other, StateGears::Number { value, start, .. }) => {
                    StateGears::Number {
                        value,
                        start,
                        end: start,
                        is_eligible: previous_line.has_symbol(index)
                            || previous_line.has_symbol(index.saturating_sub(1)),
                    }
                }
                (StateGears::Gear { count }, StateGears::Number { value, start, .. }) => {
                    StateGears::Number {
                        value,
                        start,
                        end: start,
                        is_eligible: true,
                    }
                }
                (
                    StateGears::Number {
                        value: value_a,
                        start,
                        is_eligible,
                        ..
                    },
                    StateGears::Number {
                        value: value_b,
                        end,
                        ..
                    },
                ) => StateGears::Number {
                    value: value_a * 10 + value_b,
                    start,
                    end,
                    is_eligible: is_eligible || previous_line.has_symbol(index),
                },
                (
                    StateGears::Number {
                        start,
                        end,
                        value,
                        is_eligible: _,
                    },
                    StateGears::Other,
                ) => {
                    current_line.register_number(start, end, value);
                    StateGears::Other
                }
                (StateGears::Number { value, .. }, StateGears::Gear { count }) => {
                    if let Some(value) = previous_line.get_number(index) {
                        result += value;
                    }

                    StateGears::Gear { count }
                }
                (StateGears::Other | StateGears::Gear { .. }, StateGears::Gear { count }) => {
                    current_line.register_symbol(index);

                    if let Some(value) = previous_line.get_number(index) {
                        result += value;
                    }

                    StateGears::Gear { count }
                }
                (StateGears::Other | StateGears::Gear { .. }, StateGears::Other) => {
                    StateGears::Other
                }
            };
        }

        previous_line = current_line.finish();
        current_line = LineState::<CurrentLine>::new();
    }

    result
}

#[cfg(test)]
mod tests {
    const TEST: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_1() {
        assert_eq!(super::part1(TEST.as_bytes()), 4361);
    }

    #[test]
    fn test_2() {
        assert_eq!(super::part2(TEST.as_bytes()), 467835);
    }
}
