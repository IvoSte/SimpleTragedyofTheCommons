use crate::agent::actions::{Action, Actions};
use rand::Rng;


//pub mod rl_mod {
//pub fn q_learning() -> &mut Action {

//}

pub fn init_qtable(n_actions: i32) -> &mut QTable {
    let mut q_table: HashMap<String, Actions> = HashMap::new();
    // loop over all state permutations
    for state_1 in ResourceState::iter() {
        for state_2 in ResourceState::iter() {
            // state permutation is the key in the table
            let statekey = String::from(format!("{} {}", state_1.to_string(), state_2.to_string()));
            // init new actions per possible state
            q_table.insert(statekey, Actions::new(n_actions));
        }
    }
}

pub fn qlearning(qtable: &mut QTable, state: &AgentState) -> &mut Action {
    epsilon_greedy(qtable.get(&state.to_string()), 0.1)
}

pub fn update_qlearning(qtable: &mut QTable, old_state: &AgentState, new_state: &AgentState, 
                        action_idx: usize, reward: i32, alpha: f32, gamma: f32) {
    // from value
    let old_ev: f32 = qtable.get(old_state.to_string())[action_idx].get_expected_value();
    // off-policy best new action
    let max_next_ev: f32 = qtable.get(new_state.to_string()).max_ev_action().get_expected_value();
    // update ev
    let new_ev: f32 = old_ev + (alpha * (reward + (gamma * max_next_ev) - old_ev));
    qtable.get(old_state.to_string())[action_idx].set_expected_value(new_ev);
}

pub fn bandit(actions: &mut Actions) -> &mut Action {
    epsilon_greedy(actions, 0.0)
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