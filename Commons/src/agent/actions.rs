use float_ord::FloatOrd;
use rand::prelude::*;
use rand::seq::SliceRandom;
use std::ops::{Index, IndexMut};

use crate::CONFIG;

/// An action / action availible to an agent, tracking its own statistics
///
#[derive(Clone, Copy)]
pub struct Action {
    /// Number of resources to take
    num_resources: i32,
    /// Expected value of that action
    expected_value: f32,
    /// Number of times the action is chosen
    times_chosen: i32,
}

impl Action {
    pub fn new(num_resources: i32, expected_value: f32, times_chosen: i32) -> Action {
        Action {
            num_resources,
            expected_value,
            times_chosen,
        }
    }

    // Instead of updating an action, create a new one.
    pub fn new_from_with_ev(action: &Action, expected_value: f32) -> Action {
        Action {
            num_resources: action.num_resources,
            expected_value: expected_value,
            times_chosen: action.times_chosen,
        }
    }

    pub fn increment_chosen(&mut self, n: i32) {
        self.times_chosen += n;
    }

    pub fn get_num_resources(&self) -> i32 {
        return self.num_resources;
    }

    pub fn get_expected_value(&self) -> f32 {
        return self.expected_value;
    }

    pub fn set_expected_value(&mut self, value: f32) {
        self.expected_value = value;
    }

    pub fn report(&self) {
        println!(
            "NR: {} EV: {} NC: {}",
            self.num_resources, self.expected_value, self.times_chosen
        );
    }
}

pub struct Actions {
    // Consider changing implementation to HashMap or other data structure
    actions: Vec<Action>,
}

/// Container for all availible actions. All 'non-cognitive' operations on action selection can be done here
impl Actions {
    pub fn new(num_actions: i32) -> Actions {
        Actions {
            actions: Self::init_actions(num_actions),
        }
    }

    fn init_actions(num_actions: i32) -> Vec<Action> {
        let mut actions: Vec<Action> = Vec::with_capacity(num_actions as usize);
        for i in 0..num_actions {
            // initialization strategy 
            match CONFIG.rl_params.init_mode {
                // 0: 0 with fuzzing [0, 0.01]
                0 => actions.push(Action::new(i, rand::thread_rng().gen::<f32>() * 0.01, 0)),
                // 1: optimistic inital values [5 + fuzzing]
                1 => actions.push(Action::new(i, 5.0 + (rand::thread_rng().gen::<f32>() * 0.01), 0)),
                // 2: 0.0 --> without tiebreaker will slide from max to min index 
                2 => actions.push(Action::new(i, 0.0, 0)),
                _ => actions.push(Action::new(i, rand::thread_rng().gen::<f32>() * 0.01, 0)),
            }
        }
        return actions;
    }

    pub fn max_ev_action(&mut self) -> &mut Action {
        //TODO needs tiebreaker contingency, for initial selection
        return self
            .actions
            .iter_mut()
            .max_by_key(|action| FloatOrd(action.expected_value))
            .unwrap();
    }

    // this doesn't need to be mutable. We can get this action as non-mutable TODO
    // and then in the higher layer get the action index
    // and use that index to get a mutable version of that action to the higher layer
    // by having a get_mut_action(&self mut, action_idx) -> &mut action function here low level.
    pub fn random_action(&mut self) -> &mut Action {
        return self.actions.choose_mut(&mut rand::thread_rng()).unwrap();
    }

    pub fn report(&self) {
        for action in &self.actions {
            action.report();
        }
    }
}

impl Index<usize> for Actions {
    type Output = Action;
    fn index(&self, i: usize) -> &Action {
        &self.actions[i]
    }
}

impl IndexMut<usize> for Actions {
    fn index_mut(&mut self, i: usize) -> &mut Action {
        &mut self.actions[i]
    }
}
