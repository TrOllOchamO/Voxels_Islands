use noise::NoiseFn;

pub struct CircularIsland {
    island_center_x: f64,
    island_center_z: f64,
    island_radius: f64,
}

impl CircularIsland {
    pub fn new(island_center_x: u32, island_center_z: u32, island_radius: u32) -> Self {
        Self {
            island_center_x: island_center_x as f64,
            island_center_z: island_center_z as f64,
            island_radius: island_radius as f64,
        }
    }
}

impl NoiseFn<f64, 2> for CircularIsland {
    fn get(&self, point: [f64; 2]) -> f64 {
        let diff_on_x = self.island_center_x - point[0];
        let diff_on_z = self.island_center_z - point[1];

        let distance = f64::sqrt(diff_on_x.powi(2) + diff_on_z.powi(2));

        if distance > self.island_radius {
            return 0.0;
        }
        1.0
    }
}
