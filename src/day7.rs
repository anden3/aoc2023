#![allow(long_running_const_eval)]

use bstr::ByteSlice;
use btoi::btou;

#[cfg(test)]
const HAND_COUNT: usize = 5;
#[cfg(not(test))]
const HAND_COUNT: usize = 1000;

const MAX_HASH: usize = 0b11001100110011001100;

static HAND_SCORES: [u8; MAX_HASH + 1] = {
    let mut scores = [0u8; MAX_HASH + 1];

    let mut a = 0u8;

    while a < 13 {
        let mut b = 0u8;

        while b < 13 {
            let mut c = 0u8;

            while c < 13 {
                let mut d = 0u8;

                while d < 13 {
                    let mut e = 0u8;

                    while e < 13 {
                        let mut hash = 0u64;
                        hash |= e as u64;
                        hash |= (d as u64) << 4;
                        hash |= (c as u64) << 8;
                        hash |= (b as u64) << 12;
                        hash |= (a as u64) << 16;

                        scores[hash as usize] = get_hand_score([a, b, c, d, e]);
                        e += 1;
                    }

                    d += 1;
                }

                c += 1;
            }

            b += 1;
        }

        a += 1;
    }

    scores
};

const fn get_hand_hash(hand: [u8; 5]) -> u64 {
    let mut hash = 0u64;
    let mut i = 0;

    while i < 5 {
        let card_value = get_card_value(hand[4 - i]);
        hash |= (card_value as u64) << (i * 4);

        i += 1;
    }

    hash
}

const fn get_card_value(card: u8) -> u8 {
    match card {
        b'2'..=b'9' => card - b'2',
        b'T' => 8,
        b'J' => 9,
        b'Q' => 10,
        b'K' => 11,
        b'A' => 12,
        _ => unreachable!(),
    }
}

const fn get_hand_score(hand: [u8; 5]) -> u8 {
    let mut freqs = [0u8; 5];
    let mut mapping = [usize::MAX; 13];
    let mut i = 0;
    let mut mapped = 0;

    while i < 5 {
        let card = hand[i];
        if mapping[card as usize] == usize::MAX {
            mapping[card as usize] = mapped;
            mapped += 1;
        }

        freqs[mapping[card as usize]] += 1;
        i += 1;
    }

    get_freq_score(freqs)
}

const fn get_freq_score(freqs: [u8; 5]) -> u8 {
    let sorted = sort_arr(freqs);

    let hand_type = match sorted {
        [.., 5] => HandType::FiveOfAKind,
        [.., 2, 3] => HandType::FullHouse,
        [.., 4] => HandType::FourOfAKind,
        [.., 3] => HandType::ThreeOfAKind,
        [.., 2, 2] => HandType::TwoPair,
        [.., 2] => HandType::OnePair,
        [.., 1] => HandType::HighCard,
        _ => unreachable!(),
    };

    hand_type as u8
}

const fn sort_arr(mut hand: [u8; 5]) -> [u8; 5] {
    loop {
        let mut swapped = false;
        let mut i = 1;
        while i < hand.len() {
            if hand[i - 1] > hand[i] {
                let left = hand[i - 1];
                let right = hand[i];
                hand[i - 1] = right;
                hand[i] = left;
                swapped = true;
            }
            i += 1;
        }
        if !swapped {
            break;
        }
    }
    hand
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
enum HandType {
    HighCard = 0,
    OnePair = 1,
    TwoPair = 2,
    ThreeOfAKind = 3,
    FullHouse = 4,
    FourOfAKind = 5,
    FiveOfAKind = 6,
}

#[aoc(day7, part1)]
pub fn part1(input: &[u8]) -> i64 {
    let mut result = 0;

    let mut hands = [(0usize, 0i64); HAND_COUNT];

    for (i, hand) in input
        .split(|b| *b == b'\n')
        .filter(|l| !l.is_empty())
        .enumerate()
    {
        let (hand, bid) = hand.split_once_str(" ").unwrap();
        let hand = hand.try_into().unwrap();
        let hand = get_hand_hash(hand);
        let bid = btou(bid).unwrap();

        hands[i] = (hand as usize, bid);
    }

    hands.sort_unstable_by(|(a, _), (b, _)| HAND_SCORES[*a].cmp(&HAND_SCORES[*b]).then(a.cmp(b)));

    for (i, (_, bid)) in hands.into_iter().enumerate() {
        result += (i as i64 + 1) * bid;
    }

    result
}

// #[aoc(day7, part2)]
// pub fn part2(input: &str) -> i64 {
//     let mut result = 0;
//
//     result
// }

#[cfg(test)]
mod tests {
    const TEST: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_1() {
        assert_eq!(super::part1(TEST.as_bytes()), 6440);
    }

    // #[test]
    // fn test_2() {
    //     assert_eq!(super::part2(TEST), 6440);
    // }
}
