use ::rand::Rng;
use macroquad::prelude::*;

// Visual representation structs
#[derive(Clone, Copy, Debug)]
pub struct Unit {
    pub x: f32,
    pub y: f32,
    pub color: Color,
}

#[derive(Clone, Copy, Debug)]
pub struct CreepUnit {
    pub x: f32,
    pub y: f32,
    pub color: Color,
    pub hit_timer: f32, // Visual timer for hit effect
}

#[derive(Clone, Copy, Debug)]
pub struct ParticleUnit {
    pub x: f32,
    pub y: f32,
    pub color: Color,
    pub vx: f32,
    pub vy: f32,
    pub life: f32,
    pub max_life: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct SoundWaveUnit {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub color: Color,
    pub life: f32,
    pub max_life: f32,
}

// Visual effects manager
pub struct VisualEffects {
    pub particles: Vec<ParticleUnit>,
    pub sound_waves: Vec<SoundWaveUnit>,
    pub flash_timer: f32,
    pub reset_timer: f32,
}

impl VisualEffects {
    pub fn new() -> Self {
        VisualEffects {
            particles: Vec::new(),
            sound_waves: Vec::new(),
            flash_timer: 0.0,
            reset_timer: 0.0,
        }
    }

    pub fn add_explosion(&mut self, x: f32, y: f32, color: Color) {
        let mut rng = ::rand::thread_rng();

        // Create 15-25 particles per explosion
        let particle_count = rng.gen_range(15..25);

        for _ in 0..particle_count {
            let angle = rng.gen_range(0.0..std::f64::consts::TAU);
            let speed = rng.gen_range(0.3..0.8);
            let vx = angle.cos() * speed;
            let vy = angle.sin() * speed;
            let life = rng.gen_range(0.5..1.5);

            // Random color: mostly the specified color with some variation
            let particle_color = match rng.gen_range(0..10) {
                0..=7 => color,
                8 => WHITE,                          // Some white particles for highlight
                _ => Color::new(0.0, 1.0, 1.0, 1.0), // Some cyan particles for variation
            };

            self.particles.push(ParticleUnit {
                x,
                y,
                color: particle_color,
                vx: vx as f32,
                vy: vy as f32,
                life: life as f32,
                max_life: life as f32,
            });
        }
    }

    pub fn add_sound_wave(&mut self, x: f32, y: f32) {
        self.sound_waves.push(SoundWaveUnit {
            x,
            y,
            radius: 0.0,
            color: Color::new(0.0, 1.0, 1.0, 0.5), // Semi-transparent cyan
            life: 1.0,
            max_life: 1.0,
        });
    }

    pub fn trigger_flash(&mut self) {
        self.flash_timer = 0.2; // Flash for this duration in [s]
    }

    pub fn trigger_reset_message(&mut self) {
        self.reset_timer = 2.0; // Show message for 2 seconds
    }

    pub fn update(&mut self, delta_time: f32) {
        // Update flash timer
        if self.flash_timer > 0.0 {
            self.flash_timer -= delta_time;
        }

        // Update reset timer
        if self.reset_timer > 0.0 {
            self.reset_timer -= delta_time;
        }

        // Update particles - they move and fade out
        self.particles.retain_mut(|particle| {
            particle.x += particle.vx * delta_time * 60.0; // Scale by expected FPS
            particle.y += particle.vy * delta_time * 60.0;
            particle.life -= delta_time * 2.16; // Decrease life based on frame time (assuming ~60fps)
            particle.life > 0.0
        });

        // Update sound waves - they expand and fade out
        self.sound_waves.retain_mut(|wave| {
            wave.radius += 0.5 * delta_time * 60.0; // Expand the wave
            wave.life -= 0.05 * delta_time * 60.0;
            wave.life > 0.0
        });
    }

    pub fn draw(&self) {
        // Draw particles with alpha based on their lifetime
        for particle in &self.particles {
            let alpha = particle.life / particle.max_life;
            let size = 3.0 * (1.0 - alpha * 0.7); // Particles shrink as they fade

            let particle_color = Color {
                r: particle.color.r,
                g: particle.color.g,
                b: particle.color.b,
                a: alpha,
            };

            draw_circle(particle.x, particle.y, size, particle_color);
        }

        // Draw sound waves as semi-transparent rings
        for wave in &self.sound_waves {
            let alpha = wave.life / wave.max_life;
            let wave_color = Color {
                r: wave.color.r,
                g: wave.color.g,
                b: wave.color.b,
                a: alpha * wave.color.a,
            };

            draw_circle_lines(wave.x, wave.y, wave.radius, 1.0, wave_color);
        }
    }

