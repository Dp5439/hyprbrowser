// Examples: Using GPU Detection & Optimization in HyprBrowser

// =============================================================================
// EXAMPLE 1: Basic GPU Detection
// =============================================================================

#[cfg(test)]
mod example_gpu_detection {
    use crate::gpu_detect::{GpuInfo, GpuTier, GpuSettings};

    #[tokio::test]
    async fn detect_gpu_at_startup() {
        // This is automatically called in Application::new()
        let gpu_info = GpuInfo::detect().await;

        println!("GPU Detection Result:");
        println!("  Tier: {:?}", gpu_info.tier);
        println!("  Adapter: {}", gpu_info.adapter_name);
        println!("  Backend: {:?}", gpu_info.backend);
        println!("  Max Texture: {}", gpu_info.max_texture_size);
        println!("  Max Buffer: {}", gpu_info.max_uniform_buffer_size);

        // Auto-generate settings based on GPU tier
        let settings = GpuSettings::for_gpu(&gpu_info);

        println!("\nAuto-Generated Settings:");
        println!("  Effect Intensity: {:.0}%", settings.effect_intensity * 100.0);
        println!("  Particle Count: {}", settings.particle_count);
        println!("  Target FPS: {}", settings.animation_fps_target);
        println!("  Reduce Transparency: {}", settings.reduce_transparency_effects);
    }
}

// =============================================================================
// EXAMPLE 2: Conditional Feature Rendering
// =============================================================================

#[cfg(test)]
mod example_feature_detection {
    use crate::gpu_detect::{GpuInfo, ShaderFeature};

    fn render_advanced_snow(gpu_info: &GpuInfo) {
        if gpu_info.supports_shader_feature(ShaderFeature::AdvancedSnow) {
            // Full advanced snow with complex particles
            println!("✓ Rendering advanced snow effect");
            render_shader_snow(500); // Full particles
        } else {
            // Simplified snow fallback
            println!("✓ Rendering simplified snow effect");
            render_basic_snow(150); // Reduced particles
        }
    }

    fn render_shader_snow(particle_count: usize) {
        println!("  Particle Count: {}", particle_count);
        println!("  Physics: Full");
        println!("  Lighting: Per-particle");
    }

    fn render_basic_snow(particle_count: usize) {
        println!("  Particle Count: {}", particle_count);
        println!("  Physics: Simplified");
        println!("  Lighting: Global");
    }

    #[test]
    fn example() {
        let gpu_info = crate::gpu_detect::GpuInfo {
            tier: crate::gpu_detect::GpuTier::LowPower,
            adapter_name: "Intel UHD Graphics 630".to_string(),
            backend: wgpu::BackendBit::DX12,
            max_texture_size: 16384,
            max_uniform_buffer_size: 65536,
            features: wgpu::Features::empty(),
            limits: wgpu::Limits::default(),
        };

        render_advanced_snow(&gpu_info);
    }
}

// =============================================================================
// EXAMPLE 3: Dynamic Effect Adjustment
// =============================================================================

#[cfg(test)]
mod example_dynamic_adjustment {
    use crate::gpu_detect::GpuSettings;

    struct EffectManager {
        settings: GpuSettings,
        frame_time_ms: f64,
    }

    impl EffectManager {
        fn update(&mut self) {
            // Monitor frame time and dynamically adjust
            if self.frame_time_ms > 33.0 {
                // Dropping below 30 FPS on iGPU
                println!("⚠ Frame time high: {:.2}ms", self.frame_time_ms);
                self.reduce_effects();
            } else if self.frame_time_ms < 20.0 {
                // Good headroom, could increase effects
                println!("✓ Frame time good: {:.2}ms", self.frame_time_ms);
            }
        }

        fn reduce_effects(&mut self) {
            println!("Reducing visual effects...");
            self.settings.effect_intensity = (self.settings.effect_intensity - 0.1).max(0.1);
            self.settings.particle_count = (self.settings.particle_count / 2).max(30);
            println!(
                "  New intensity: {:.0}% | Particles: {}",
                self.settings.effect_intensity * 100.0,
                self.settings.particle_count
            );
        }
    }

    #[test]
    fn example() {
        let settings = crate::gpu_detect::GpuSettings {
            enable_snow_effect: true,
            effect_intensity: 0.6,
            particle_count: 150,
            animation_fps_target: 45,
            reduce_transparency_effects: true,
        };

        let mut manager = EffectManager {
            settings,
            frame_time_ms: 35.0, // Simulating slow frame
        };

        manager.update();
    }
}

