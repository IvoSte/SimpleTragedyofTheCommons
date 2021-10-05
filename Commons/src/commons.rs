/// The commons resource available to all agents in the ToTC simulation
pub struct Commons {
    init_resources: i32,
    max_resources: i32,
    pub resource_pool: i32,
    pub regrowth_function: fn(i32) -> i32,
}

impl Commons {
    pub fn new(resource_pool: i32, regrowth_function: fn(i32) -> i32) -> Commons {
        Commons {
            init_resources: resource_pool,
            max_resources: 2*resource_pool, // TODO make better, possibly some theoretical explanation
            resource_pool,
            regrowth_function,
        }
    }
    pub fn grow(&mut self) {
        self.resource_pool = (self.regrowth_function)(self.resource_pool);
        if self.resource_pool > self.max_resources {
            self.resource_pool = self.max_resources;
        }
    }
    pub fn take_resources(&mut self, value: i32) -> i32 {
        let res = if self.resource_pool >= value { value } else { self.resource_pool };
        self.resource_pool -= res;
        return res;
    }
    pub fn reset(&mut self) {
        self.resource_pool = self.init_resources;
    }

    pub fn print_pool(&self) {
        println!("Resource pool: {}", self.resource_pool)
    }
}
