#[derive(Serialize, Deserialize)]
pub struct TragedyConfig {
    pub n_generations: i32,
    pub epochs_per_gen: i32,
    pub n_agents: i32,
    pub n_actions: i32,
    pub init_pool_size: i32,
    pub max_pool_size: i32,
    pub regrowth_rate: f32,
}

impl Default for TragedyConfig {
    fn default() -> Self {
        Self {
            n_generations: 10,
            epochs_per_gen: 200,
            n_agents: 5,
            n_actions: 4,
            init_pool_size: 10,
            max_pool_size: 60,
            regrowth_rate: 1.2,
        }
    }
}
