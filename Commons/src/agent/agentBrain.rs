use rand::Rng;

pub struct Action {
    // An action / action availible to an agent, tracking its own statistics
    // Number of resources to take
    num_resources: i32,
    // Expected value of that action
    expected_value: f32,
    // Number of times the action is chosen
    times_chosen: i32,
}

impl Action {
    fn new(num_resources: i32, expected_value: f32, times_chosen: i32) -> Action {
        Action {
            num_resources: num_resources,
            expected_value: expected_value,
            times_chosen: times_chosen,
        }
    }
}

pub struct Actions {
    actions: Vec<Action>,
}

impl Actions {
    // Container for all availible actions. All 'non-cognitive' operations on action selection can be done here

    pub fn new(num_actions: Option<i32>) -> Actions {
        Actions {
            actions: Self::init_actions(num_actions.unwrap_or(5)),
        }
    }

    fn init_actions(num_actions: i32) -> Vec<Action> {
        let mut actions: Vec<Action> = Vec::with_capacity(num_actions as usize);
        for action_num in 0..num_actions {
            // TODO: initialization strategy can be applied here
            actions.push(Action::new(action_num, 0., 0));
        }
        return actions;
    }

    pub fn max_ev_action(&self) -> &Action {
        // This can be a lot cleaner TODO
        let max_action: &mut Action = Action::new(0, -99999., 0);
        for action in &mut self.actions {
            if action.expected_value > max_action.expected_value {
                max_action = action;
            }
        }
        return max_action;
    }

    pub fn random_action(&self) -> &Action {
        return &self.actions.choose(&mut rand::thread_rng());
    }
}

pub struct AgentBrain {
    actions: Vec<Action>,
    last_action: Action,
    last_reward: i32,
}

impl AgentBrain {
    // Cognitive component of the agent. All 'cognitive' operations / decision making of actions can be done here
    pub fn new(num_actions: Option<i32>) -> AgentBrain {
        AgentBrain {
            actions: Actions::new(num_actions.unwrap_or(6)),
            // TODO find better way than declaring dummy defaults
            last_action: Action::new(0, -99999., 0),
            last_reward: 0,
        }
    }

    pub fn decide_action(&self) -> i32 {
        self.last_action = epsilon_greedy(&self.actions, 0.1);
        self.last_action.times_chosen += 1;
        return self.last_action.num_resources;
    }

    pub fn update_ev(&self, value: i32) {
        // TODO ended here. Update EV of the chosen action with the new value.
        // Average received value with amount of times action is chosen. 

        // Dummy function updating the value only to actual value, for testing purposes TODO remove
        self.last_action.expected_value = value as f32;
    }
}

// TODO should this be inside or outside the brain? Outside seems good, but possibly better inside. 
pub fn epsilon_greedy(actions: &Vec<Action>, epsilon: &f32) -> Action {
    if rand::thread_rng().gen::<f32> < epsilon {
        return &actions.random_action();
    } else {
        return &actions.max_ev_action();
    }
}
