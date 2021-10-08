use crate::agent::actions::Actions;
use crate::agent::rl_algs::{bandit, update_bandit};

/// Cognitive component of the agent. All 'cognitive' operations / decision making of actions can be done here
pub struct AgentBrain {
    actions: Actions,
    last_action_idx: Option<usize>,
    last_reward: i32,
}

impl AgentBrain {
    pub fn new(num_actions: i32) -> AgentBrain {
        AgentBrain {
            actions: Actions::new(num_actions),
            last_action_idx: None,
            last_reward: 0,
        }
    }

    pub fn decide_action(&mut self) -> i32 {
        // replace egreedy with rl alg
        let chosen_action = bandit(&mut self.actions);
        // Increment amount this action has been chosen
        chosen_action.increment_chosen(1);
        // remember which one we chose this round
        self.last_action_idx = Some(chosen_action.get_num_resources() as usize);
        // Return the chosen integer of resources
        return chosen_action.get_num_resources();
    }

    pub fn update_ev(&mut self, action_idx: usize) {
        update_bandit(&mut self.actions, action_idx, self.last_reward);
    }

    pub fn set_last_reward(&mut self, value: i32) {
        self.last_reward = value;
    }

    pub fn decrease_last_reward(&mut self, subtract: i32) {
        self.last_reward -= subtract;
    }
}
