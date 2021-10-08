use float_ord::FloatOrd;
use rand::seq::SliceRandom;
use std::ops::{Index, IndexMut};

/// An action / action availible to an agent, tracking its own statistics
pub struct Action {
    /// Number of resources to take
    num_resources: i32,
    /// Expected value of that action
    expected_value: f32,
    /// Number of times the action is chosen
    times_chosen: i32,
}

impl Action {
    pub fn new(num_resources: i32, expected_value: f32, times_chosen: i32) -> Action {
        Action {
            num_resources: num_resources,
            expected_value: expected_value,
            times_chosen: times_chosen,
        }
    }

    pub fn increment_chosen(&mut self, n: i32) {
        self.times_chosen += n;
    }

    pub fn get_num_resources(&self) -> i32 {
        return self.num_resources;
    }

    pub fn get_expected_value(&self) -> f32 {
        return self.expected_value;
    }

    pub fn set_expected_value(&mut self, value: f32) {
        self.expected_value = value;
    }
}

pub struct Actions {
    // Consider changing implementation to HashMap or other data structure
    actions: Vec<Action>,
}

/// Container for all availible actions. All 'non-cognitive' operations on action selection can be done here
impl Actions {
    pub fn new(num_actions: i32) -> Actions {
        Actions {
            actions: Self::init_actions(num_actions),
        }
    }

    fn init_actions(num_actions: i32) -> Vec<Action> {
        let mut actions: Vec<Action> = Vec::with_capacity(num_actions as usize);
        for i in 0..num_actions {
            // TODO: initialization strategy can be applied here
            actions.push(Action::new(i, 0., 0));
        }
        return actions;
    }

    pub fn max_ev_action(&mut self) -> &mut Action {
        return self
            .actions
            .iter_mut()
            .max_by_key(|action| FloatOrd(action.expected_value))
            .unwrap();
    }

    pub fn random_action(&mut self) -> &mut Action {
        return self.actions.choose_mut(&mut rand::thread_rng()).unwrap();
    }
}

impl Index<usize> for Actions {
    type Output = Action;
    fn index(&self, i: usize) -> &Action {
        &self.actions[i]
    }
}

impl IndexMut<usize> for Actions {
    fn index_mut(&mut self, i: usize) -> &mut Action {
        &mut self.actions[i]
    }
}
