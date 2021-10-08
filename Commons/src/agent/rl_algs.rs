use crate::agent::actions::{Action, Actions};
use rand::Rng;


//pub mod rl_mod {
pub fn q_learning() -> &mut Action {

}

pub fn bandit(actions: &mut Actions) -> &mut Action {
    epsilon_greedy(actions, 0.1)
}

pub fn update_bandit(actions: Actions, action_idx: usize, reward: i32) -> &mut Action{
    // New estimate = old estimate + stepsize(old estimate - new value)
    let stepsize = 0.1;
    let old_estimate = actions[action_idx].get_expected_value();
    let new_estimate = old_estimate + (stepsize * (old_estimate - reward as f32));

    actions[action_idx].set_expected_value(new_estimate);

}

// TODO should this be inside or outside the brain? Outside seems good, but possibly better inside.
pub fn epsilon_greedy(actions: &mut Actions, epsilon: f32) -> &mut Action {
    if rand::thread_rng().gen::<f32>() < epsilon {
        actions.random_action()
    } else {
        actions.max_ev_action()
    }
}

//}