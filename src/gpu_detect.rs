use wgpu::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuTier {
    /// Discrete GPU - high performance
    HighPerformance,
    /// Integrated GPU - moderate performance
    LowPower,
    /// CPU rendering - fallback
    Software,
}

#[derive(Debug, Clone)]
pub struct GpuInfo {
    pub tier: GpuTier,
    pub adapter_name: String,
    pub backend: Backend,
    pub max_texture_size: u32,
    pub max_uniform_buffer_size: u32,
    pub features: Features,
    pub limits: Limits,
}

impl GpuInfo {
    /// Detect GPU at startup and select best adapter
    pub async fn detect() -> Self {
        let instance = Instance::new(InstanceDescriptor {
            backends: Backends::all(),
            dx12_shader_compiler: Default::default(),
            flags: InstanceFlags::empty(),
            gles_minor_version: Default::default(),
        });

        // Priority 1: High Performance (Discrete GPU)
        let high_perf_adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: None,
            })
            .await;

        if let Some(adapter) = high_perf_adapter {
            let info = adapter.get_info();
            let limits = adapter.limits();
            let features = adapter.features();

            log::info!(
                "Selected HighPerformance GPU: {} ({:?})",
                info.name,
                info.device_type
            );

            return GpuInfo {
                tier: GpuTier::HighPerformance,
                adapter_name: info.name,
                backend: info.backend,
                max_texture_size: limits.max_texture_dimension_2d,
                max_uniform_buffer_size: limits.max_uniform_buffer_binding_size,
                features,
                limits,
            };
        }

        // Priority 2: Low Power (Integrated GPU)
        let low_power_adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::LowPower,
                force_fallback_adapter: false,
                compatible_surface: None,
            })
            .await;

        if let Some(adapter) = low_power_adapter {
            let info = adapter.get_info();
            let limits = adapter.limits();
            let features = adapter.features();

            log::info!(
                "Selected LowPower (iGPU) adapter: {} ({:?})",
                info.name,
                info.device_type
            );

            return GpuInfo {
                tier: GpuTier::LowPower,
                adapter_name: info.name,
                backend: info.backend,
                max_texture_size: limits.max_texture_dimension_2d,
                max_uniform_buffer_size: limits.max_uniform_buffer_binding_size,
                features,
                limits,
            };
        }

        // Fallback: CPU rendering
        log::warn!("No GPU adapter found, falling back to software rendering");

        GpuInfo {
            tier: GpuTier::Software,
            adapter_name: "Software Renderer (CPU)".to_string(),
            backend: Backend::Empty,
            max_texture_size: 2048,
            max_uniform_buffer_size: 65536,
            features: Features::empty(),
            limits: Limits::default(),
        }
    }

    /// Get recommended effect intensity based on GPU tier
    pub fn recommended_effect_intensity(&self) -> f32 {
        match self.tier {
            GpuTier::HighPerformance => 1.0, // 100% effects
            GpuTier::LowPower => 0.6,        // 60% - reduce particles, animation speed
            GpuTier::Software => 0.3,        // 30% - minimal effects
        }
    }

    /// Get recommended particle count for snow effect
    pub fn recommended_particle_count(&self) -> usize {
        match self.tier {
            GpuTier::HighPerformance => 500,  // Full snow
            GpuTier::LowPower => 150,         // Reduced snow
            GpuTier::Software => 30,          // Minimal snow
        }
    }

    /// Check if shader features are supported
    pub fn supports_shader_feature(&self, feature: ShaderFeature) -> bool {
        match feature {
            ShaderFeature::AdvancedSnow => {
                self.tier != GpuTier::Software
                    && self.max_texture_size >= 2048
                    && self.max_uniform_buffer_size >= 65536
            }
            ShaderFeature::ComplexAnimations => {
                self.tier != GpuTier::Software || self.tier == GpuTier::LowPower
            }
            ShaderFeature::HighResTextures => self.tier == GpuTier::HighPerformance,
        }
    }

    /// Get device info summary
    pub fn to_string(&self) -> String {
        format!(
            "GPU: {} | Tier: {:?} | Backend: {:?} | Max Texture: {} | Max Uniform Buffer: {}",
            self.adapter_name, self.tier, self.backend, self.max_texture_size, self.max_uniform_buffer_size
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShaderFeature {
    AdvancedSnow,
    ComplexAnimations,
    HighResTextures,
}

/// GPU performance settings
#[derive(Debug, Clone, Copy)]
pub struct GpuSettings {
    pub enable_snow_effect: bool,
    pub effect_intensity: f32, // 0.0 to 1.0
    pub particle_count: usize,
    pub animation_fps_target: u32,
    pub reduce_transparency_effects: bool,
}

impl GpuSettings {
    /// Create optimized settings based on GPU tier
    pub fn for_gpu(gpu_info: &GpuInfo) -> Self {
        match gpu_info.tier {
            GpuTier::HighPerformance => GpuSettings {
                enable_snow_effect: true,
                effect_intensity: 1.0,
                particle_count: 500,
                animation_fps_target: 60,
                reduce_transparency_effects: false,
            },
            GpuTier::LowPower => GpuSettings {
                enable_snow_effect: true,
                effect_intensity: 0.6,
                particle_count: 150,
                animation_fps_target: 45,
                reduce_transparency_effects: true,
            },
            GpuTier::Software => GpuSettings {
                enable_snow_effect: true,
                effect_intensity: 0.2,
                particle_count: 30,
                animation_fps_target: 30,
                reduce_transparency_effects: true,
            },
        }
    }
}
