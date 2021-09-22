pub struct Commons {
    pub resource_pool: u32,
    pub regrowth_function: fn(u32) -> u32,
}

impl Commons {
    pub fn new(resource_pool: u32, regrowth_function: fn(u32) -> u32) -> Commons {
        Commons {
            resource_pool: resource_pool,
            regrowth_function: regrowth_function,
        }
    }
    pub fn grow(&mut self) {
        self.resource_pool = (self.regrowth_function)(self.resource_pool);
    }
    pub fn take_resources(&mut self, value: u32) -> u32 {
        let res = if self.resource_pool >= value { value } else { self.resource_pool };
        self.resource_pool -= res;
        return res;
    }

    pub fn print_pool(&self) {
        println!("Resource pool: {}", self.resource_pool)
    }
}
