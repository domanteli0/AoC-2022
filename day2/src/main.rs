use std::{fs, cmp::Ordering, str::FromStr};

// "The first column is what your opponent is going to play: 
// A for Rock, 
// B for Paper, 
// and C for Scissors.

// The second column, you reason, must be what you should play in response: 
// X for Rock, 
// Y for Paper,
//  and Z for Scissors.

// The score for a single round is 
// the score for the shape you selected
//  (1 for Rock, 2 for Paper, and 3 for Scissors) 
// plus the score for the outcome of the round 
// (0 if you lost, 3 if the round was a draw, and 6 if you won).

#[derive(Debug)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    fn cmp(&self, other: &Hand) -> Ordering
    {
        match self {
            Hand::Rock => match other {
                Hand::Scissors => Ordering::Greater,
                Hand::Paper => Ordering::Less,
                _ => Ordering::Equal
            },
            Hand::Paper => match other {
                Hand::Rock => Ordering::Greater,
                Hand::Scissors => Ordering::Less,
                _ => Ordering::Equal
            },
            Hand::Scissors => match other {
                Hand::Rock => Ordering::Less,
                Hand::Paper => Ordering::Greater,
                _ => Ordering::Equal
            }
        }
    }
}

impl FromStr for Hand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        match s {
            "A" => Ok(Hand::Rock),
            "X" => Ok(Hand::Rock),
            "B" => Ok(Hand::Paper),
            "Y" => Ok(Hand::Paper),
            "C" => Ok(Hand::Scissors),
            "Z" => Ok(Hand::Scissors),
            _ => Err(String::from("failed to parse"))
        }
    }    
}

// X means you need to lose, 
// Y means you need to end the round in a draw,
//  and Z means you need to win. Good luck!"


fn main() {    
    let strs = fs::read_to_string("input.txt")
        .expect("Failed to read input.txt");
    
    let mut pairs: Vec<(Hand, Hand)> = Vec::new();

    for str in strs.split("\n")
    {
        if str.len() < 1 { continue; }
        let pair: (Hand, Hand) = {
            let pair = str.split(' ');
            let mut it = pair.into_iter().into_iter();

            let fst = it.next().expect("no next found").parse::<Hand>().expect("failed to parse");
            let snd: Hand = match fst {
                Hand::Rock => match it.next().expect("no next found") {
                    "X" => Hand::Scissors,
                    "Y" => Hand::Rock,
                    "Z" => Hand::Paper,
                    _ => panic!("unexpected symbol in second pair")
                },
                Hand::Paper => match it.next().expect("no next found") {
                    "X" => Hand::Rock,
                    "Y" => Hand::Paper,
                    "Z" => Hand::Scissors,
                    _ => panic!("unexpected symbol in second pair")
                }
                Hand::Scissors => match it.next().expect("no next found") {
                    "X" => Hand::Paper,
                    "Y" => Hand::Scissors,
                    "Z" => Hand::Rock,
                    _ => panic!("unexpected symbol in second pair")
                }
            };

            (fst, snd)

        };

        println!("{pair:?}");
        pairs.push(pair);

    }
    println!("{pairs:?}");

    let mut score = 0;

    for pair in pairs
    {
        let (a, b) = pair;
        let score0 = match b.cmp(&a) {
            Ordering::Less => 0,
            Ordering::Equal => 3,
            Ordering::Greater => 6,
        };

        let score1 = match b {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        };

        println!("{score0}, {score1}");

        score += score0 + score1;
    }

    println!("score: {score}");
    
}
