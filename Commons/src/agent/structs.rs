use core::panic;

use std::{collections::HashMap, fmt};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::agent::actions::{Action, Actions};
use crate::config::{ExperimentConfig, StateThresholds};
use crate::{Agent, CONFIG};

pub enum AgentType {
    BANDIT,
    QLEARNING,
}

#[derive(Debug, EnumIter, Clone, Copy)]
pub enum ResourceState {
    LOW,
    MEDIUM,
    HIGH,
}

impl fmt::Display for ResourceState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Copy)]
pub struct AgentState {
    commons_state: ResourceState,
    score_state: ResourceState,
}

impl AgentState {
    // Using state as key for the Q table, as string
    pub fn to_string(&self) -> String {
        String::from(format!(
            "{} {}",
            &self.commons_state.to_string(),
            &self.score_state.to_string()
        ))
    }
    // Maybe wierd way to initialize?
    pub fn from_values(commons_value: i32, score_value: i32) -> AgentState {

        let mut agentstate = AgentState {
            commons_state: ResourceState::MEDIUM,
            score_state: ResourceState::MEDIUM,
        };
        agentstate.map_commons(commons_value, CONFIG.experiment.max_pool_size);
        agentstate.map_score(score_value, CONFIG.experiment.consumption);
        return agentstate;
    }
    // this is a mess, refactor TODO. should be some global config map dynamically determined using key values/bounds
    pub fn map_commons(&mut self, commons_value: i32, max_commons_value: i32) {
        // resource state of the commons as seen by the agent. Below 30% is low (maybe this needs to be upped)
        // below 70% is medium
        // above 70% is high
        let cfg: StateThresholds = Default::default(); // TODO: Allow passing non-default state thresholds

        if (commons_value as f32) < (cfg.commons_low * max_commons_value as f32) {
            self.commons_state = ResourceState::LOW;
        } else if (commons_value as f32) < (cfg.commons_med * max_commons_value as f32) {
            self.commons_state = ResourceState::MEDIUM;
        } else {
            self.commons_state = ResourceState::HIGH;
        }
    }

    pub fn map_score(&mut self, score_value: i32, consume_value: i32) {
        // resource state of the agents score. Have food for x days, you're low, medium or high on resources.
        let cfg: StateThresholds = Default::default(); // TODO: Allow passing non-default state thresholds

        if score_value <= cfg.score_low * consume_value {
            self.score_state = ResourceState::LOW;
        } else if score_value <= cfg.score_med * consume_value {
            self.score_state = ResourceState::MEDIUM;
        } else {
            self.score_state = ResourceState::HIGH;
        }
    }

    pub fn state_keys() -> Vec<String> {
        let mut vec: Vec<String> = Vec::new();
        // loop over all state permutations
        for state_1 in ResourceState::iter() {
            for state_2 in ResourceState::iter() {
                // state permutation is the key in the table
                let state_key =
                    String::from(format!("{} {}", state_1.to_string(), state_2.to_string()));
                vec.push(state_key);
            }
        }
        vec
    }
}

pub struct QTable {
    pub n_actions: i32,
    pub state_action_pairs: HashMap<String, Actions>,
}

impl QTable {
    pub fn new(n_actions: i32) -> QTable {
        let mut state_action_pairs: HashMap<String, Actions> = HashMap::new();
        let state_keys = AgentState::state_keys();
        for state_key in state_keys {
            state_action_pairs.insert(state_key, Actions::new(n_actions));
        }
        QTable {
            n_actions,
            state_action_pairs,
        }
    }

    pub fn get_mut(&mut self, key: &String) -> &mut Actions {
        match self.state_action_pairs.get_mut(key) {
            Some(actions) => actions,
            None => panic!("Tried to access state that isn't there"),
        }
    }

    /// Get action in a specific state
    pub fn get_action_mut(&mut self, key: &String, action_idx: usize) -> &mut Action {
        &mut self.get_mut(key)[action_idx]
    }

    pub fn get(&self, key: &String) -> &Actions {
        match self.state_action_pairs.get(key) {
            Some(actions) => actions,
            None => panic!("Tried to access state that isn't there"),
        }
    }

    pub fn get_action(&self, key: &String, action_idx: usize) -> &Action {
        &self.get(key)[action_idx]
    }

    pub fn report(&self) {
        for (key, value) in &self.state_action_pairs {
            println!("state: {:?}", key);
            value.report();
        }
    }

    pub fn average_q_table(agents: &Vec<Agent>) -> QTable {
        Self::average_from_vector(&agents.iter().map(|agent| &agent.brain.q_table).collect())
    }

    pub fn average_from_vector(q_tables: &Vec<&QTable>) -> QTable {
        let n_actions = q_tables[0].n_actions;
        let mut avg_q_table = QTable::new(n_actions);
        let state_keys = &AgentState::state_keys();

        // Sum the EVs from all QTables
        for q_table in q_tables {
            for state in state_keys {
                for action_idx in 0..n_actions as usize {
                    let old_value = avg_q_table
                        .get_action(state, action_idx)
                        .get_expected_value();
                    let new_value =
                        old_value + q_table.get_action(state, action_idx).get_expected_value();
                    avg_q_table
                        .get_action_mut(state, action_idx)
                        .set_expected_value(new_value);
                }
            }
        }

        // Divide values by number of QTables to compute average EVs
        let n_tables = q_tables.len() as f32;
        for state in state_keys {
            for action_idx in 0..n_actions as usize {
                let old_value = avg_q_table
                    .get_action(state, action_idx)
                    .get_expected_value();
                avg_q_table
                    .get_action_mut(state, action_idx)
                    .set_expected_value(old_value / n_tables);
            }
        }

        avg_q_table
    }
}
