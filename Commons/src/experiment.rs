
// Aliases
use super::agent::Agent;
use super::commons::Commons;


pub struct Experiment {
    generations: usize,
    epochs: usize,
    agents: &mut Vec<Agent>,
    commons: &mut Commons,
}

impl Experiment {
    pub fn new( generations: usize,
                epochs: usize,
                agents: &mut Vec<Agent>,
                commons: &mut Commons,
    ) -> Experiment {
        Experiment {
            generations, 
            epochs, 
            agents, 
            commons,                    
        }
    }

    pub fn run(&mut self) {
    // An experiment run consists of muliple generations with the same agents.
        for _ in 0..self.generations {
            self.single_generation();
        }    
    }
    
    fn single_generation(&mut self) {
    // One generation runs until the commons are exhausted and all agents are dead, or equilibrium
        for _ in 0..self.epochs {
            self.single_epoch();
        }
        for agent in self.agents {
            agent.revive();
        }
        self.commons.reset();
    }
    
    fn single_epoch(&mut self) {
    // A single epoch is a single day, where each agent decides a single action and the commons gerows once
        for agent in self.agents {
            if agent.is_alive() {
                agent.decide_action();
                let desired_resources = agent.desired_resources();
                let taken_resources = self.commons.take_resources(desired_resources);
                agent.get_resources(taken_resources);
                agent.consume(0);
                agent.print_score();
            }
        }
    
        self.commons.grow();
        self.commons.print_pool();
    }        
}
