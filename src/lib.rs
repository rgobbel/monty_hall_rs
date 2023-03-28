#[derive(Eq, Hash, PartialEq, Debug)]
pub enum Choice {
    Switch,
    Stay,
}
#[derive(Eq, Hash, PartialEq, Debug)]
pub enum Outcome {
    Win,
    Lose,
}

pub mod stats {

    // Print the results of a simulation run. do_probs controls printout of
    // theoretical probabilities, as well as accumulated simulation results.

    // use std::collections::HashMap;
    // use crate::{Choice};
    // use crate::Choice::{Stay, Switch};
    // use crate::Outcome::{Win, Lose};

    pub fn print_summary(
        n_trials: usize,
        n_doors: usize,
        n_opens: usize,
        win_stay: i32,
        win_switch: i32,
        lose_stay: i32,
        lose_switch: i32,
        do_probs: Option<bool>,
    ) {
        use num_format::{Locale, ToFormattedString};
        // pub fn print_summary(n_trials: usize, n_doors: usize, n_opens: usize, stats: &HashMap<(&Choice, &Outcome), i32>, do_probs: Option<bool>) {
        // let win_stay = *stats.get(&(&Stay, &Win)).unwrap_or(&0) as f32;
        // let win_switch = *stats.get(&(&Switch, &Win)).unwrap_or(&0) as f32;
        // let lose_stay = *stats.get(&(&Stay, &Lose)).unwrap_or(&0) as f32;
        // let lose_switch = *stats.get(&(&Switch, &Lose)).unwrap_or(&0) as f32;
        let win_stay = win_stay as f32;
        let win_switch = win_switch as f32;
        let lose_stay = lose_stay as f32;
        let lose_switch = lose_switch as f32;
        let switch_runs = win_switch + lose_switch;
        let stay_runs = win_stay + lose_stay;

        let s_t = n_trials.to_formatted_string(&Locale::en);
        let s_d = n_doors.to_formatted_string(&Locale::en);
        let s_n = n_opens.to_formatted_string(&Locale::en);
        println!("{s_t} trials, {s_d} doors, {s_n} opens");
        println!("wins = switch:{win_switch}, stay:{win_stay}");
        println!("losses = switch:{lose_switch}, stay:{lose_stay}");
        println!("switch: win:{win_switch}, lose:{lose_switch}");
        println!("stay: win:{win_stay}, lose:{lose_stay}");
        let stay_ratio = win_stay / stay_runs;
        let switch_ratio = win_switch / switch_runs;
        println!("stay win/loss ratio = {}", stay_ratio);
        println!("switch win/loss ratio = {}", switch_ratio);
        let advantage = switch_ratio / stay_ratio;
        println!("switch/stay advantage = {}", advantage);
        if do_probs != Some(false) {
            let n_f: f32 = n_doors as f32;
            let o_f: f32 = n_opens as f32;
            let pre_prob = 1.0 / n_f;
            let post_prob = ((n_f - 1.0) / (n_f - o_f - 1.0)) / n_f;
            println!("pre-reveal (stay) chance correct =  {}", pre_prob);
            println!("post-reveal switch chance correct = {}", post_prob);
            let mut sim_err: f32 = ((switch_ratio - post_prob) + (stay_ratio - pre_prob)) / 2.0;
            if sim_err < 0.0 {
                sim_err = -sim_err;
            }
            println!("error = {}", sim_err);
        }
    }
}
