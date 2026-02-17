//! # Poker - 扑克牌型比较
//!
//! 解析扑克手牌，按牌型排序，返回赢家。牌型从高到低：同花顺、四条、葫芦、同花、顺子、三条、两对、一对、高牌。
//!
//! ## 考点
//! - 解析与 TryFrom
//! - counter crate 统计频率
//! - PartialOrd 实现比较

use std::cmp::Ordering;

use counter::Counter;

/// 返回赢家手牌（原始字符串引用）
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut parsed: Vec<Hand> = hands
        .iter()
        .filter_map(|s| Hand::try_from(*s).ok())
        .collect();
    parsed.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Less));
    let last = match parsed.last() {
        Some(l) => l,
        None => return vec![],
    };
    parsed
        .iter()
        .rev()
        .take_while(|h| h.partial_cmp(&last) == Some(Ordering::Equal))
        .map(|h| h.source)
        .collect()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Copy, Hash)]
enum Suit {
    Spades,
    Clubs,
    Diamonds,
    Hearts,
}

impl TryFrom<&str> for Suit {
    type Error = &'static str;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "S" => Ok(Suit::Spades),
            "C" => Ok(Suit::Clubs),
            "D" => Ok(Suit::Diamonds),
            "H" => Ok(Suit::Hearts),
            _ => Err("Invalid suit"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Rank {
    Num(u8),
    Jack,
    Queen,
    King,
    Ace,
}

impl Rank {
    fn value(&self) -> usize {
        match self {
            Rank::Ace => 14,
            Rank::King => 13,
            Rank::Queen => 12,
            Rank::Jack => 11,
            Rank::Num(n) => *n as usize,
        }
    }
}

impl PartialOrd for Rank {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.value().cmp(&other.value()))
    }
}

impl TryFrom<&str> for Rank {
    type Error = &'static str;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "A" => Ok(Rank::Ace),
            "K" => Ok(Rank::King),
            "Q" => Ok(Rank::Queen),
            "J" => Ok(Rank::Jack),
            "10" => Ok(Rank::Num(10)),
            "9" => Ok(Rank::Num(9)),
            "8" => Ok(Rank::Num(8)),
            "7" => Ok(Rank::Num(7)),
            "6" => Ok(Rank::Num(6)),
            "5" => Ok(Rank::Num(5)),
            "4" => Ok(Rank::Num(4)),
            "3" => Ok(Rank::Num(3)),
            "2" => Ok(Rank::Num(2)),
            _ => Err("Invalid rank"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    fn try_parse(s: &str) -> Result<Self, &'static str> {
        let split = if s.starts_with("10") { 2 } else { 1 };
        Ok(Card {
            rank: Rank::try_from(&s[..split])?,
            suit: Suit::try_from(&s[split..])?,
        })
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.rank.partial_cmp(&other.rank)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum PokerHand {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
}

impl PokerHand {
    fn is_ace_low_straight(cards: &[Card]) -> bool {
        cards[0].rank.value() == 2
            && cards[4].rank == Rank::Ace
            && cards
                .windows(2)
                .take(3)
                .all(|w| w[1].rank.value() - w[0].rank.value() == 1)
    }

    fn analyze(cards: &[Card]) -> Result<Self, &'static str> {
        if cards.len() != 5 {
            return Err("Not 5 cards");
        }
        let suits: Counter<_> = cards.iter().map(|c| c.suit).collect();
        let is_flush = suits.most_common().first().map(|(_, n)| *n) == Some(5);

        let is_straight = cards
            .windows(2)
            .all(|w| w[1].rank.value() - w[0].rank.value() == 1)
            || Self::is_ace_low_straight(cards);

        if is_flush && is_straight {
            return Ok(PokerHand::StraightFlush);
        }

        let ranks: Counter<_> = cards.iter().map(|c| c.rank).collect();
        let counts: Vec<_> = ranks.most_common().into_iter().map(|(_, n)| n).collect();
        let (most, second) = (counts.get(0).copied(), counts.get(1).copied());

        if most == Some(4) {
            return Ok(PokerHand::FourOfAKind);
        }
        if most == Some(3) && second == Some(2) {
            return Ok(PokerHand::FullHouse);
        }
        if is_flush {
            return Ok(PokerHand::Flush);
        }
        if is_straight {
            return Ok(PokerHand::Straight);
        }
        if most == Some(3) {
            return Ok(PokerHand::ThreeOfAKind);
        }
        if most == Some(2) && second == Some(2) {
            return Ok(PokerHand::TwoPair);
        }
        if most == Some(2) {
            return Ok(PokerHand::OnePair);
        }
        Ok(PokerHand::HighCard)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand<'a> {
    source: &'a str,
    cards: [Card; 5],
    hand_type: PokerHand,
}

impl Hand<'_> {
    fn cmp_high_card(&self, other: &Hand, idx: usize) -> Ordering {
        let c = self.cards[idx]
            .rank
            .value()
            .cmp(&other.cards[idx].rank.value());
        if c == Ordering::Equal && idx > 0 {
            self.cmp_high_card(other, idx - 1)
        } else {
            c
        }
    }

    fn value_by_freq(&self) -> (Option<Rank>, Option<Rank>, Option<Rank>) {
        let ranks: Counter<_> = self.cards.iter().map(|c| c.rank).collect();
        let mut iter = ranks
            .most_common_tiebreaker(|a, b| b.partial_cmp(a).unwrap_or(Ordering::Equal))
            .into_iter()
            .map(|(r, _)| r);
        (iter.next(), iter.next(), iter.next())
    }

    fn cmp_by_freq(&self, other: &Hand) -> Ordering {
        let (s1, s2, s3) = self.value_by_freq();
        let (o1, o2, o3) = other.value_by_freq();
        s1.partial_cmp(&o1)
            .unwrap_or(Ordering::Equal)
            .then_with(|| s2.partial_cmp(&o2).unwrap_or(Ordering::Equal))
            .then_with(|| s3.partial_cmp(&o3).unwrap_or(Ordering::Equal))
    }

    fn cmp_straight(&self, other: &Hand) -> Ordering {
        let s = if PokerHand::is_ace_low_straight(&self.cards) {
            5
        } else {
            self.cards[4].rank.value()
        };
        let o = if PokerHand::is_ace_low_straight(&other.cards) {
            5
        } else {
            other.cards[4].rank.value()
        };
        s.cmp(&o)
    }
}

impl PartialOrd for Hand<'_> {
    fn partial_cmp(&self, other: &Hand) -> Option<Ordering> {
        Some(
            self.hand_type
                .cmp(&other.hand_type)
                .then_with(|| match self.hand_type {
                    PokerHand::HighCard => self.cmp_high_card(other, 4),
                    PokerHand::OnePair
                    | PokerHand::TwoPair
                    | PokerHand::ThreeOfAKind
                    | PokerHand::FullHouse
                    | PokerHand::FourOfAKind => self.cmp_by_freq(other),
                    PokerHand::Straight | PokerHand::StraightFlush => self.cmp_straight(other),
                    PokerHand::Flush => self.cmp_high_card(other, 4),
                }),
        )
    }
}

impl<'a> TryFrom<&'a str> for Hand<'a> {
    type Error = &'static str;
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let mut cards: Vec<Card> = s
            .split_whitespace()
            .map(Card::try_parse)
            .collect::<Result<_, _>>()?;
        if cards.len() != 5 {
            return Err("Need 5 cards");
        }
        cards.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
        Ok(Hand {
            source: s,
            cards: [cards[0], cards[1], cards[2], cards[3], cards[4]],
            hand_type: PokerHand::analyze(&cards)?,
        })
    }
}
