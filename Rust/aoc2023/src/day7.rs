use std::{fmt::Debug, hash::Hash, iter::zip, ops::Deref};

use aoc_runner_derive::aoc;

use utils::counting_set::CountingSet;

trait CardTrait: Ord + PartialOrd + PartialEq + Eq + Copy + Clone + for<'a> From<&'a u8> {}

trait CardsTrait<Card: CardTrait>: Debug {
    fn get_hand_type(&self) -> HandType;
    fn get_cards(&self) -> &[Card; 5];
    fn cmp_cards(&self, other: &Self) -> std::cmp::Ordering;
}

impl CardsTrait<CardP1> for Cards<CardP1> {
    fn cmp_cards(&self, other: &Self) -> std::cmp::Ordering {
        let t1 = self.get_hand_type();
        let t2 = other.get_hand_type();

        if t1 == t2 {
            for (self_card, other_card) in zip(self.get_cards(), other.get_cards()) {
                match self_card.cmp(other_card) {
                    std::cmp::Ordering::Equal => continue,
                    cmp => return cmp,
                };
            }
            std::cmp::Ordering::Equal
        } else {
            t1.cmp(&t2)
        }
    }
    fn get_hand_type(&self) -> HandType {
        let counting_set = CountingSet::from(self.iter());

        let mut has_three_of_a_kind = false;
        let mut pairs = 0;
        for count in counting_set.iter_counts() {
            match count {
                5 => return HandType::FiveOfAKind,
                4 => {
                    return HandType::FourOfAKind;
                }
                3 => has_three_of_a_kind = true,
                2 => pairs += 1,
                _ => (),
            }
        }
        if has_three_of_a_kind {
            if pairs == 1 {
                return HandType::FullHouse;
            } else {
                return HandType::ThreeOfAKind;
            }
        }
        if pairs == 2 {
            return HandType::TwoPair;
        }
        if pairs == 1 {
            return HandType::OnePair;
        }

        HandType::HighCard
    }

    fn get_cards(&self) -> &[CardP1; 5] {
        &self.cards
    }
}

impl CardsTrait<CardP2> for Cards<CardP2> {
    fn cmp_cards(&self, other: &Self) -> std::cmp::Ordering {
        let t1 = self.get_hand_type();
        let t2 = other.get_hand_type();

        if t1 == t2 {
            for (self_card, other_card) in zip(self.get_cards(), other.get_cards()) {
                match self_card.cmp(other_card) {
                    std::cmp::Ordering::Equal => continue,
                    cmp => return cmp,
                };
            }
            std::cmp::Ordering::Equal
        } else {
            t1.cmp(&t2)
        }
    }
    fn get_hand_type(&self) -> HandType {
        let counting_set = CountingSet::from(self.iter());

        let joker_count: usize = counting_set.get(&&CardP2::J).unwrap_or(0);

        let mut has_three_of_a_kind = false;
        let mut pairs = 0;
        for (c, count) in counting_set.iter() {
            match (count, c) {
                (5, _) => return HandType::FiveOfAKind,
                (4, CardP2::J) => return HandType::FiveOfAKind,
                (4, _) => {
                    if joker_count == 1 {
                        return HandType::FiveOfAKind;
                    }
                    return HandType::FourOfAKind;
                }
                (3, _) => has_three_of_a_kind = true,
                (2, _) => pairs += 1,
                _ => (),
            }
        }
        if has_three_of_a_kind {
            match joker_count {
                2 => return HandType::FiveOfAKind,
                1 => return HandType::FourOfAKind,
                _ => (),
            }

            if pairs == 1 {
                if joker_count == 3 {
                    return HandType::FiveOfAKind;
                }
                return HandType::FullHouse;
            } else {
                if joker_count == 3 {
                    return HandType::FourOfAKind;
                }
                return HandType::ThreeOfAKind;
            }
        }
        if pairs == 2 {
            match joker_count {
                2 => return HandType::FourOfAKind,
                1 => return HandType::FullHouse,
                _ => (),
            }
            return HandType::TwoPair;
        }
        if pairs == 1 {
            match joker_count {
                2 | 1 => return HandType::ThreeOfAKind,
                _ => (),
            }
            return HandType::OnePair;
        }
        if joker_count == 1 {
            return HandType::OnePair;
        }
        HandType::HighCard
    }

