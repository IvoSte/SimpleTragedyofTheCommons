pub mod actions;
pub mod agent_brain;
pub mod rl_algs;
pub mod structs;

use self::agent_brain::AgentBrain;
use self::structs::{AgentState, AgentType};

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
            brain: AgentBrain::new(n_actions, AgentType::QLEARNING),
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
            self.die();
        }
    }
    ///  Agent dies 
    pub fn die(&mut self) {
        self.vitals = AgentVitalState::DEAD;
        self.brain.death_punishment();
    }

    /// Manually kill an agent, E.G. when the commons are depleted
    pub fn kill(&mut self) {
        if self.vitals == AgentVitalState::ALIVE { // What is dead may never die
            self.vitals = AgentVitalState::DEAD;
            self.brain.death_punishment();
        }
    }

    /// Update expected values
    pub fn learn(&mut self) {
        self.brain.update_ev(self.planned_action as usize);
    }

    pub fn update_state(&mut self, pool_value: i32) {
        self.brain.update_state(pool_value, self.score);
    }

    pub fn is_alive(&self) -> bool {
        return self.vitals == AgentVitalState::ALIVE;
    }

    pub fn revive(&mut self) {
        //println!("{}", self.score);
        self.score = 0;
        self.vitals = AgentVitalState::ALIVE;
    }

    pub fn report(&self) {
        self.print_score();
        println!("planned action {}", self.planned_action);
        self.report_brain();
    }

    pub fn report_brain(&self) {
        self.brain.report();
    }

    pub fn print_score(&self) {
        println!("Agent {} has score {}", self.id, self.score);
    }

    pub fn get_current_state(&self) -> Option<AgentState> {
        self.brain.get_current_state()
    }
}
