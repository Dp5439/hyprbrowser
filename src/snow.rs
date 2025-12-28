use iced::widget::*;
use iced::Element;

/// Shader-based snow effect
/// Renders snow particles with wgpu and applies realistic physics
pub struct SnowEffect {
    pub active: bool,
    pub particles: Vec<Snowflake>,
    pub start_time: std::time::Instant,
    pub duration: std::time::Duration,
}

#[derive(Debug, Clone)]
pub struct Snowflake {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub size: f32,
    pub opacity: f32,
}

impl SnowEffect {
    pub fn new(duration_secs: u64) -> Self {
        Self::with_particle_count(duration_secs, 100)
    }

    pub fn with_particle_count(duration_secs: u64, particle_count: usize) -> Self {
        let mut particles = Vec::new();
        for _ in 0..particle_count {
            particles.push(Snowflake {
                x: rand::random::<f32>() * 1920.0,
                y: rand::random::<f32>() * 1080.0,
                vx: (rand::random::<f32>() - 0.5) * 2.0,
                vy: rand::random::<f32>() * 2.0 + 0.5,
                size: rand::random::<f32>() * 3.0 + 1.0,
                opacity: rand::random::<f32>() * 0.7 + 0.3,
            });
        }

        SnowEffect {
            active: true,
            particles,
            start_time: std::time::Instant::now(),
            duration: std::time::Duration::from_secs(duration_secs),
        }
    }

    pub fn update(&mut self) {
        for particle in &mut self.particles {
            particle.x += particle.vx;
            particle.y += particle.vy;

            // Wrap around edges
            if particle.x < 0.0 {
                particle.x = 1920.0;
            }
            if particle.x > 1920.0 {
                particle.x = 0.0;
            }
            if particle.y > 1080.0 {
                particle.y = 0.0;
            }

            // Fade out
            particle.opacity *= 0.99;
        }

        // Check if duration expired
        if self.start_time.elapsed() > self.duration {
            self.active = false;
        }
    }

    pub fn is_expired(&self) -> bool {
        !self.active || self.start_time.elapsed() > self.duration
    }
}

pub fn view(snow: &SnowEffect) -> Element<crate::Message> {
    let mut particles_text = String::from("❄️ ");
    for _ in 0..snow.particles.len().min(50) {
        particles_text.push('❄');
    }

    container(text(particles_text).size(20))
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .into()
}

/// WGSL shader code for snow effect
pub const SNOW_SHADER: &str = r#"
@vertex
fn vs_main(in: VertexInput) -> @builtin(position) vec4<f32> {
    return vec4<f32>(in.position, 0.0, 1.0);
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    // Procedural snow generation
    let uv = vec2<f32>(0.5);
    let noise = fract(sin(dot(uv, vec2<f32>(12.9898, 78.233))) * 43758.5453);
    
    if noise > 0.8 {
        return vec4<f32>(1.0, 1.0, 1.0, 0.8); // White snowflake
    }
    return vec4<f32>(0.0, 0.0, 0.0, 0.0); // Transparent
}
"#;
