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

pub struct ParticleUnit {
    pub x: f32,
    pub y: f32,
    pub size: f32,
    pub color: Color,
}

pub struct SoundWaveUnit {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub color: Color,
}

pub struct SimulationGraphics {
    pub units: Vec<Unit>,
    pub creeps: Vec<CreepUnit>,
    pub particles: Vec<ParticleUnit>,
    pub sound_waves: Vec<SoundWaveUnit>,
}

impl SimulationGraphics {
    pub fn new() -> Self {
        SimulationGraphics {
            units: Vec::new(),
            creeps: Vec::new(),
            particles: Vec::new(),
            sound_waves: Vec::new(),
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

    pub fn add_particle(&mut self, x: f32, y: f32, size: f32, color: Color) {
        self.particles.push(ParticleUnit { x, y, size, color });
    }

    pub fn clear_particles(&mut self) {
        self.particles.clear();
    }

    pub fn add_sound_wave(&mut self, x: f32, y: f32, radius: f32, color: Color) {
        self.sound_waves.push(SoundWaveUnit { x, y, radius, color });
    }

    pub fn clear_sound_waves(&mut self) {
        self.sound_waves.clear();
    }

    pub fn draw(&self) {
        for unit in &self.units {
            draw_circle(unit.x, unit.y, 10.0, unit.color);
        }

        for creep in &self.creeps {
            draw_circle(creep.x, creep.y, 7.5, creep.color);
        }

        // Draw particles with alpha based on their lifetime
        for particle in &self.particles {
            draw_circle(particle.x, particle.y, particle.size, particle.color);
        }

        // Draw sound waves as semi-transparent rings
        for wave in &self.sound_waves {
            draw_circle_lines(wave.x, wave.y, wave.radius, 1.0, wave.color);
        }
    }
}
