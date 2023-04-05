const HAND_SIZE:usize = 5;
mod Card;
use std::process::Command;
use dialoguer::{
    console::Term, 
    theme::ColorfulTheme,
    Select,
    Input
};

use Card::Deck;


fn helper() {
    let my_hand: [Card::Card; 5] = [Card::Card::new((0, 0)); 5];
    loop {
        my_hand.iter().for_each(|c| c.print());
    }
}


Hand = [Card::Card; 5];
fn ask_number_or_suit(hand: Hand) -> Hand {
    const num_or_suit: [&str; 2] = ["Number", "Suit"];
    let info_type = Select::with_theme(&ColorfulTheme::default())
        .items(&num_or_suit)
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
    return hand.map(|c| c.remove_option(&(number, suit)))
}

fn ask_which_number(hand: Hand) -> Hand {
    const NUMBERS: [&str; 5] = ["0", "1", "2", "3", "4"];
    match Select::with_theme(&ColorfulTheme::default()) .items(&NUMBERS).interact() {
        Ok(suit) => hand.map(|c| c.learn_number(suit as u8)),
        Err(e) => {
            print!("Error {e}");
            ask_which_number(hand)
        }
    }
}

fn ask_which_suit(hand: Hand) -> Hand {
    const suits: [&str; 5] = ["Red", "Orange", "Yellow", "Green", "Blue"];
    match Select::with_theme(&ColorfulTheme::default())
        .items(&suits)
        .default(0)
        .interact() {
        Ok(suit) => hand.map(|c| c.learn_suit(suit as u8)),
        Err(_) => ask_which_suit(hand)
        }

}

fn main() {
    let mut deck = Deck::new().shuffle();
    let mut my_hand: [Card::Card; 5] = [Card::Card::new((0, 0)); 5];
    let mut your_hand: [Card::Card; 5] = [Card::Card::new((0, 0)); 5];
    for i in 0..5 {
        print!("Pulling card {}", i);
        my_hand[i] = deck.pull().unwrap();
        your_hand[i] = deck.pull().unwrap();
    }
    
    your_hand.map(|card| {
            my_hand = my_hand.map(|c| c.remove_option(&card.value()));
            card.is_value(*card.value())
        });

    let (number1, _) = my_hand[0].value().clone();
    let (_, suit2) = my_hand[1].value().clone();
    
    my_hand.iter()
        .map(|c| c.learn_number(number1))   
        .map(|c| c.learn_suit(suit2))
        .for_each(|c| c.print());
    // Drawing 5 new cards  
    //
    for i in 0..5 {
        my_hand[i] = deck.pull().unwrap();
    }

    loop {
        match Command::new("clear").output() {
            Ok(m) => println!("{:?}", m),
            Err(e) => println!("Error: {}", e),
        };
        my_hand.map(|c| c.print());
        my_hand = ask_information_type(my_hand);
    }
    
}

struct Game {
    hands: [Card::Hand; HAND_SIZE],
    middle: [u8; 5],
    turn: usize,
    resource: u8,
    fuse: u8,
    deck: Card::Deck,
}

impl Game {
    pub fn new() -> Game {
        // create deck
        // Shuffle 
        // deal
        Game {
            hands: [Card::Hand::new(); HAND_SIZE],
            middle: [0;5],
            turn: 0,
            resource: 8,
            fuse: 2,
            deck: Card::Deck::new(),
        }
    }


    pub fn play(action:Action) {
       match action {
           Action::Play(u8) => {
           },
           Action::Discard(u8) => {
           },
           Action::GiveInformation(info) => {
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
