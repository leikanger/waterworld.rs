use macroquad::prelude::*;

pub struct Unit {
    pub x: f32,
    pub y: f32,
    pub color: Color,
}

pub struct CreepUnit {
    pub x: f32,
    pub y: f32,
    pub color: Color,
}

pub struct SimulationGraphics {
    pub units: Vec<Unit>,
    pub creeps: Vec<CreepUnit>,
}

impl SimulationGraphics {
    pub fn new() -> Self {
        SimulationGraphics {
            units: Vec::new(),
            creeps: Vec::new(),
        }
    }

    pub fn add_unit(&mut self, x: f32, y: f32, color: Color) {
        self.units.push(Unit { x, y, color });
    }

    #[allow(unused)]
    pub fn add_creep(&mut self, x: f32, y: f32, color: Color) {
        self.creeps.push(CreepUnit { x, y, color });
    }

    #[allow(unused)]
    pub fn clear_creeps(&mut self) {
        self.creeps.clear();
    }

    pub fn draw(&self) {
        for unit in &self.units {
            draw_circle(unit.x, unit.y, 10.0, unit.color);
        }

        for creep in &self.creeps {
            draw_circle(creep.x, creep.y, 7.5, creep.color);
        }
    }
}
