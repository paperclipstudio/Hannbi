#![allow(dead_code)]
const HAND_SIZE:usize = 5;

mod card;
use std::backtrace::{Backtrace, BacktraceStatus};
use crate::card::{Deck, Suit};
use std::process::Command;
use card::Card;
use dialoguer::{
    console::Term, 
    theme::ColorfulTheme,
    Select
};

enum Infomation {
    Suit(card::Suit),
    Number(card::Number),
    NotCard((u8, u8))
}

#[derive(Clone)]
struct Hand {
    cards: [card::Card; 5]
}

impl Hand {

    fn new_from_deck(deck:&mut Deck) -> Option<Self> {
        if deck.size() < HAND_SIZE {
            return None
        }
        Some(Hand{cards :[0;5].map(|_|deck.pull().unwrap())})
    }

    fn apply_info(mut self: Hand, info:Infomation) -> Hand {
        self.cards = match info {
            Infomation::Suit(suit) => {
                self.cards.map(|c: Card|c.learn_suit(suit.as_u8()))
            },
            Infomation::Number(number) => {
                self.cards.map(|c|c.learn_number(number.as_u8()))
            },
            Infomation::NotCard(card_value) => {
                self.cards.map(|c|c.remove_option(&card_value))
            }
        };
        self
    }
    
    fn total_infomation(&self) -> f32 {
        self.cards.map(|c:Card| c.missing_information_as_bits())
            .iter().sum()
    }

    fn print(&self) {
        self.cards.map(|c| {c.print(); println!("Woror")});
    }

    pub fn how_much_info_gained(self, info:Infomation) -> f32 {
        self.total_infomation() - 
        self.apply_info(info).total_infomation()
    }
}

fn how_much_info_gained(hand: Hand, info:Infomation) -> f32 {
    hand.total_infomation() - 
    hand.apply_info(info).total_infomation()
}



fn helper() {
    let my_hand: [card::Card; 5] = [card::Card::new((0, 0)); 5];

    loop {
        my_hand.iter().for_each(|c| c.print());
    }
}


fn ask_number_or_suit(hand: Hand) -> Hand {
    const NUM_OR_SUIT: [&str; 2] = ["Number", "Suit"];
    let info_type = Select::with_theme(&ColorfulTheme::default())
        .items(&NUM_OR_SUIT)
        .default(0)
        .interact_on_opt(&Term::stderr()).unwrap();
    match info_type {
        Some(0) => ask_which_number(hand),
        Some(1) => ask_which_suit(hand),
        Some(_) => ask_number_or_suit(hand),
        None => ask_number_or_suit(hand)
    }
}

fn ask_information_type(hand: Hand) -> Hand {
    const INPUTS: [&str; 2] = ["Information", "Not Card"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&INPUTS)
        .default(0)
        .interact_on_opt(&Term::stderr()).unwrap();
    return match selection {
        Some(0) => ask_number_or_suit(hand),
        Some(1) => ask_what_card(hand),
        _ => {
            println!("Error");
            ask_information_type(hand)
        }
    }
}

fn ask_what_card(hand: Hand) -> Hand {
    const NUMBERS: [&str; 5] = ["0", "1", "2", "3", "4"];
    const SUITS: [&str; 5] = ["Red", "Orange", "Yellow", "Green", "Blue"];
    let number = Select::with_theme(&ColorfulTheme::default())
        .items(&NUMBERS)
        .default(0)
        .interact()
        .unwrap() as u8;
    let suit = Select::with_theme(&ColorfulTheme::default())
        .items(&SUITS)
        .default(0)
        .interact()
        .unwrap() as u8;
    return hand.apply_info(Infomation::NotCard((number, suit)));
}

fn ask_which_number(hand: Hand) -> Hand {
    use card::Number;
    const NUMBERS: [&str; 5] = ["0", "1", "2", "3", "4"];
    let mut options: Vec<String> = Vec::new();
    for num in 0..5 {
        let info = hand.clone().how_much_info_gained(Infomation::Number(Number::all()[num]));
        options.push(format!("{num}: {info}bits"))
    }
    match Select::with_theme(&ColorfulTheme::default())
    .items(&options).interact() {
        Ok(suit) => hand.apply_info(Infomation::Number(Number::all()[suit])),
        Err(e) => {
            print!("Error {e}");
            ask_which_number(hand)
        }
    }
}

fn ask_which_suit(hand: Hand) -> Hand {
    match Select::with_theme(&ColorfulTheme::default())
        .items(&Suit::all().map(|s|s.as_str()))

        .default(0)
        .interact() {
            Ok(suit) => hand.apply_info(Infomation::Suit(Suit::all()[suit])),
            Err(_) => ask_which_suit(hand)
        }

}

fn main() {
    let mut deck = Deck::new();
    let mut my_hand = Hand::new_from_deck(&mut deck).unwrap();
    let your_hand = Hand::new_from_deck(&mut deck).unwrap();

    for card in your_hand.cards {
        let tmp = my_hand.apply_info(Infomation::NotCard(card.clone().value));
        my_hand = tmp;
    }

    loop {
        match Command::new("clear").output() {
            Ok(m) => println!("{:?}", m),
            Err(e) => println!("Error: {}", e),
        };
        let info = my_hand.total_infomation();
        println!("Missing information: {info}");
        my_hand.print();
        my_hand = ask_information_type(my_hand);
        let bt = Backtrace::capture();
        if bt.status() == BacktraceStatus::Captured {
            print!("{}", bt);
        }
    }
    
}

struct Game {
    hands: [card::Hand; HAND_SIZE],
    middle: [u8; 5],
    turn: usize,
    resource: u8,
    fuse: u8,
    deck: card::Deck,
}

impl Game {
    pub fn new() -> Game {
        // create deck
        // Shuffle 
        // deal
        Game {
            hands: [card::Hand::new(); HAND_SIZE],
            middle: [0;5],
            turn: 0,
            resource: 8,
            fuse: 2,
            deck: card::Deck::new(),
        }
    }


    pub fn play(action:Action) {
       match action {
           Action::Play(_u8) => {
           },
           Action::Discard(_u8) => {
           },
           Action::GiveInformation(_info) => {
           }
       }
    }

    pub fn is_won(&self) -> bool {
        self.middle == [5;5]
    }


}

enum Action {
    Play(usize),
    Discard(usize),
    GiveInformation(InformationType)
}

enum InformationType {
    Colour(u8),
    Number(u8)
}

#[cfg(tests)]
mod tests {
    use super::*;
    #[test]
    fn setup() {
        let game = Game();
    }
}
