use macroquad::prelude::*;

pub struct Unit {
    pub x: f32,
    pub y: f32,
    pub color: Color,
}

pub struct SimulationGraphics {
    pub units: Vec<Unit>,
}

impl SimulationGraphics {
    pub fn new() -> Self {
        SimulationGraphics { units: Vec::new() }
    }

    pub fn add_unit(&mut self, x: f32, y: f32, color: Color) {
        self.units.push(Unit { x, y, color });
    }

    pub fn draw(&self) {
        clear_background(BLACK);

        for unit in &self.units {
            draw_circle(unit.x, unit.y, 10.0, unit.color);
        }
    }
}
