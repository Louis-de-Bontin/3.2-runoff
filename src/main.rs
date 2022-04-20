// use std::{io, env, process};
use std::{self, io, process, env};
use std::io::Write;
use std::cmp::Reverse;

const MAX_VOTERS:usize = 100;
const MAX_CANDIDATES:usize =9;

#[derive(Debug)]
pub struct Candidate {
    name: String,
    votes: usize,
    eliminated: bool
}

fn main() {
    let voter_count: usize;
    let candidate_count: usize;
    let mut preferences:Vec<Vec<String>> = vec![vec![String::from(""); MAX_CANDIDATES]; MAX_VOTERS];
    let mut candidates: Vec<Candidate> = vec![];
    let args:Vec<String> = get_args("Usage: cargo run cadidate1 candicate2....(min 2, max 9)", 2, MAX_CANDIDATES);
    
    candidate_count = args.len() as usize;
    if candidate_count > MAX_CANDIDATES as usize {
        println!("The maximum number of candidates is {}.", MAX_CANDIDATES);
        process::exit(2);
    }

    for i in 0..candidate_count {
        let new_candidate = Candidate {
            name: args[i as usize].clone(),
            votes: 0,
            eliminated: false
        };
        candidates.push(new_candidate);
    }

    voter_count = string_to_int(&input("Number of voters : "));
    if voter_count > MAX_VOTERS {
        println!("The maximum number of voters is {}.", MAX_VOTERS);
        process::exit(3);
    }

    
    for i in 0..voter_count {
        for j in 0..candidate_count {

            // Get vote frome user
            print!("Rank {}: ", j + 1);
            io::stdout().flush().unwrap();
            let mut name = String::from("");
            io::stdin()
                .read_line(&mut name)
                .expect("Failed to read input.");
            name = name.replace("\n", "");

            if !vote(i, j, name, &mut preferences, &candidates) {
                println!("Invalid vote.");
                process::exit(4);
            }
        }
        println!("\n");
    }    

    loop {

        tabulate(&preferences, &mut candidates);
        let won: bool = print_winner(&mut candidates, voter_count);
        if won {
            break;
        }

        let min: usize = find_min(&mut candidates);
        let tie: bool = is_tie(min, &mut candidates);
        if tie {
            println!("The election is tie between :");
            for i in 0..candidate_count {
                if !candidates[i].eliminated {
                    println!("{}", candidates[i].name);
                }
            }
            break;
        }
        eliminate(min, &mut candidates, &mut preferences, voter_count, candidate_count);

        for i in 0..candidate_count {
            candidates[i].votes = 0;
        }
    }
    process::exit(0);
}


fn vote(i: usize, j: usize, name: String, preferences: &mut Vec<Vec<String>>, candidates: &Vec<Candidate>) -> bool {
    // Update the voters choices
    for candidate in candidates {
        if name == candidate.name && !preferences[i].contains(&name) {
            preferences[i][j] = name.clone();
            return true
        }
    }
    false
}

fn tabulate(preferences: &Vec<Vec<String>>, candidates: &mut Vec<Candidate>) {
    // Analyse the votes, and update the votes of each candidate
    // One 1st choice = 1 vote
    for candidate in candidates {
        for bulletin in preferences {
            if candidate.name == bulletin[0] {
                candidate.votes += 1;
            }
        }
    }
}

fn print_winner(candidates: &mut Vec<Candidate>, voter_count: usize) -> bool {
    // print the winner if there is one. Exit if there is
    // Otherwise returns false
    candidates.sort_by_key(|c| Reverse(c.votes));

    for candidate in candidates {
        if candidate.eliminated == false && (candidate.votes as f32 / voter_count as f32) > 0.5 as f32 {
            println!("The winner is : {}.", candidate.name);
            return true
        }
    }
    false
}

fn find_min(candidates: &mut Vec<Candidate>) -> usize {
    // Find the candidate the less liked among remaining candidate (less 1st choices)
    for candidate in candidates.iter().rev() {
        if candidate.eliminated == false {
            return candidate.votes
        }
    }
    println!("Something went wrong...");
    process::exit(5);
}

fn is_tie(min: usize, candidates: &mut Vec<Candidate>) -> bool {
    // Check if there is a tie between the remaining candidates
    for candidate in candidates.iter() {
        if candidate.votes != min && candidate.eliminated == false {
            return false
        }
    }
    true
}

fn eliminate(min: usize, candidates: &mut Vec<Candidate>, preferences: &mut Vec<Vec<String>>, nb_voters: usize, nb_candidates: usize) {
    // Is called if tie is false, and eliminate the candidate
    // with fewest number of votes
    for candidate in candidates {
        if candidate.votes == min {
            candidate.eliminated = true;

            for i in 0..nb_voters {
                for j in 0..nb_candidates {
                    if preferences[i][j] == candidate.name {
                        preferences[i].remove(j);
                    }
                }
            }
        }
    }
}












fn get_args(info: &str, min: usize, max: usize) -> Vec<String> {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    let argc = args.len();
    if argc < min || argc > max {
        println!("{}", info);
        process::exit(1); 
    }
    args
}

fn input(message: &str) -> String {
    // This function displays an informative string to the user, and return his input
    println!("{}", message);
    let mut user_input = String::from("");
    io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read input.");
    user_input
}

fn string_to_int(nb_str: &str) -> usize {
    // Transform a string into an unsigned int, or exit if failed.
    let nb: usize = match nb_str.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please provide a correct number.");
            process::exit(1);
        },
    };
    nb
}
