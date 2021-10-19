use crate::agent::actions::{Actions};
use core::panic;
use std::{collections::HashMap, fmt};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use crate::config::ExperimentConfig;

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
        let config: ExperimentConfig = Default::default();

        let mut agentstate = AgentState {
            commons_state: ResourceState::MEDIUM,
            score_state: ResourceState::MEDIUM,
        };
        agentstate.map_commons(commons_value, config.max_pool_size);
        agentstate.map_score(score_value, config.consumption);
        return agentstate;
    }
    // this is a mess, refactor TODO. should be some global config map dynamically determined using key values/bounds
    pub fn map_commons(&mut self, commons_value: i32, max_commons_value: i32) {
        // resource state of the commons as seen by the agent. Below 30% is low (maybe this needs to be upped)
        // below 70% is medium
        // above 70% is high
        if (commons_value as f32) < (0.3 * max_commons_value as f32) {
            self.commons_state = ResourceState::LOW;
        } else if (commons_value as f32) < (0.7 * max_commons_value as f32) {
            self.commons_state = ResourceState::MEDIUM;
        } else {
            self.commons_state = ResourceState::HIGH;
        }
    }

    pub fn map_score(&mut self, score_value: i32, consume_value: i32) {
        // resource state of the agents score. Have food for x days, you're low, medium or high on resources.
        if score_value <= 2 * consume_value {
            self.score_state = ResourceState::LOW;
        } else if score_value <= 6 * consume_value {
            self.score_state = ResourceState::MEDIUM;
        } else {
            self.score_state = ResourceState::HIGH;
        }
    }
}

pub struct QTable {
    pub state_action_pairs: HashMap<String, Actions>,
}

impl QTable {
    pub fn new(n_actions: i32) -> QTable {
        let mut state_action_pairs: HashMap<String, Actions> = HashMap::new();
        // loop over all state permutations
        for state_1 in ResourceState::iter() {
            for state_2 in ResourceState::iter() {
                // state permutation is the key in the table
                let statekey =
                    String::from(format!("{} {}", state_1.to_string(), state_2.to_string()));
                // init new actions per possible state
                state_action_pairs.insert(statekey, Actions::new(n_actions));
            }
        }
        QTable { state_action_pairs }
    }

    pub fn get(&mut self, key: String) -> &mut Actions {
        match self.state_action_pairs.get_mut(&key) {
            Some(actions) => actions,
            None => panic!("Tried to access state that isn't there"),
        }
    }

    pub fn report(&self) {
        for (key, value) in &self.state_action_pairs {
            println!("state: {:?}", key);
            value.report();
        }
    }

    // pub fn update_action(&mut self, key: String, index: usize, value: Action) {
    //     self.state_action_pairs
    //         .get(&key)
    //         .unwrap()
    //         .replace(index, value);
    // }
}
