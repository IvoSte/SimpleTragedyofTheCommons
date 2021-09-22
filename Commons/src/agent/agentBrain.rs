pub struct Choice {
    num_resources: u32,
    expected_value: f32,
    num_chosen: u32,
}

impl Choice {
    fn new(num_resources: u32, expected_value: f32, num_chosen: u32) -> Choice {
        Choice {
            num_resources: num_resources,
            expected_value: expected_value,
            num_chosen: num_chosen,
        }
    }
}

pub struct AgentBrain {
    pub choices: Vec<Choice>,

}

pub impl AgentBrain {
    
    pub fn new(num_choices: i32) -> AgentBrain {
        AgentBrain {
            let mut choices: Vec<Choice> = Vec::with_capacity(num_choices)
            for choice_num in 0..num_choices {
                // TODO: initialization strategy can be applied here
                choices.push(Choice::new(choice_num, 0, 0));
            }
        }
    }
}
