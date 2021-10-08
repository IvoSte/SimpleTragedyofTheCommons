pub mod actions;
pub mod agent_brain;
pub mod rl_algs;

use self::agent_brain::AgentBrain;

/// The state of an agent, either alive or dead
#[derive(PartialEq, Eq)]
pub enum AgentState {
    ALIVE,
    DEAD,
}

/// An agent in the ToTC simulation
pub struct Agent {
    pub id: i32,
    score: i32,
    pub days_lived: i32,
    state: AgentState,
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
            state: AgentState::ALIVE,
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
        self.brain.update_ev(value as usize);
    }
    /// Consume resources to stay alive, or perish if they are out
    pub fn consume(&mut self, value: i32) {
        self.score -= value;
        if self.score < 0 {
            self.state = AgentState::DEAD;
        }
    }

    pub fn is_alive(&self) -> bool {
        return self.state == AgentState::ALIVE;
    }

    pub fn revive(&mut self) {
        self.state = AgentState::ALIVE;
    }

    pub fn print_score(&self) {
        println!("Agent {} has score {}", self.id, self.score);
    }

    //    pub fn live(&mut self) {
    // Advance one time step, so do things like
    // reducing score, planning next action, etc.
    //    }
}
