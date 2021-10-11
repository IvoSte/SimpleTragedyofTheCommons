pub mod actions;
pub mod agent_brain;
pub mod rl_algs;
pub mod structs;

use self::agent_brain::AgentBrain;
use 
/// The state of an agent, either alive or dead
#[derive(PartialEq, Eq)]
pub enum AgentVitalState {
    ALIVE,
    DEAD,
}


/// An agent in the ToTC simulation
pub struct Agent {
    pub id: i32,
    score: i32,
    pub days_lived: i32,
    vitals: AgentVitalState,
    planned_action: i32,
    brain: AgentBrain,
}

impl Agent {
    /// Returns an Agent with the given id and score
    ///
    /// # Arguments
    ///
    /// * `id`    - An unsigned int to uniquely identify this agent
    /// * `score` - An integer that represents the agent's starting score
    pub fn new(id: i32, score: Option<i32>, n_actions: i32) -> Agent {
        Agent {
            id,
            // Default score value 0
            score: score.unwrap_or(0),
            days_lived: 0,
            vitals: AgentVitalState::ALIVE,
            planned_action: 0,
            brain: AgentBrain::new(n_actions),
        }
    }

    /// Choose how many resources to take
    pub fn decide_action(&mut self) {
        // Q-Learning decision process
        self.planned_action = self.brain.decide_action();
    }
    /// Tell how many resource I want
    pub fn desired_resources(&self) -> i32 {
        return self.planned_action;
    }
    /// Receive the resources, update the EV from the last action
    pub fn get_resources(&mut self, value: i32) {
        self.score += value;
        self.brain.set_last_reward(value);
    }
    /// Consume resources to stay alive, or perish if they are out
    pub fn consume(&mut self, value: i32) {
        self.score -= value;
        self.brain.decrease_last_reward(value);
        if self.score < 0 {
            self.vitals = AgentVitalState::DEAD;
            self.brain.decrease_last_reward(10000);// TODO config punish value
        }
    }
    /// Update expected values
    pub fn learn(&mut self) {
        self.brain.update_ev(self.planned_action as usize);
    }


    pub fn is_alive(&self) -> bool {
        return self.vitals == AgentVitalState::ALIVE;
    }

    pub fn revive(&mut self) {
        self.vitals = AgentVitalState::ALIVE;
    }

    pub fn report_action_evs(&self) {
        self.brain.report_action_evs();
    }

    pub fn print_score(&self) {
        println!("Agent {} has score {}", self.id, self.score);
    }

    //    pub fn live(&mut self) {
    // Advance one time step, so do things like
    // reducing score, planning next action, etc.
    //    }
}