    fn get_cards(&self) -> &[CardP2; 5] {
        &self.cards
    }
}

#[derive(Debug)]
struct Cards<Card: CardTrait> {
    cards: [Card; 5],
}

impl Deref for Cards<CardP2> {
    type Target = [CardP2; 5];

    fn deref(&self) -> &Self::Target {
        &self.cards
    }
}

impl Deref for Cards<CardP1> {
    type Target = [CardP1; 5];

    fn deref(&self) -> &Self::Target {
        &self.cards
    }
}

impl Eq for Cards<CardP2> {}

impl PartialEq for Cards<CardP2> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}
impl Eq for Cards<CardP1> {}

impl PartialEq for Cards<CardP1> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl Ord for Cards<CardP1> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cmp_cards(other)
    }
}

impl Ord for Cards<CardP2> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cmp_cards(other)
    }
}

impl PartialOrd for Cards<CardP1> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialOrd for Cards<CardP2> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct Hand<Card: CardTrait> {
    bid: usize,
    cards: Cards<Card>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum CardP1 {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum CardP2 {
    J,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    Q,
    K,
    A,
}

impl CardTrait for CardP1 {}
impl CardTrait for CardP2 {}

impl From<u8> for CardP1 {
    fn from(value: u8) -> Self {
        match value {
            b'A' => CardP1::A,
            b'K' => CardP1::K,
            b'Q' => CardP1::Q,
            b'J' => CardP1::J,
            b'T' => CardP1::T,
            b'9' => CardP1::Nine,
            b'8' => CardP1::Eight,
            b'7' => CardP1::Seven,
            b'6' => CardP1::Six,
            b'5' => CardP1::Five,
            b'4' => CardP1::Four,
            b'3' => CardP1::Three,
            b'2' => CardP1::Two,
            _ => unreachable!("This is not a card"),
        }
    }
}

impl From<u8> for CardP2 {
    fn from(value: u8) -> Self {
        match value {
            b'A' => CardP2::A,
            b'K' => CardP2::K,
            b'Q' => CardP2::Q,
            b'J' => CardP2::J,
            b'T' => CardP2::T,
            b'9' => CardP2::Nine,
            b'8' => CardP2::Eight,
            b'7' => CardP2::Seven,
            b'6' => CardP2::Six,
            b'5' => CardP2::Five,
            b'4' => CardP2::Four,
            b'3' => CardP2::Three,
            b'2' => CardP2::Two,
            _ => unreachable!("This is not a card"),
        }
    }
}

impl From<&u8> for CardP1 {
    fn from(value: &u8) -> Self {
        (*value).into()
    }
}
impl From<&u8> for CardP2 {
    fn from(value: &u8) -> Self {
        (*value).into()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn parse<Card: CardTrait>(input: &str) -> Vec<Hand<Card>> {
    input
        .lines()
        .map(|line| {
            let mut iter = line.split_ascii_whitespace();
            let cards: [Option<Card>; 5] = iter.next().unwrap().as_bytes().iter().enumerate().fold(
                [None; 5],
                |mut acc, (index, card)| {
                    acc[index] = Some(card.into());
                    acc
                },
            );
            let bid = iter
                .next()
                .unwrap()
                .as_bytes()
                .iter()
                .fold(0_usize, |acc, digit| 10 * acc + (digit - b'0') as usize);

            Hand {
                bid,
                cards: Cards {
                    cards: cards.map(|a| a.unwrap()),
                },
            }
        })
        .collect()
}

#[aoc(day7, part1)]
fn part1(input: &str) -> usize {
    let mut hands = parse::<CardP1>(input);
    hands.sort_by(|a, b| a.cards.cmp(&b.cards));
    hands
        .iter()
        .enumerate()
        .fold(0, |acc, (index, hand)| acc + (index + 1) * hand.bid)
}

#[aoc(day7, part2)]
fn part2(input: &str) -> usize {
    let mut hands = parse::<CardP2>(input);
    hands.sort_by(|a, b| a.cards.cmp(&b.cards));
    // for hand in hands.iter() {
    //     //.filter(|hand| hand.cards.contains(&CardP2::J)) {
    //     dbg!(hand);
    //     dbg!(hand.cards.get_hand_type());
    // }
    hands
        .iter()
        .enumerate()
        .fold(0, |acc, (index, hand)| acc + (index + 1) * hand.bid)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static str {
        "
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
        "
        .trim()
    }

    fn real_input() -> String {
        std::fs::read_to_string("./input/2023/day7.txt")
            .unwrap()
            .trim()
            .to_string()
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(example_input()), 6440);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(example_input()), 5905);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(&real_input()), 249390788);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(&real_input()), 248750248);
    }

