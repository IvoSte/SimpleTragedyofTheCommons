use crate::agent::actions::{self, Action, Actions};
use core::panic;
use std::{collections::HashMap, fmt};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub enum AgentType {
    BANDIT,
    QLEARNING,
}

#[derive(Debug, EnumIter)]
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
        agentstate.map_commons(commons_value);
        agentstate.map_score(score_value);
        return agentstate;
    }
    // this is a mess, refactor TODO. should be some global config map dynamically determined using key values/bounds
    pub fn map_commons(&mut self, commons_value: i32) {
        if commons_value < 3 {
            self.commons_state = ResourceState::LOW;
        } else if commons_value < 8 {
            self.commons_state = ResourceState::MEDIUM;
        } else {
            self.commons_state = ResourceState::HIGH;
        }
    }

    pub fn map_score(&mut self, score_value: i32) {
        if score_value < 2 {
            self.score_state = ResourceState::LOW;
        } else if score_value < 7 {
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

    // pub fn update_action(&mut self, key: String, index: usize, value: Action) {
    //     self.state_action_pairs
    //         .get(&key)
    //         .unwrap()
    //         .replace(index, value);
    // }
}
