use std::fs;

const OPPONENT_ROCK: &str = "A";
const OPPONENT_PAPER: &str = "B";
const OPPONENT_SCISSORS: &str = "C";

const SELF_ROCK: &str = "X";
const SELF_PAPER: &str = "Y";
const SELF_SCISSORS: &str = "Z";

const SCORE_ROCK: u32 = 1;
const SCORE_PAPER: u32 = 2;
const SCORE_SCISSORS: u32 = 3;

fn read_input() -> Vec<String> {
    let file_path = "./src/input.txt";

    let contents = fs::read_to_string(file_path)
        .expect(" Should have been able to read the file");

    return Vec::from_iter(contents.split("\n").map(String::from));
}


fn get_score_for_move(player_move: &str) -> Result<u32, &str> {
    match player_move {
        SELF_ROCK => Ok(SCORE_ROCK),
        SELF_PAPER => Ok(SCORE_PAPER),
        SELF_SCISSORS => Ok(SCORE_SCISSORS),
        _ => Err("Illegal move"),
    }
}

fn get_round_score(player_move: &str, opponent_move: &str) -> Result<u32, &'static str> {
    match player_move {
        SELF_ROCK => {
            return match opponent_move {
                OPPONENT_PAPER => Ok(0),
                OPPONENT_ROCK => Ok(3),
                OPPONENT_SCISSORS => Ok(6),

                _ => Err("Illegal Opponent move"),
            };
        }

        SELF_PAPER => {
            return match opponent_move {
                OPPONENT_SCISSORS => Ok(0),
                OPPONENT_ROCK => Ok(6),
                OPPONENT_PAPER => Ok(3),

                _ => Err("Illegal Opponent move"),
            };
        }

        SELF_SCISSORS => {
            return match opponent_move {
                OPPONENT_ROCK => Ok(0),
                OPPONENT_SCISSORS => Ok(3),
                OPPONENT_PAPER => Ok(6),

                _ => Err("Illegal Opponent move"),
            };
        }

        _ => Err("Illegal player move"),
    }
}

fn first_part() {
    let input = read_input();

    let mut total_score = 0;

    for item in input {
        let strategy = Vec::from_iter(item.split(" "));

        let opponent_move = strategy.first().unwrap();
        let player_move = strategy.last().unwrap();

        if opponent_move.is_empty() || player_move.is_empty() {
            continue;
        }

        total_score += get_score_for_move(player_move).unwrap();
        total_score += get_round_score(player_move, opponent_move).unwrap();
    }

    println!("total score (part 1): {total_score}");
}

const SCENARIO_LOSE: &str = "X";
const SCENARIO_DRAW: &str = "Y";
const SCENARIO_WIN: &str = "Z";

fn second_part() {
    let input = read_input();

    let mut total_score: u32 = 0;

    for item in input {
        let strategy = Vec::from_iter(item.split(" "));

        let opponent_move = strategy.first().unwrap();
        let game_scenario = strategy.last().unwrap();

        // handle last line empty
        if opponent_move.is_empty() || game_scenario.is_empty() {
            continue;
        }

        total_score +=get_player_move_score_for_scenario(opponent_move, game_scenario).expect("Could not get score for move");
        total_score += get_scenario_score(game_scenario).expect("Could not get score game scenario");
    }


    println!("total score (part 2): {total_score}")
}

fn main() {
    first_part();

    second_part();
}

fn get_player_move_score_for_scenario(opponent_move: &str, game_scenario: &str) -> Result<u32, &'static str> {
    match game_scenario {
        SCENARIO_LOSE => {
            match opponent_move {
                OPPONENT_ROCK => Ok(SCORE_SCISSORS),
                OPPONENT_PAPER => Ok(SCORE_ROCK),
                OPPONENT_SCISSORS => Ok(SCORE_PAPER),

                _ => Err("Illegal opponent move")
            }
        },

        SCENARIO_DRAW => {
            match opponent_move {
                OPPONENT_ROCK => Ok(SCORE_ROCK),
                OPPONENT_PAPER => Ok(SCORE_PAPER),
                OPPONENT_SCISSORS => Ok(SCORE_SCISSORS),

                _ => Err("Illegal opponent move")
            }
        },

        SCENARIO_WIN => {
            match opponent_move {
                OPPONENT_ROCK => Ok(SCORE_PAPER),
                OPPONENT_PAPER => Ok(SCORE_SCISSORS),
                OPPONENT_SCISSORS => Ok(SCORE_ROCK),

                _ => Err("Illegal opponent move")
            }
        },

        _ => Err("Illegal game scenario")
    }
}


fn get_scenario_score(scenario: &str) -> Result<u32, &'static str> {
   match scenario {
        SCENARIO_WIN => Ok(6),
        SCENARIO_DRAW => Ok(3),
        SCENARIO_LOSE => Ok(0),

       _ => Err("Illegal game scenario")
   }
}