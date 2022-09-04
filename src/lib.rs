use rand::{thread_rng, prelude::SliceRandom};
use std::collections::VecDeque;
use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    players: Vec<Player>,
    deck: VecDeque<Card>,
    current_card: Card,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    name: String,
    turn: usize,
    cards: Vec<Card>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
enum CardKind {
    WildCard,
    DrawFour,
    DrawTwo,
    Cancel,
    Reverse,
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Card {
    color: Color,
    kind: CardKind,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
enum Color {
    Red,
    Blue,
    Green,
    Yellow,
    Wild,
}

impl Game {
    pub fn new() -> Game {
        let mut game = Game { 
            players: Vec::new(), 
            deck: Card::random_deck(), 
            // Set current card to placeholder
            current_card: Card { color: Color::Wild, kind: CardKind::Cancel }, 
        };

        // 
        game.current_card = game.card_from_deck_top().unwrap();

        game
    }

    fn card_from_deck_top(&mut self) -> Option<Card> {
        self.deck.pop_front()
    }

    fn reshuffle(&mut self) {
        let mut deck: Vec<Card> = self.deck.clone().into();

        deck.shuffle(&mut thread_rng());

        self.deck = deck.into();
    }
}

impl Card {
    /// Generate a random deck, uno decks consist of the following:
    /// 4 1's, 4 2's, 4 3's, etc (all of different colors, Red, Blue, Green, and Yellow)
    /// 4 Draw 2's, 4 Cancel's, 4 Reverse's, 2 Draw 4's, 2 Wild Card's, and 4 0's (all of different colors)
    /// Deck is shuffled
    fn random_deck() -> VecDeque<Card> {
        // Create a deck with all cards
        let mut deck = vec![
            Card { color: Color::Red, kind: CardKind::Reverse }, Card { color: Color::Red, kind: CardKind::DrawTwo }, Card { color: Color::Red, kind: CardKind::Cancel }, Card { color: Color::Red, kind: CardKind::Zero }, Card { color: Color::Red, kind: CardKind::One }, Card { color: Color::Red, kind: CardKind::Two }, Card { color: Color::Red, kind: CardKind::Three }, Card { color: Color::Red, kind: CardKind::Four }, Card { color: Color::Red, kind: CardKind::Five }, Card { color: Color::Red, kind: CardKind::Six }, Card { color: Color::Red, kind: CardKind::Seven }, Card { color: Color::Red, kind: CardKind::Eight }, Card { color: Color::Red, kind: CardKind::Nine },
            Card { color: Color::Blue, kind: CardKind::Reverse }, Card { color: Color::Blue, kind: CardKind::DrawTwo }, Card { color: Color::Blue, kind: CardKind::Cancel }, Card { color: Color::Blue, kind: CardKind::Zero }, Card { color: Color::Blue, kind: CardKind::One }, Card { color: Color::Blue, kind: CardKind::Two }, Card { color: Color::Blue, kind: CardKind::Three }, Card { color: Color::Blue, kind: CardKind::Four }, Card { color: Color::Blue, kind: CardKind::Five }, Card { color: Color::Blue, kind: CardKind::Six }, Card { color: Color::Blue, kind: CardKind::Seven }, Card { color: Color::Blue, kind: CardKind::Eight }, Card { color: Color::Blue, kind: CardKind::Nine },
            Card { color: Color::Green, kind: CardKind::Reverse }, Card { color: Color::Green, kind: CardKind::DrawTwo }, Card { color: Color::Green, kind: CardKind::Cancel }, Card { color: Color::Green, kind: CardKind::Zero }, Card { color: Color::Green, kind: CardKind::One }, Card { color: Color::Green, kind: CardKind::Two }, Card { color: Color::Green, kind: CardKind::Three }, Card { color: Color::Green, kind: CardKind::Four }, Card { color: Color::Green, kind: CardKind::Five }, Card { color: Color::Green, kind: CardKind::Six }, Card { color: Color::Green, kind: CardKind::Seven }, Card { color: Color::Green, kind: CardKind::Eight }, Card { color: Color::Green, kind: CardKind::Nine },
            Card { color: Color::Yellow, kind: CardKind::Reverse }, Card { color: Color::Yellow, kind: CardKind::DrawTwo }, Card { color: Color::Yellow, kind: CardKind::Cancel }, Card { color: Color::Yellow, kind: CardKind::Zero }, Card { color: Color::Yellow, kind: CardKind::One }, Card { color: Color::Yellow, kind: CardKind::Two }, Card { color: Color::Yellow, kind: CardKind::Three }, Card { color: Color::Yellow, kind: CardKind::Four }, Card { color: Color::Yellow, kind: CardKind::Five }, Card { color: Color::Yellow, kind: CardKind::Six }, Card { color: Color::Yellow, kind: CardKind::Seven }, Card { color: Color::Yellow, kind: CardKind::Eight }, Card { color: Color::Yellow, kind: CardKind::Nine },
            Card { color: Color::Wild, kind: CardKind::DrawFour}, Card { color: Color::Wild, kind: CardKind::DrawFour}, Card { color: Color::Wild, kind: CardKind::WildCard }, Card { color: Color::Wild, kind: CardKind::WildCard },
        ];

        // Shuffle the deck
        deck.shuffle(&mut thread_rng());

        // Return the shuffled deck
        deck.into()
    }
}

impl Player {
    pub fn new() -> Player {
        Player { name: String::new(), turn: 0, cards: Vec::new() }
    }   
    pub fn set_name(&mut self, name: &str) {
        self.name = name.into();
    }
    pub fn set_turn(&mut self, turn: usize) {
        self.turn = turn;
    }
}