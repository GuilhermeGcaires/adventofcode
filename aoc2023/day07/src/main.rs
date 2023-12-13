use std::{collections::HashMap, fs};

#[derive(Debug, Clone)]
struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,
    bet: i64,
    score: i64,
}

impl Hand {
    fn new(cards: Vec<Card>, bet: i64, hand_type: HandType) -> Hand {
        Self {
            cards,
            bet,
            hand_type,
            score: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn from_char(ch: char) -> Self {
        match ch {
            '2' => Self::Two, // 0x0
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::Ten,
            'J' => Self::Jack,
            'Q' => Self::Queen,
            'K' => Self::King,
            'A' => Self::Ace,
            _ => panic!("code bug"),
        }
    }
}

fn main() {
    let input = fs::read_to_string("./input/input.txt").expect("Could not open file");

    let mut all_hands = parse(&input);
    let mut all_hands2 = all_hands.clone();
    process_part1(&mut all_hands);
    process_part2(&mut all_hands2);
}

fn process_part1(all_hands: &mut Vec<Hand>) {
    all_hands.sort_by(|a, b| {
        let type_cmp = a.hand_type.cmp(&b.hand_type);
        if type_cmp != std::cmp::Ordering::Equal {
            return type_cmp;
        }

        for i in 0..5 {
            let card_cmp = a.cards[i].cmp(&b.cards[i]);
            if card_cmp == std::cmp::Ordering::Less {
                return card_cmp;
            } else if card_cmp == std::cmp::Ordering::Greater {
                return card_cmp;
            }
        }

        std::cmp::Ordering::Equal
    });

    let total_winnings: i64 = all_hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| hand.bet * (rank as i64 + 1))
        .sum();

    println!("Total Winnings: {}", total_winnings);
}

fn process_part2(all_hands: &mut Vec<Hand>) {
    for hand in all_hands.into_iter() {
        for card in &mut hand.cards {
            if *card == Card::Jack {
                *card = Card::Joker;
            }
        }
    }
    println!("All hands{:?}", all_hands);

    for hand in all_hands.into_iter() {
        let mut joker_count = 0;
        for card in &hand.cards {
            if *card == Card::Joker {
                joker_count += 1;
            }
        }
        match joker_count {
            0 => match hand.hand_type {
                HandType::HighCard => hand.hand_type = HandType::HighCard,
                HandType::OnePair => hand.hand_type = HandType::OnePair,
                HandType::TwoPair => hand.hand_type = HandType::TwoPair,
                HandType::ThreeKind => hand.hand_type = HandType::ThreeKind,
                HandType::FullHouse => hand.hand_type = HandType::FullHouse,
                HandType::FourKind => hand.hand_type = HandType::FourKind,
                HandType::FiveKind => hand.hand_type = HandType::FiveKind,
            },
            1 => match hand.hand_type {
                HandType::HighCard => hand.hand_type = HandType::OnePair,
                HandType::OnePair => hand.hand_type = HandType::ThreeKind,
                HandType::TwoPair => hand.hand_type = HandType::FullHouse,
                HandType::ThreeKind => hand.hand_type = HandType::FourKind,
                HandType::FullHouse => hand.hand_type = HandType::FourKind,
                HandType::FourKind => hand.hand_type = HandType::FiveKind,
                HandType::FiveKind => hand.hand_type = HandType::FiveKind,
            },
            2 => match hand.hand_type {
                HandType::HighCard => hand.hand_type = HandType::ThreeKind,
                HandType::OnePair => hand.hand_type = HandType::FourKind,
                HandType::TwoPair => hand.hand_type = HandType::FourKind,
                HandType::ThreeKind => hand.hand_type = HandType::FiveKind,
                _ => hand.hand_type = HandType::FiveKind,
            },
            3 => match hand.hand_type {
                HandType::HighCard => hand.hand_type = HandType::FourKind,
                HandType::OnePair => hand.hand_type = HandType::FiveKind,
                _ => hand.hand_type = HandType::FiveKind,
            },
            4 => match hand.hand_type {
                HandType::HighCard => hand.hand_type = HandType::FiveKind,
                _ => hand.hand_type = HandType::FiveKind,
            },
            5 => match hand.hand_type {
                _ => hand.hand_type = HandType::FiveKind,
            },
            _ => panic!("Code failed"),
        }
    }

    all_hands.sort_by(|a, b| {
        let type_cmp = a.hand_type.cmp(&b.hand_type);
        if type_cmp != std::cmp::Ordering::Equal {
            return type_cmp;
        }

        for i in 0..5 {
            let card_cmp = a.cards[i].cmp(&b.cards[i]);
            if card_cmp == std::cmp::Ordering::Less {
                return card_cmp;
            } else if card_cmp == std::cmp::Ordering::Greater {
                return card_cmp;
            }
        }

        std::cmp::Ordering::Equal
    });

    let total_winnings: i64 = all_hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| hand.bet * (rank as i64 + 1))
        .sum();

    println!("Total Winnings: {}", total_winnings);
}

fn parse(input: &str) -> Vec<Hand> {
    let mut all_hands: Vec<Hand> = Vec::new();
    for line in input.lines() {
        let (hand, bet) = line.split_once(" ").unwrap();
        let bet = bet.parse::<i64>().unwrap();
        let cards = hand.chars().map(Card::from_char).collect::<Vec<_>>();
        let hand_type = get_hand_type2(&cards);
        let hand_parsed = Hand::new(cards, bet, hand_type);
        all_hands.push(hand_parsed);
    }
    all_hands
}

fn get_hand_type(hand: &Vec<Card>) -> HandType {
    let mut occurrences: HashMap<Card, u8> = HashMap::new();
    for card in hand {
        *occurrences.entry(card.clone()).or_insert(0) += 1;
    }

    if occurrences.values().any(|&count| count == 5) {
        HandType::FiveKind
    } else if occurrences.values().any(|&count| count == 4) {
        HandType::FourKind
    } else if occurrences.values().any(|&count| count == 3)
        && occurrences.values().any(|&count| count == 2)
    {
        HandType::FullHouse
    } else if occurrences.values().any(|&count| count == 3) {
        HandType::ThreeKind
    } else if occurrences.values().filter(|&&count| count == 2).count() == 2 {
        HandType::TwoPair
    } else if occurrences.values().any(|&count| count == 2) {
        HandType::OnePair
    } else {
        HandType::HighCard
    }
}

fn get_hand_type2(hand: &Vec<Card>) -> HandType {
    let mut occurrences: HashMap<Card, u8> = HashMap::new();
    for card in hand {
        if *card == Card::Jack {
            continue;
        } else {
            *occurrences.entry(card.clone()).or_insert(0) += 1;
        }
    }

    if occurrences.values().any(|&count| count == 5) {
        HandType::FiveKind
    } else if occurrences.values().any(|&count| count == 4) {
        HandType::FourKind
    } else if occurrences.values().any(|&count| count == 3)
        && occurrences.values().any(|&count| count == 2)
    {
        HandType::FullHouse
    } else if occurrences.values().any(|&count| count == 3) {
        HandType::ThreeKind
    } else if occurrences.values().filter(|&&count| count == 2).count() == 2 {
        HandType::TwoPair
    } else if occurrences.values().any(|&count| count == 2) {
        HandType::OnePair
    } else {
        HandType::HighCard
    }
}
