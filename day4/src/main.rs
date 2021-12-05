#[derive(Clone)]
struct BingoCard {
    numbers: Vec<Vec<usize>>,
    marked: Vec<Vec<bool>>,
    already_won: bool,
}

impl BingoCard {
    fn check_number(&mut self, number: usize) {
        'outer: for i in 0..self.numbers.len() {
            for j in 0..self.numbers.len() {
                if self.numbers[i][j] == number {
                    self.marked[i][j] = true;
                    break 'outer;
                }
            }
        }
    }

    fn won(&mut self) -> bool {
        let mut rowcount = [0; 5];
        let mut colcount = [0; 5];

        for (i, _) in self.marked.iter().enumerate() {
            for (j, _) in self.marked.iter().enumerate() {
                if self.marked[i][j] {
                    rowcount[i] += 1;
                    colcount[j] += 1;
                }

                if ((rowcount[i] == 5) | (colcount[j] == 5)) & !self.already_won {
                    self.already_won = true;
                    return true;
                }
            }
        }

        false
    }

    fn create(input: &str) -> Self {
        let numbers: Vec<Vec<usize>> = input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|num| num.parse().unwrap())
                    .collect::<Vec<usize>>()
            })
            .collect();
        let marked = vec![Vec::from([false; 5]); 5];

        BingoCard {
            numbers,
            marked,
            already_won: false,
        }
    }

    fn score(&self) -> usize {
        let mut res = 0;

        for i in 0..self.marked.len() {
            for j in 0..self.marked.len() {
                if !self.marked[i][j] {
                    res += self.numbers[i][j];
                }
            }
        }

        res
    }
}

fn find_first_winner(mut cards: Vec<BingoCard>, instructions: &[usize]) -> (BingoCard, usize) {
    for instruction in instructions {
        for card in cards.iter_mut() {
            card.check_number(*instruction);
            if card.won() {
                return (card.clone(), *instruction);
            }
        }
    }
    unreachable!();
}

fn find_last_winner(mut cards: Vec<BingoCard>, instructions: &[usize]) -> (BingoCard, usize) {
    let n_cards = cards.len();
    let mut n_won = 0;

    for instruction in instructions {
        for card in cards.iter_mut() {
            card.check_number(*instruction);
            if card.won() {
                n_won += 1
            };
            if n_won == n_cards {
                return (card.clone(), *instruction);
            }
        }
    }
    unreachable!();
}

fn main() {
    let mut input = include_str!("../input.txt").split("\n\n");

    let instructions: Vec<usize> = input
        .next()
        .unwrap()
        .split(',')
        .map(|numstring| numstring.parse::<usize>().unwrap())
        .collect();
    let cards: Vec<BingoCard> = input
        .map(|cardstring| BingoCard::create(cardstring))
        .collect();

    let (winning_board, winning_call) = find_first_winner(cards.clone(), &instructions);
    let (last_winning_board, last_winning_call) = find_last_winner(cards, &instructions);

    println!("Part 1: {}", winning_board.score() * winning_call);
    println!("Part 2: {}", last_winning_board.score() * last_winning_call);
}