// =============================================================================
// EXAMPLE 4: GPU-Aware Module Development
// =============================================================================

#[cfg(test)]
mod example_module_gpu_optimization {
    use crate::gpu_detect::{GpuInfo, GpuSettings};

    struct CustomModule {
        name: String,
        gpu_settings: Option<GpuSettings>,
    }

    impl CustomModule {
        fn initialize_with_gpu(&mut self, gpu_info: &GpuInfo) {
            let settings = GpuSettings::for_gpu(gpu_info);
            self.gpu_settings = Some(settings);

            println!("Module '{}' initialized with GPU settings:", self.name);
            println!("  Tier: {:?}", gpu_info.tier);
            println!("  Effect Intensity: {:.0}%", settings.effect_intensity * 100.0);
            println!("  Particles: {}", settings.particle_count);

            self.adjust_rendering();
        }

        fn adjust_rendering(&self) {
            if let Some(settings) = &self.gpu_settings {
                if settings.effect_intensity < 0.5 {
                    println!("  → Using lightweight shaders");
                    println!("  → Disabling particle effects");
                } else if settings.effect_intensity < 0.8 {
                    println!("  → Using standard shaders");
                    println!("  → Reduced particle count");
                } else {
                    println!("  → Using advanced shaders");
                    println!("  → Full particle effects");
                }
            }
        }
    }

    #[test]
    fn example() {
        let gpu_info = crate::gpu_detect::GpuInfo {
            tier: crate::gpu_detect::GpuTier::LowPower,
            adapter_name: "AMD Radeon Graphics".to_string(),
            backend: wgpu::BackendBit::VULKAN,
            max_texture_size: 8192,
            max_uniform_buffer_size: 65536,
            features: wgpu::Features::empty(),
            limits: wgpu::Limits::default(),
        };

        let mut module = CustomModule {
            name: "CustomEffects".to_string(),
            gpu_settings: None,
        };

        module.initialize_with_gpu(&gpu_info);
    }
}

// =============================================================================
// EXAMPLE 5: Benchmarking GPU Performance
// =============================================================================

#[cfg(test)]
mod example_benchmarking {
    use crate::gpu_benchmark::GpuBenchmark;

    #[tokio::test]
    async fn run_benchmark_suite() {
        println!("Running comprehensive GPU benchmarks...\n");

        // Benchmark iGPU
        let mut igpu_bench =
            GpuBenchmark::new(crate::gpu_detect::GpuTier::LowPower);
        let _ = igpu_bench.run_all().await;
        println!("{}", igpu_bench.report());

        // Benchmark discrete GPU
        let mut dgpu_bench =
            GpuBenchmark::new(crate::gpu_detect::GpuTier::HighPerformance);
        let _ = dgpu_bench.run_all().await;
        println!("{}", dgpu_bench.report());
    }
}

// =============================================================================
// EXAMPLE 6: Snow Effect with GPU Optimization
// =============================================================================

#[cfg(test)]
mod example_snow_optimization {
    use crate::snow::SnowEffect;
    use crate::gpu_detect::GpuSettings;

    fn create_snow_effect_for_gpu(settings: &GpuSettings) -> SnowEffect {
        println!("Creating snow effect:");
        println!("  Particle Count: {}", settings.particle_count);
        println!("  Effect Intensity: {:.0}%", settings.effect_intensity * 100.0);

        // Create snow with GPU-optimized particle count
        let snow = SnowEffect::with_particle_count(5, settings.particle_count as u64);

        println!("  Status: ✓ Ready");

        snow
    }

    #[test]
    fn example() {
        // iGPU settings
        let igpu_settings = crate::gpu_detect::GpuSettings {
            enable_snow_effect: true,
            effect_intensity: 0.6,
            particle_count: 150,
            animation_fps_target: 45,
            reduce_transparency_effects: true,
        };

        let snow = create_snow_effect_for_gpu(&igpu_settings);
        println!("Snow particles: {}", snow.particles.len());

        println!("\n---\n");

        // dGPU settings
        let dgpu_settings = crate::gpu_detect::GpuSettings {
            enable_snow_effect: true,
            effect_intensity: 1.0,
            particle_count: 500,
            animation_fps_target: 60,
            reduce_transparency_effects: false,
        };

        let snow = create_snow_effect_for_gpu(&dgpu_settings);
        println!("Snow particles: {}", snow.particles.len());
    }
}