    pub fn get_flash_intensity(&self) -> f32 {
        if self.flash_timer > 0.0 {
            (self.flash_timer * 10.0).sin().abs()
        } else {
            0.0
        }
    }

    pub fn get_reset_alpha(&self) -> f32 {
        if self.reset_timer > 0.0 {
            (self.reset_timer / 2.0).min(1.0)
        } else {
            0.0
        }
    }
}

// Main graphics struct
pub struct SimulationGraphics {
    pub units: Vec<Unit>,
    pub creeps: Vec<CreepUnit>,
    pub visual_effects: VisualEffects,
}

impl SimulationGraphics {
    pub fn new() -> Self {
        SimulationGraphics {
            units: Vec::new(),
            creeps: Vec::new(),
            visual_effects: VisualEffects::new(),
        }
    }

    pub fn add_unit(&mut self, x: f32, y: f32, color: Color) {
        self.units.push(Unit { x, y, color });
    }

    pub fn update_creeps(&mut self, creeps: &[crate::Creep], scale: f32, offset_x: f32, offset_y: f32) {
        self.creeps.clear();

        for creep in creeps {
            // Get base color based on creep type
            let base_color = match creep.creep_type {
                crate::CreepType::Green => GREEN,
                crate::CreepType::Red => RED,
            };

            // For now, creeps don't have hit_timer in simulation anymore
            // Visual effects are handled separately
            self.creeps.push(CreepUnit {
                x: (creep.x as f32 - 5.0) * scale + offset_x,
                y: (creep.y as f32 - 5.0) * scale + offset_y,
                color: base_color,
                hit_timer: 0.0,
            });
        }
    }

    pub fn handle_collision_event(&mut self, event: &crate::CollisionEvent, scale: f32, offset_x: f32, offset_y: f32) {
        // Create explosion particles at collision position
        let color = match event.creep_type {
            crate::CreepType::Green => GREEN,
            crate::CreepType::Red => RED,
        };

        let screen_x = (event.x as f32 - 5.0) * scale + offset_x;
        let screen_y = (event.y as f32 - 5.0) * scale + offset_y;

        self.visual_effects.add_explosion(screen_x, screen_y, color);
        self.visual_effects.add_sound_wave(screen_x, screen_y);
    }

    pub fn trigger_all_green_completed(&mut self) {
        self.visual_effects.trigger_flash();
        self.visual_effects.trigger_reset_message();
    }

    pub fn update(&mut self, delta_time: f32) {
        self.visual_effects.update(delta_time);
    }

    pub fn draw(&self) {
        // Draw units (player)
        for unit in &self.units {
            draw_circle(unit.x, unit.y, 10.0, unit.color);
        }

        // Draw creeps
        for creep in &self.creeps {
            // Add pulsing effect for creeps with hit_timer
            let mut creep_color = creep.color;
            if creep.hit_timer > 0.0 {
                let pulse_factor = (creep.hit_timer * 5.0).sin().abs();
                let white_factor = pulse_factor;
                creep_color = Color {
                    r: creep.color.r * (1.0 - white_factor) + WHITE.r * white_factor,
                    g: creep.color.g * (1.0 - white_factor) + WHITE.g * white_factor,
                    b: creep.color.b * (1.0 - white_factor) + WHITE.b * white_factor,
                    a: 1.0,
                };
            }
            draw_circle(creep.x, creep.y, 7.5, creep_color);
        }

        // Draw visual effects
        self.visual_effects.draw();
    }

    pub fn get_flash_intensity(&self) -> f32 {
        self.visual_effects.get_flash_intensity()
    }

    pub fn get_reset_alpha(&self) -> f32 {
        self.visual_effects.get_reset_alpha()
    }

    /// Graphical feedback on completion of all greens: flash background white
    pub fn flash_bakground_white_if_timer(&mut self) {
        // Flash background white if flash timer is active
        let flash_intensity = self.get_flash_intensity();
        if flash_intensity > 0.0 {
            let bg_color = Color {
                r: flash_intensity,
                g: flash_intensity,
                b: flash_intensity,
                a: 1.0,
            };
            clear_background(bg_color);
        } else {
            clear_background(BLACK);
        }
    }
}
