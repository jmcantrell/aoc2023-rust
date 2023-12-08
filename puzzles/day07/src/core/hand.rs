use anyhow::ensure;

use std::cmp::Ordering;
use std::collections::HashMap;
use std::convert::TryFrom;

type Inner = Vec<Card>;

pub const HAND_LEN: usize = 5;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hand(Inner);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Card {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandKind {
    FiveKind,
    FourKind,
    FullHouse,
    ThreeKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl Hand {
    pub fn kind(&self) -> HandKind {
        let mut counts: HashMap<&Card, usize> =
            self.0.iter().fold(HashMap::new(), |mut counts, card| {
                *counts.entry(card).or_default() += 1;
                counts
            });

        // If this hand contains jokers, add that count to the largest group.
        if let Some(joker_count) = counts.remove(&Card::Joker) {
            let card = counts
                .iter()
                .max_by_key(|&(_, count)| count)
                .map(|(card, _)| card)
                .unwrap_or(&&Card::Ace);

            *counts.entry(card).or_default() += joker_count;
        }

        match counts.len() {
            1 => HandKind::FiveKind,
            2 => match counts.into_values().next().unwrap() {
                1 | 4 => HandKind::FourKind,
                2 | 3 => HandKind::FullHouse,
                _ => unreachable!(),
            },
            3 => match counts.into_values().find(|&count| count == 3) {
                Some(_) => HandKind::ThreeKind,
                None => HandKind::TwoPair,
            },
            4 => HandKind::OnePair,
            5 => HandKind::HighCard,
            _ => unreachable!(),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.kind()
            .cmp(&other.kind())
            .then_with(|| self.0.cmp(&other.0).reverse())
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl TryFrom<Inner> for Hand {
    type Error = anyhow::Error;

    fn try_from(cards: Inner) -> Result<Self, Self::Error> {
        ensure!(
            cards.len() == HAND_LEN,
            "expected there to be {} cards, but there were {}",
            HAND_LEN,
            cards.len()
        );

        Ok(Self(cards))
    }
}
