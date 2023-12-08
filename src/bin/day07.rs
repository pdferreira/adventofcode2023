use std::fs;
use adventofcode2023::{Result, Part};
use adventofcode2023::str::StringOps;

#[derive(Debug)]
enum HandType {
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,
}

struct Hand<'a> {
    cards: &'a str,
    bid: u32
}

fn main() {
    println!("example (part1): {:?}", solve("inputs/day07_example", Part::One));
    println!("input (part1): {:?}", solve("inputs/day07", Part::One));
    println!("example (part2): {:?}", solve("inputs/day07_example", Part::Two));
    println!("input (part2): {:?}", solve("inputs/day07", Part::Two));
}

fn solve(path: &str, part: Part) -> Result<u32> {
    let content = fs::read_to_string(path)?;
    let mut hands: Vec<_> = content
        .lines()
        .map(|l| Hand::parse(&l).unwrap())
        .collect();

    Ok(match part {
        Part::One => {
            hands.sort_by_cached_key(|h| h.strength(/*use_joker_rule*/false));
            hands
                .iter()
                .enumerate()
                .map(|(idx, h)| (idx as u32 + 1) * h.bid)
                .sum()
        },
        
        Part::Two => {
            hands.sort_by_cached_key(|h| h.strength(/*use_joker_rule*/true));
            hands
                .iter()
                .enumerate()
                .map(|(idx, h)| (idx as u32 + 1) * h.bid)
                .sum()
        }
    })
}

impl Hand<'_> {

    fn parse(line: &str) -> Result<Hand> {
        let (cards, bid_str) = line.try_split_once(" ")?;
        return Ok(Hand {
            cards,
            bid: bid_str.parse()?
        })
    }

    fn get_type(&self, use_joker_rule: bool) -> HandType {
        // First sort the cards so we can inspect consecutive groups
        let mut sorted_cards: Vec<char> = self.cards
            .chars()
            .collect();

        sorted_cards.sort();

        // Then go through all, collecting the number of distinct labels
        // and the number of pairs
        let mut num_distinct_labels: u8 = 1;
        let mut curr_label = sorted_cards[0];
        let mut curr_group_size: u8 = 1;
        let mut num_pairs: u8 = 0;
        let mut num_jokers = (curr_label == 'J') as u8; 

        for i in 1 .. sorted_cards.len() {
            if sorted_cards[i] == curr_label {
                curr_group_size += 1;
            } else {
                if curr_group_size == 2 {
                    num_pairs += 1;
                }

                num_distinct_labels += 1;
                curr_group_size = 1;
                curr_label = sorted_cards[i];
            }

            if curr_label == 'J' {
                num_jokers += 1;
            }
        }

        if curr_group_size == 2 {
            num_pairs += 1;
        }

        // If we're playing accounting for jokers, evaluate those cases first
        if use_joker_rule && num_jokers > 0 {
            return self.classify_with_jokers(num_pairs, num_distinct_labels, num_jokers);
        }

        // Based on the number of labels and number of pairs we can now classify in types
        return self.classify_without_jokers(num_pairs, num_distinct_labels);
    }

    fn classify_with_jokers(&self, num_pairs: u8, num_distinct_labels: u8, num_jokers: u8) -> HandType {
        if num_distinct_labels == 5 && num_jokers == 1 {
            // If there's a joker and everything is different, the joker joins any
            HandType::OnePair
        } else if num_pairs == 1 && num_distinct_labels == 4 && num_jokers == 1  {
            // If there's a joker and a pair, the other two are different, so the joker joins the pair
            HandType::ThreeOfAKind
        } else if num_pairs == 1 && num_distinct_labels == 4 && num_jokers == 2 {
            // If there're two jokers and no other pairs, all the others are different so they join any
            HandType::ThreeOfAKind
        } else if num_pairs == 0 && num_distinct_labels == 3 && num_jokers == 1 {
            // If there's one joker and no pairs, it means we got 3 + 1, so the joker joins the 3
            HandType::FourOfAKind
        } else if num_pairs == 0 && num_distinct_labels == 3 && num_jokers == 3 {
            // All the jokers join one of the two remaining single labels
            HandType::FourOfAKind
        } else if num_pairs == 2 && num_distinct_labels == 3 && num_jokers == 1 {
            // If there's one joker, plus two pairs, join either pair 
            HandType::FullHouse
        } else if num_pairs == 2 && num_distinct_labels == 3 && num_jokers == 2 {
            // If there's a pair of jokers, some other pair and a single label, join the pair
            HandType::FourOfAKind
        } else if num_distinct_labels <= 2 {
            // All the jokers join the only other label, if any
            HandType::FiveOfAKind
        } else {
            panic!("Couldn't classify {} in a hand-type (num_distinct_labels={}, num_pairs={}, num_jokers={})", 
                self.cards,
                num_distinct_labels,
                num_pairs,
                num_jokers);
        }
    }

    fn classify_without_jokers(&self, num_pairs: u8, num_distinct_labels: u8) -> HandType {
        if num_distinct_labels == 5 {
            HandType::HighCard
        } else if num_pairs == 1 && num_distinct_labels == 4 {
            HandType::OnePair
        } else if num_pairs == 2 && num_distinct_labels == 3 {
            HandType::TwoPair
        } else if num_pairs == 0 && num_distinct_labels == 3 {
            HandType::ThreeOfAKind
        } else if num_pairs == 1 && num_distinct_labels == 2 {
            HandType::FullHouse
        } else if num_pairs == 0 && num_distinct_labels == 2 {
            HandType::FourOfAKind
        } else if num_distinct_labels == 1 {
            HandType::FiveOfAKind
        } else {
            panic!("Couldn't classify {} in a hand-type (num_distinct_labels={}, num_pairs={})",
                self.cards,
                num_distinct_labels,
                num_pairs)
        }
    }

    fn strength(&self, use_joker_rule: bool) -> String {
        /* The goal is to give something that is sortable while respecting the
           order/weight the cards and the hand type have. So we map the AKQJT
           into the EDCBA (letters > digits in ASCII) range and use the numeric
           representation of the hand type, which is already properly ordered.

           For Part 2 specifically, we map J to 1, since the cards start at 2.
        */
        let cards_as_strengths = self.cards
            .replace('A', "E")
            .replace('K', "D")
            .replace('Q', "C")
            .replace('J', if use_joker_rule { "1" } else { "B" })
            .replace('T', "A");    
        
        let type_n = self.get_type(use_joker_rule) as u8;
        let mut strength_str = type_n.to_string();
        strength_str.push_str(cards_as_strengths.as_str());
        
        return strength_str
    }

}