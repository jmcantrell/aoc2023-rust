use anyhow::{anyhow, Context};

use aoc::Input;

use crate::core::{Bid, Card, Hand};

type Play = (Hand, Bid);

type Parsed = Vec<Play>;
pub type Parsed1 = Parsed;
pub type Parsed2 = Parsed;

fn parse_card(c: char, j_card: Card) -> anyhow::Result<Card> {
    match c {
        '2' => Ok(Card::Two),
        '3' => Ok(Card::Three),
        '4' => Ok(Card::Four),
        '5' => Ok(Card::Five),
        '6' => Ok(Card::Six),
        '7' => Ok(Card::Seven),
        '8' => Ok(Card::Eight),
        '9' => Ok(Card::Nine),
        'T' => Ok(Card::Ten),
        'J' => Ok(j_card),
        'Q' => Ok(Card::Queen),
        'K' => Ok(Card::King),
        'A' => Ok(Card::Ace),
        _ => Err(anyhow!("invalid card: {:?}", c)),
    }
}

fn parse_play(s: &str, j_card: Card) -> anyhow::Result<Play> {
    let (left, right) = s
        .split_once(|c: char| c.is_whitespace())
        .context("expected hand/bid to be delimited by whitespace")?;

    let hand = left
        .chars()
        .enumerate()
        .map(|(i, c)| parse_card(c, j_card).with_context(|| format!("card number {}", i + 1)))
        .collect::<Result<Vec<_>, _>>()?
        .try_into()
        .context("hand")?;

    let bid = right.parse().context("bid")?;

    Ok((hand, bid))
}

fn parse(input: Input, j_card: Card) -> anyhow::Result<Parsed> {
    input
        .lines()
        .enumerate()
        .map(|(i, s)| parse_play(s, j_card).with_context(|| format!("play number {}", i + 1)))
        .collect::<Result<Vec<_>, _>>()
}

pub fn parse1(input: Input) -> anyhow::Result<Parsed1> {
    parse(input, Card::Jack)
}

pub fn parse2(input: Input) -> anyhow::Result<Parsed2> {
    parse(input, Card::Joker)
}

#[cfg(test)]
mod tests {
    use aoc::Input;

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn parse1() -> anyhow::Result<()> {
        dbg!(super::parse1(INPUT)?);
        Ok(())
    }

    #[test]
    fn parse2() -> anyhow::Result<()> {
        dbg!(super::parse2(INPUT)?);
        Ok(())
    }
}