    #[test]
    fn cards_test() {
        assert_eq!(
            Cards {
                cards: [CardP2::A, CardP2::Seven, CardP2::J, CardP2::J, CardP2::J],
            }
            .get_hand_type(),
            HandType::FourOfAKind
        )
    }

    #[test]
    fn cards_test2() {
        assert_eq!(
            Cards {
                cards: [CardP2::J, CardP2::J, CardP2::Nine, CardP2::Nine, CardP2::J,],
            }
            .get_hand_type(),
            HandType::FiveOfAKind
        )
    }

    #[test]
    fn cards_test3() {
        assert_eq!(
            Cards {
                cards: [
                    CardP2::A,
                    CardP2::Four,
                    CardP2::Two,
                    CardP2::Eight,
                    CardP2::J,
                ],
            }
            .get_hand_type(),
            HandType::OnePair
        )
    }

    #[test]
    fn cards_test4() {
        assert_eq!(
            Cards {
                cards: [CardP2::K, CardP2::Two, CardP2::Nine, CardP2::J, CardP2::J,],
            }
            .get_hand_type(),
            HandType::ThreeOfAKind
        )
    }

    #[test]
    fn cards_test5() {
        assert_eq!(
            Cards {
                cards: [CardP2::K, CardP2::Two, CardP2::Nine, CardP2::Two, CardP2::J,],
            }
            .get_hand_type(),
            HandType::ThreeOfAKind
        )
    }

    #[test]
    fn cards_test6() {
        assert_eq!(
            Cards {
                cards: [CardP2::K, CardP2::Two, CardP2::Nine, CardP2::Q, CardP2::J,],
            }
            .get_hand_type(),
            HandType::OnePair
        )
    }

    #[test]
    fn cards_test7() {
        assert_eq!(
            Cards {
                cards: [CardP2::K, CardP2::Two, CardP2::Nine, CardP2::Q, CardP2::Q,],
            }
            .get_hand_type(),
            HandType::OnePair
        )
    }

    #[test]
    fn cards_test8() {
        assert_eq!(
            Cards {
                cards: [CardP2::K, CardP2::Two, CardP2::Two, CardP2::Q, CardP2::Q,],
            }
            .get_hand_type(),
            HandType::TwoPair
        )
    }

    #[test]
    fn cards_test9() {
        assert!(
            Cards {
                cards: [
                    CardP2::A,
                    CardP2::Four,
                    CardP2::Two,
                    CardP2::Eight,
                    CardP2::J,
                ],
            } < Cards {
                cards: [
                    CardP2::A,
                    CardP2::Four,
                    CardP2::Two,
                    CardP2::Nine,
                    CardP2::J,
                ],
            }
        )
    }
    #[test]
    fn cards_test10() {
        assert!(
            Cards {
                cards: [
                    CardP2::A,
                    CardP2::Four,
                    CardP2::Two,
                    CardP2::Eight,
                    CardP2::J,
                ],
            } > Cards {
                cards: [
                    CardP2::K,
                    CardP2::Four,
                    CardP2::Two,
                    CardP2::Eight,
                    CardP2::J,
                ],
            }
        )
    }
    #[test]
    fn cards_test11() {
        let c1 = Cards {
            cards: [CardP2::K, CardP2::K, CardP2::Two, CardP2::Eight, CardP2::J],
        };
        let c2 = Cards {
            cards: [
                CardP2::J,
                CardP2::A,
                CardP2::Four,
                CardP2::Two,
                CardP2::Eight,
            ],
        };

        assert_eq!(c1.get_hand_type(), HandType::ThreeOfAKind);
        assert_eq!(c2.get_hand_type(), HandType::OnePair);

        assert!(c1 > c2);
    }

