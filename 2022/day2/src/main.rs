use std::env;
use std::fs;
use std::str::Split;
use std::collections::HashMap;


#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Action {
    ROCK=1,
    PAPER=2,
    SCISSOR=3
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Result {
    WIN=6,
    DRAW=3,
    LOSE=0
}

#[derive(Debug)]
struct Outcome {
    opponent: Action,
    you: Action,
    result: Result
}

fn part1(lines:&Vec<&str>) {
    let mut outcomes:Vec<Outcome>= Vec::new();

    let action_map = HashMap::from([
        ("A", Action::ROCK),
        ("B", Action::PAPER),
        ("C", Action::SCISSOR),
        ("X", Action::ROCK),
        ("Y", Action::PAPER),
        ("Z", Action::SCISSOR)
    ]);
 
    for line in lines {
        if !line.is_empty() {
            let actions:Vec<&str> = line.split(" ").collect();
            let opponent = action_map[actions[0]];
            let you = action_map[actions[1]];
            let outcome = Outcome {
                opponent: opponent,
                you: you,
                result: 
                    if opponent == you {
                        Result::DRAW
                    }
                    else if opponent == Action::ROCK && you == Action::PAPER {
                        Result::WIN
                    }
                    else if opponent == Action::PAPER && you == Action::SCISSOR {
                        Result::WIN
                    }
                    else if opponent == Action::SCISSOR && you == Action::ROCK {
                        Result::WIN
                    }
                    else {
                        Result::LOSE
                    }
            };
            outcomes.push(outcome);
        };
    }

    let mut total_score = 0;

    for outcome in outcomes {
        println!("Outcome: {:?}", outcome);
        let score = outcome.result as i32 + outcome.you as i32;
        println!("Score: {}", score);

        total_score += score;
    }

    println!("Solution first star. Total score: {}", total_score);
}

fn part2(lines:&Vec<&str>) {
    let mut outcomes:Vec<Outcome>= Vec::new();

    let action_map = HashMap::from([
        ("A", Action::ROCK),
        ("B", Action::PAPER),
        ("C", Action::SCISSOR)
    ]);

    let outcome_map = HashMap::from([
        ("X", Result::LOSE),
        ("Y", Result::DRAW),
        ("Z", Result::WIN)
    ]);
 
    for line in lines {
        if !line.is_empty() {
            let actions:Vec<&str> = line.split(" ").collect();
            let opponent = action_map[actions[0]];
            let expected_result = outcome_map[actions[1]].clone();

            let outcome = Outcome {
                opponent: opponent,
                result: expected_result,
                you: 
                    if expected_result == Result::DRAW {
                        opponent.clone()
                    }
                    else if expected_result == Result::LOSE {
                        if opponent == Action::PAPER {
                            Action::ROCK
                        }
                        else if opponent == Action::SCISSOR {
                            Action::PAPER
                        }
                        else {
                            Action::SCISSOR
                        }
                    }
                    else {
                        if opponent == Action::PAPER {
                            Action::SCISSOR
                        }
                        else if opponent == Action::SCISSOR {
                            Action::ROCK
                        }
                        else {
                            Action::PAPER
                        }
                    }
                    
            };
            outcomes.push(outcome);
        };
    }

    let mut total_score = 0;

    for outcome in outcomes {
        println!("Outcome: {:?}", outcome);
        let score = outcome.result as i32 + outcome.you as i32;
        println!("Score: {}", score);

        total_score += score;
    }

    println!("Solution second star. Total score: {}", total_score);

}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines: Vec<&str>= contents.split("\n").collect();
    part1(&lines);
    part2(&lines);

}