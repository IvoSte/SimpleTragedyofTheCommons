//use std::num;
use crate::agent::actions::{Action, Actions};
use crate::agent::rl_algs::{bandit, qlearning, update_bandit, update_qlearning};
use crate::agent::structs::{AgentState, AgentType, QTable};
use crate::config::RLParameters;

/// Cognitive component of the agent. All 'cognitive' operations / decision making of actions can be done here
pub struct AgentBrain {
    actions: Actions,
    q_table: QTable, // possibly should be hidden somewhere, merged with actions so its only visible if its a qlearning agent
    last_reward: i32,
    current_state: Option<AgentState>,
    previous_state: Option<AgentState>,
    behaviour_type: AgentType,
    rlparameters: RLParameters,
}

impl AgentBrain {
    pub fn new(num_actions: i32, agent_type: AgentType) -> AgentBrain {
        AgentBrain {
            actions: Actions::new(num_actions),
            q_table: QTable::new(num_actions),
            last_reward: 0,
            current_state: None,
            previous_state: None,
            behaviour_type: agent_type,
            rlparameters: Default::default(),
        }
    }

    pub fn decide_action(&mut self) -> i32 {
        // replace egreedy with rl alg
        let chosen_action = self.decision_behaviour_interface();
        // Increment amount this action has been chosen
        chosen_action.increment_chosen(1);
        // Return the chosen integer of resources
        return chosen_action.get_num_resources();
    }

    fn decision_behaviour_interface(&mut self) -> &mut Action {
        match self.behaviour_type {
            AgentType::BANDIT => bandit(&mut self.actions, self.rlparameters.epsilon),
            AgentType::QLEARNING => qlearning(
                &mut self.q_table,
                self.current_state.unwrap().to_string().clone(),
                self.rlparameters.epsilon,
            ),
        }
    }

    pub fn update_ev(&mut self, action_idx: usize) {
        self.update_behaviour_interface(action_idx);
    }

    fn update_behaviour_interface(&mut self, action_idx: usize) {
        match self.behaviour_type {
            AgentType::BANDIT => update_bandit(&mut self.actions, action_idx, self.last_reward),
            AgentType::QLEARNING => update_qlearning(
                &mut self.q_table,
                &self.previous_state.unwrap(),
                &self.current_state.unwrap(),
                action_idx,
                self.last_reward,
                self.rlparameters.alpha,
                self.rlparameters.gamma,
            ),
        }
    }

    pub fn set_last_reward(&mut self, value: i32) {
        self.last_reward = value;
    }

    pub fn decrease_last_reward(&mut self, subtract: i32) {
        self.last_reward -= subtract;
    }

    pub fn death_punishment(&mut self) {
        self.decrease_last_reward(self.rlparameters.death_punish);
    }

    pub fn report(&self) {
        match self.behaviour_type {
            AgentType::BANDIT => self.report_action_evs(),
            AgentType::QLEARNING => self.report_q_table()
        }
    }

    pub fn report_action_evs(&self) {
        self.actions.report();
    }

    pub fn report_q_table(&self) {
        self.q_table.report();
    }

    pub fn update_state(&mut self, pool: i32, score: i32) {
        self.previous_state = self.current_state.clone();
        // better encapsulate this, agent brain should not know the size of the pool
        self.current_state = Some(AgentState::from_values(pool, score));
    }
}
