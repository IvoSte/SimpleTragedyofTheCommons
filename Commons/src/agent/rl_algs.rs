use crate::agent::actions::{Action, Actions};
use crate::agent::structs::{AgentState, QTable};
use rand::Rng;



pub fn qlearning(qtable: &mut QTable, state: String, epsilon: f32) -> &mut Action {
    epsilon_greedy(qtable.get(state), epsilon)
}

pub fn update_qlearning(qtable: &mut QTable, old_state: &AgentState, new_state: &AgentState, 
                        action_idx: usize, reward: i32, alpha: f32, gamma: f32) {
    // from value
    let old_ev: f32 = qtable.get(old_state.to_string())[action_idx].get_expected_value();
    // off-policy best new action
    let max_next_ev: f32 = qtable.get(new_state.to_string()).max_ev_action().get_expected_value();
    // calculate new ev
    let new_ev: f32 = old_ev + (alpha * (reward as f32 + (gamma * max_next_ev) - old_ev));
    // update ev
    qtable.get(old_state.to_string())[action_idx].set_expected_value(new_ev);
}

pub fn bandit(actions: &mut Actions, epsilon: f32) -> &mut Action {
    epsilon_greedy(actions, epsilon)
}

pub fn update_bandit(actions: &mut Actions, action_idx: usize, reward: i32) {
    // New estimate = old estimate + stepsize(target - old estimate)
    let stepsize = 0.1;
    let old_estimate = actions[action_idx].get_expected_value();
    let new_estimate = old_estimate + (stepsize * (reward as f32 - old_estimate));

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