    #[test]
    fn cards_test12() {
        let c1 = Cards {
            cards: [CardP2::K, CardP2::K, CardP2::Two, CardP2::Eight, CardP2::K],
        };
        let c2 = Cards {
            cards: [
                CardP2::A,
                CardP2::Four,
                CardP2::Two,
                CardP2::Eight,
                CardP2::Two,
            ],
        };

        assert_eq!(c1.get_hand_type(), HandType::ThreeOfAKind);
        assert_eq!(c2.get_hand_type(), HandType::OnePair);

        assert!(c1 > c2);
    }

    #[test]
    fn cards_test13() {
        assert!(HandType::HighCard < HandType::OnePair);
        assert!(HandType::OnePair < HandType::TwoPair);
        assert!(HandType::TwoPair < HandType::ThreeOfAKind);
        assert!(HandType::ThreeOfAKind < HandType::FullHouse);
        assert!(HandType::FullHouse < HandType::FourOfAKind);
        assert!(HandType::FourOfAKind < HandType::FiveOfAKind);
    }

    #[test]
    fn cards_test14() {
        let c1 = Cards {
            cards: [CardP2::K, CardP2::K, CardP2::Two, CardP2::Eight, CardP2::J],
        };
        let c2 = Cards {
            cards: [CardP2::K, CardP2::K, CardP2::Two, CardP2::J, CardP2::Eight],
        };

        assert_eq!(c1.get_hand_type(), HandType::ThreeOfAKind);
        assert_eq!(c2.get_hand_type(), HandType::ThreeOfAKind);

        assert!(c1 > c2);
    }

    #[test]
    fn cards_test15() {
        let c1 = &parse::<CardP2>("JKKK2 1")[0].cards;
        let c2 = &parse::<CardP2>("QQQQ2 2")[0].cards;

        assert_eq!(c1.get_hand_type(), HandType::FourOfAKind);
        assert_eq!(c2.get_hand_type(), HandType::FourOfAKind);

        assert!(c1 < c2);
    }
    #[test]
    fn cards_test16() {
        let c1 = &parse::<CardP2>("A6543 1")[0].cards;
        let c2 = &parse::<CardP2>("A426T 2")[0].cards;

        assert_eq!(c1.get_hand_type(), HandType::HighCard);
        assert_eq!(c2.get_hand_type(), HandType::HighCard);

        assert!(c1 > c2);
    }

    #[test]
    fn cards_test17() {
        let c1 = &parse::<CardP2>("53482 1")[0].cards;
        let c2 = &parse::<CardP2>("48573 10")[0].cards;
        let c3 = &parse::<CardP2>("56932 100")[0].cards;

        assert_eq!(c1.get_hand_type(), HandType::HighCard);
        assert_eq!(c2.get_hand_type(), HandType::HighCard);
        assert_eq!(c3.get_hand_type(), HandType::HighCard);

        assert!(c1 > c2);
        assert!(c3 > c2);
        assert!(c3 > c1);
    }

    #[test]
    fn cards_test18() {
        let c1 = &parse::<CardP2>("JJJJJ 1")[0].cards;
        let c2 = &parse::<CardP2>("JJJJ2 1")[0].cards;
        let c3 = &parse::<CardP2>("22222 1")[0].cards;

        assert_eq!(c1.get_hand_type(), HandType::FiveOfAKind);
        assert_eq!(c2.get_hand_type(), HandType::FiveOfAKind);
        assert_eq!(c3.get_hand_type(), HandType::FiveOfAKind);

        assert!(c1 < c2);
        assert!(c1 < c3);
        assert!(c2 < c3);
    }
}
