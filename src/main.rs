// use std::collections::HashMap;
use clap::Parser;
use monty_hall::Outcome::{Lose, Win};
use monty_hall::{stats, Choice, Choice::Stay, Choice::Switch};
use rand::distributions::{Distribution, Uniform};
use rand::seq::SliceRandom;
use rand::Rng;
use stats::print_summary;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value_t = 100000)]
    trials: usize,
    #[arg(short, long, default_value_t = 3)]
    doors: usize,
    #[arg(short, long, default_value_t = 1)]
    opens: usize,
    #[arg(short, long, default_value_t = 0)]
    always_switch: u8,
}

fn main() {
    let args = Args::parse();
    let n_trials = args.trials;
    let n_doors = args.doors;
    let n_opens = args.opens;
    let always_switch = args.always_switch == 1;
    let mut win_stay = 0;
    let mut win_switch = 0;
    let mut lose_stay = 0;
    let mut lose_switch = 0;
    assert!(n_opens < n_doors - 1, "n_opens must be less than n_doors-1");
    let choices = [&Stay, &Switch];
    // I like the HashMap approach, but it has a noticeable effect on performance
    // let mut stats: HashMap<(&Choice, &Outcome), i32> = HashMap::from([
    //     ((&Stay, &Win), 0),
    //     ((&Stay, &Lose), 0),
    //     ((&Stay, &Win), 0),
    //     ((&Stay, &Lose), 0)]);
    let mut rng = rand::thread_rng();
    let i_last = n_doors - n_opens - 1;
    let rand_car = Uniform::from(0..n_doors);

    for _ in tqdm::tqdm(0..n_trials) {
        // for _ in 0..n_trials {
        let mut outcome = &Lose;
        let final_choice: usize;
        let car_door = rand_car.sample(&mut rng);
        let first_choice = 0; //rng.gen_range(0..n_doors);
        let decision: &Choice;

        if always_switch {
            decision = &Switch;
        } else {
            decision = choices.choose(&mut rng).unwrap();
        }

        if decision == &Stay {
            if first_choice == car_door {
                outcome = &Win
            }
        } else {
            if first_choice != car_door {
                if car_door <= i_last {
                    final_choice = rng.gen_range(0..i_last) + 1;
                } else {
                    let mut doors: Vec<usize> = (1..i_last + 1).collect();
                    doors[i_last - 1] = car_door;
                    final_choice = *doors.choose(&mut rng).unwrap();
                }
                if final_choice == car_door {
                    outcome = &Win
                }
            }
        }
        if decision == &Stay {
            if outcome == &Win {
                win_stay += 1;
            } else {
                lose_stay += 1
            }
        } else {
            if outcome == &Win {
                win_switch += 1;
            } else {
                lose_switch += 1;
            }
        }
        // *stats.entry((decision, outcome)).or_default() += 1;
    }

    // print_summary(n_trials, n_doors, n_opens, &stats, Some(true))
    print_summary(
        n_trials,
        n_doors,
        n_opens,
        win_stay,
        win_switch,
        lose_stay,
        lose_switch,
        Some(true),
    )
}
