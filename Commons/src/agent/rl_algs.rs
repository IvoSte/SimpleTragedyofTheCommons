use crate::agent::actions::{Action, Actions};
use rand::seq::SliceRandom;
use rand::Rng;


pub fn bandit(actions: &Actions) -> &Action {
    epsilon_greedy(actions, 0.1)
}

// TODO should this be inside or outside the brain? Outside seems good, but possibly better inside.
pub fn epsilon_greedy(actions: &Actions, epsilon: f32) -> &Action {
    if rand::thread_rng().gen::<f32>() < epsilon {
        actions.random_action()
    } else {
        actions.max_ev_action()
    }
}
