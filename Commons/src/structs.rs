mod structs {
    use crate::agent::actions::Actions;
    use std::{collections::HashMap, fmt};
    use strum::IntoEnumIterator; // 0.17.1
    use strum_macros::EnumIter; // 0.17.1

    #[derive(Debug, EnumIter)]
    enum ResourceState {
        LOW,
        MEDIUM,
        HIGH,
    }
    
    impl fmt::Display for ResourceState {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    struct AgentState {
        commons_state: ResourceState,
        score_state: ResourceState,
    }

    impl AgentState {
        // Using state as key for the Q table, as string
        pub fn to_string(&self) -> String {
            String::from(format!("{} {}", &self.commons_state.to_string(), &self.score_state.to_string()))
        }
        // Maybe wierd way to initialize?
        pub fn from_values(commons_value: i32, score_value: i32) -> AgentState {
            let agentstate = AgentState {commons_state: ResourceState::MEDIUM, score_state: ResourceState::MEDIUM};
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

    struct StateActions {
        state: AgentState,
        actions: Actions,
    }

    struct QTable {
        state_action_pairs: HashMap<String, StateActions>,
    }

}