// =============================================================================
// EXAMPLE 7: Memory-Efficient Particle System for iGPU
// =============================================================================

#[cfg(test)]
mod example_efficient_particles {
    use crate::gpu_detect::{GpuInfo, GpuTier};

    struct ParticleSystem {
        max_particles: usize,
        memory_per_particle: usize,
    }

    impl ParticleSystem {
        fn new_for_gpu(gpu_info: &GpuInfo) -> Self {
            // Calculate max particles based on GPU and limits
            let memory_available = match gpu_info.tier {
                GpuTier::HighPerformance => 100 * 1024 * 1024, // 100 MB
                GpuTier::LowPower => 20 * 1024 * 1024,         // 20 MB (shared system RAM)
                GpuTier::Software => 5 * 1024 * 1024,          // 5 MB
            };

            let memory_per_particle = 32; // bytes: pos(8) + vel(8) + size(4) + opacity(4) + padding(8)
            let max_particles = memory_available / memory_per_particle;

            println!("ParticleSystem initialized for {:?}:", gpu_info.tier);
            println!("  Memory available: {:.1} MB", memory_available as f64 / 1024.0 / 1024.0);
            println!("  Memory per particle: {} bytes", memory_per_particle);
            println!("  Max particles: {}", max_particles);

            ParticleSystem {
                max_particles,
                memory_per_particle,
            }
        }

        fn get_optimal_count(&self) -> usize {
            // Use 80% of max to leave headroom
            (self.max_particles as f64 * 0.8) as usize
        }
    }

    #[test]
    fn example() {
        let igpu_info = crate::gpu_detect::GpuInfo {
            tier: GpuTier::LowPower,
            adapter_name: "Intel UHD".to_string(),
            backend: wgpu::BackendBit::DX12,
            max_texture_size: 16384,
            max_uniform_buffer_size: 65536,
            features: wgpu::Features::empty(),
            limits: wgpu::Limits::default(),
        };

        let system = ParticleSystem::new_for_gpu(&igpu_info);
        println!("\nOptimal particle count: {}\n", system.get_optimal_count());

        let dgpu_info = crate::gpu_detect::GpuInfo {
            tier: GpuTier::HighPerformance,
            adapter_name: "NVIDIA GeForce".to_string(),
            backend: wgpu::BackendBit::VULKAN,
            max_texture_size: 16384,
            max_uniform_buffer_size: 65536,
            features: wgpu::Features::empty(),
            limits: wgpu::Limits::default(),
        };

        let system = ParticleSystem::new_for_gpu(&dgpu_info);
        println!("Optimal particle count: {}", system.get_optimal_count());
    }
}

// =============================================================================
// EXAMPLE 8: Performance Monitoring Loop
// =============================================================================

#[cfg(test)]
mod example_perf_monitoring {
    use std::time::Instant;

    struct PerformanceMonitor {
        frame_times: Vec<f64>,
        max_samples: usize,
    }

    impl PerformanceMonitor {
        fn new() -> Self {
            PerformanceMonitor {
                frame_times: Vec::new(),
                max_samples: 120, // Last 2 seconds @ 60 FPS
            }
        }

        fn record_frame(&mut self, frame_time_ms: f64) {
            self.frame_times.push(frame_time_ms);
            if self.frame_times.len() > self.max_samples {
                self.frame_times.remove(0);
            }
        }

        fn avg_frame_time(&self) -> f64 {
            if self.frame_times.is_empty() {
                0.0
            } else {
                self.frame_times.iter().sum::<f64>() / self.frame_times.len() as f64
            }
        }

        fn max_frame_time(&self) -> f64 {
            self.frame_times.iter().cloned().fold(0.0, f64::max)
        }

        fn fps(&self) -> f64 {
            if self.avg_frame_time() > 0.0 {
                1000.0 / self.avg_frame_time()
            } else {
                0.0
            }
        }

        fn report(&self) {
            println!("Performance Report:");
            println!("  Avg Frame Time: {:.2} ms", self.avg_frame_time());
            println!("  Max Frame Time: {:.2} ms", self.max_frame_time());
            println!("  Average FPS: {:.1}", self.fps());
        }
    }

    #[test]
    fn example() {
        let mut monitor = PerformanceMonitor::new();

        // Simulate frame times
        for i in 0..120 {
            let frame_time = if i < 60 {
                22.0 // 45 FPS
            } else {
                16.7 // 60 FPS
            };
            monitor.record_frame(frame_time);
        }

        monitor.report();
    }
}
