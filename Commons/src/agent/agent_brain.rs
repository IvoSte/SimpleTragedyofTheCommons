use crate::agent::actions::{Action, Actions};
use crate::agent::rl_algs::epsilon_greedy;

/// Cognitive component of the agent. All 'cognitive' operations / decision making of actions can be done here
pub struct AgentBrain {
    actions: Actions,
    last_action_idx: usize,
    last_reward: i32,
}

impl AgentBrain {
    pub fn new(num_actions: Option<i32>) -> AgentBrain {
        AgentBrain {
            actions: Actions::new(num_actions),
            // TODO find better way than declaring dummy defaults
            last_action_idx: 0,
            last_reward: 0,
        }
    }

    pub fn decide_action(&mut self) -> i32 {
        // replace egreedy with rl alg
        let chosen_action = epsilon_greedy(&self.actions, 0.1);
        // Increment amount this action has been chosen
        chosen_action.increment_chosen(1);
        // remember which one we chose this round
        self.last_action_idx = chosen_action.get_num_resources() as usize;
        // Return the chosen integer of resources
        return chosen_action.get_num_resources();
    }

    pub fn update_ev(&mut self, action_idx: usize) {
        // TODO ended here. Update EV of the chosen action with the new value.
        // Average received value with amount of times action is chosen.

        // Dummy function updating the value only to actual value, for testing purposes TODO remove
        // New estimate = old estimate * stepsize(old estimate - new value)
        let stepsize = 0.1;
        let old_estimate = self.actions[action_idx].get_expected_value();
        let new_estimate: f32 = old_estimate * (stepsize * (old_estimate - self.last_reward as f32));

        self.actions[action_idx].set_expected_value(new_estimate);
    }
}