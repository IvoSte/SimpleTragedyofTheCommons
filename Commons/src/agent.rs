pub enum AgentState {
    ALIVE,
    DEAD
}

pub struct Agent {
    pub id: u32,
    pub score: i32,
    pub days_lived: u32,
    pub state: AgentState,
    pub planned_action: u32,
}

impl Agent {
    /// Returns an Agent with the given id and score
    ///
    /// # Arguments
    ///
    /// * `id`    - An unsigned int to uniquely identify this agent
    /// * `score` - An integer that represents the agent's starting score
    pub fn new(id: u32, score: Option<i32>) -> Agent {
        Agent {
            id: id,
            // Default score value 0
            score: score.unwrap_or(0),
            days_lived: 0,
            state: AgentState::ALIVE,
            planned_action: 0,
        }
    }

    /// Choose how many resources to take
    pub fn decide_action(&mut self) {
    // Q-Learning decision process
        self.planned_action = 1;
    }

    pub fn desired_resources(&self) -> u32 {
        return self.planned_action;
    }

    pub fn give_resources(&mut self, value: i32) {
        self.score += value;
    }

    pub fn live(&mut self) {
        // Advance one time step, so do things like
        // reducing score, planning next action, etc.
    }
}
