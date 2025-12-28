# GPU Configuration Reference

## Quick Start

### Auto-Detection (Recommended)

HyprBrowser automatically detects your GPU on startup:

```
1. Startup → GPU Detection initiated
2. Try discrete GPU (NVIDIA/AMD)
3. Fall back to iGPU (Intel/AMD APU)
4. CPU software rendering if no GPU
5. Apply optimal settings for detected tier
```

**No configuration needed.** The browser will:
- ✅ Detect your GPU type
- ✅ Apply appropriate effect intensity (60% for iGPU, 100% for dGPU)
- ✅ Set particle counts (150 for iGPU, 500 for dGPU)
- ✅ Log GPU info on startup

---

## Manual GPU Settings

### Via Workflow Panel

**Path**: Sidebar → ⚙ Gear Icon → Workflow → GPU Settings

### Available Controls

#### 1. Effect Intensity Toggle
- **Button**: Click "Effect Intensity: X%"
- **Cycles**: 100% → 60% → 30% → 100%
- **Real-time**: Applied immediately to next animation

```
High Performance:  100% → Full shader complexity
Low Power (iGPU):   60% → Reduced particles, simplified math
Software (CPU):     30% → Minimal effects, emoji fallback
```

#### 2. Particle Count Slider
- **Range**: 30–500 particles
- **iGPU default**: 150
- **dGPU default**: 500
- **Affects**: Snow effect (letitsnow), any particle-based effects

```
Manual adjustment:
snow_effect.with_particle_count(5, 200) // 200 particles for 5 seconds
```

#### 3. GPU Information Display
- **Adapter Name**: GPU model detected
- **Tier**: HighPerformance / LowPower / Software
- **Max Texture**: Maximum texture size supported
- **Max Buffer**: Maximum uniform buffer size

#### 4. Reduce Transparency Effects
- **Toggle**: Checkbox in GPU Settings
- **Effect**: Disables blur/fade animations (iGPU mode)
- **Impact**: 15-20% FPS improvement on iGPU
- **Recommended**: Enabled on iGPU, disabled on dGPU

---

## Configuration Files

### Location

**Windows**: `%APPDATA%\hyprbrowser\gpu_config.json`  
**Linux**: `~/.local/share/hyprbrowser/gpu_config.json`  
**macOS**: `~/Library/Application Support/hyprbrowser/gpu_config.json`

### Default Configuration

```json
{
  "auto_detect": true,
  "gpu_tier": "LowPower",
  "effect_intensity": 0.6,
  "particle_count": 150,
  "animation_fps_target": 45,
  "reduce_transparency_effects": true,
  "backend": "auto"
}
```

### Manual Override Example

Create/edit `gpu_config.json`:

```json
{
  "auto_detect": false,
  "force_tier": "LowPower",
  "override_effect_intensity": 0.5,
  "override_particle_count": 100,
  "animation_fps_target": 40,
  "reduce_transparency_effects": true
}
```

**Note**: Restart HyprBrowser for manual config changes to apply.

---

## Environment Variables

### Override GPU Detection

```bash
# Force iGPU mode
set HYPRBROWSER_GPU_TIER=LowPower

# Force effect intensity (0.0–1.0)
set HYPRBROWSER_EFFECT_INTENSITY=0.5

# Force particle count
set HYPRBROWSER_PARTICLE_COUNT=100

# Target FPS
set HYPRBROWSER_TARGET_FPS=45

# Windows: DirectX12 backend
set HYPRBROWSER_WGPU_BACKEND=dx12

# Linux: Vulkan backend
set HYPRBROWSER_WGPU_BACKEND=vulkan
```

### Example

```bash
# Windows PowerShell
$env:HYPRBROWSER_GPU_TIER = "LowPower"
$env:HYPRBROWSER_EFFECT_INTENSITY = "0.6"
hyprbrowser.exe

# Linux/macOS
export HYPRBROWSER_GPU_TIER=LowPower
export HYPRBROWSER_EFFECT_INTENSITY=0.6
./hyprbrowser
```

---

## Performance Preset Configurations

### Preset 1: High Performance (Discrete GPU)

```json
{
  "gpu_tier": "HighPerformance",
  "effect_intensity": 1.0,
  "particle_count": 500,
  "animation_fps_target": 60,
  "reduce_transparency_effects": false
}
```

**For**: NVIDIA GTX 1060+, RTX 2060+, AMD Radeon RX 5600+

### Preset 2: Balanced (iGPU)

```json
{
  "gpu_tier": "LowPower",
  "effect_intensity": 0.6,
  "particle_count": 150,
  "animation_fps_target": 45,
  "reduce_transparency_effects": true
}
```

**For**: Intel UHD 630, Iris, AMD Radeon Graphics

### Preset 3: Power Saving (Old iGPU / Ultra-lightweight)

```json
{
  "gpu_tier": "LowPower",
  "effect_intensity": 0.3,
  "particle_count": 75,
  "animation_fps_target": 30,
  "reduce_transparency_effects": true
}
```

**For**: Intel HD 4000, Iris Graphics 5100, AMD APU E-series

### Preset 4: Software Rendering (CPU Only)

```json
{
  "gpu_tier": "Software",
  "effect_intensity": 0.2,
  "particle_count": 30,
  "animation_fps_target": 30,
  "reduce_transparency_effects": true
}
```

**For**: Virtual machines, headless systems, no GPU available

---

## Backend Selection

### Automatic Backend (Recommended)

```json
{
  "backend": "auto"
}
```

The browser selects optimal backend per-platform:
- **Windows**: DirectX12 (best compatibility + performance)
- **Linux**: Vulkan (modern, efficient)
- **macOS**: Metal (via wgpu)

### Manual Backend Override

#### Windows

```json
{
  "backend": "dx12"
}
```

Options: `dx12`, `vulkan`, `gl`

#### Linux

```json
{
  "backend": "vulkan"
}
```

Options: `vulkan`, `gl`

#### Platform-Specific iGPU Optimization

**Intel iGPU on Windows (DirectX12)**:
```json
{
  "backend": "dx12",
  "gpu_tier": "LowPower",
  "effect_intensity": 0.6
}
```
- Uses DXGI for memory management
- Native driver support (best compatibility)
- Target: 45 FPS @ 1080p

**Intel iGPU on Linux (Vulkan)**:
```json
{
  "backend": "vulkan",
  "gpu_tier": "LowPower",
  "effect_intensity": 0.6
}
```
- Uses Mesa 3D drivers
- Excellent modern graphics support
- Target: 45 FPS @ 1080p

**AMD APU (Radeon Graphics)**:
```json
{
  "backend": "auto",
  "gpu_tier": "LowPower",
  "effect_intensity": 0.7
}
```
- Better performance than Intel iGPU
- Can sustain 50 FPS with reduced effects
- Supports latest graphics features

---

## Troubleshooting GPU Issues

### Issue: GPU Not Detected

**Symptom**: Always uses "Software" tier

**Solution**:
1. Update graphics drivers
2. Check if GPU appears in system settings
3. Force backend selection:
   ```json
   {
     "backend": "vulkan"
   }
   ```
4. Check logs:
   ```bash
   RUST_LOG=debug cargo run 2>&1 | grep GPU
   ```

### Issue: High CPU Usage with iGPU

**Symptom**: CPU 80-90%, GPU 30-40%, audio stuttering

**Solution**:
1. Reduce particle count:
   ```json
   {
     "particle_count": 100
   }
   ```
2. Lower effect intensity:
   ```json
   {
     "effect_intensity": 0.4
   }
   ```
3. Enable transparency reduction:
   ```json
   {
     "reduce_transparency_effects": true
   }
   ```

### Issue: Stuttering on Animation

**Symptom**: Sidebar/panel transitions jank

**Solution**:
1. Check target FPS:
   ```json
   {
     "animation_fps_target": 40
   }
   ```
2. Disable transparency effects:
   ```json
   {
     "reduce_transparency_effects": true
   }
   ```
3. Lower overall effect intensity:
   ```json
   {
     "effect_intensity": 0.3
   }
   ```

### Issue: Snow Effect Too Slow

**Symptom**: letitsnow command causes FPS drop

**Solution**:
```json
{
  "particle_count": 100,
  "effect_intensity": 0.5
}
```

---

## Advanced Configuration

### Per-Profile GPU Settings

```json
{
  "profiles": {
    "gaming": {
      "gpu_tier": "HighPerformance",
      "effect_intensity": 1.0,
      "particle_count": 500,
      "animation_fps_target": 60
    },
    "productivity": {
      "gpu_tier": "LowPower",
      "effect_intensity": 0.6,
      "particle_count": 150,
      "animation_fps_target": 45
    },
    "battery_saving": {
      "gpu_tier": "LowPower",
      "effect_intensity": 0.2,
      "particle_count": 50,
      "animation_fps_target": 30
    }
  }
}
```

### Dynamic Adjustment (Coming in v2.0)

```json
{
  "dynamic_fps_scaling": true,
  "min_target_fps": 30,
  "max_target_fps": 60,
  "auto_adjust_effects": true,
  "frame_time_threshold_ms": 33.0
}
```

---

## Logging & Diagnostics

### Enable GPU Debug Logging

```bash
# Detailed GPU logs
RUST_LOG=hyprbrowser::gpu_detect=debug cargo run

# Full wgpu logs
RUST_LOG=wgpu=debug cargo run

# Performance monitoring
RUST_LOG=hyprbrowser::gpu_benchmark=info cargo run
```

### Benchmark Your GPU

1. Open DevTools (F12)
2. Run benchmark:
   ```bash
   cargo test -- gpu_benchmark --nocapture
   ```
3. Review results in console

---

## FAQ

**Q: Should I manually configure GPU settings?**  
A: No. Auto-detection is recommended. Only manual configure if you have performance issues.

**Q: Can I use iGPU settings on discrete GPU?**  
A: Yes, but you'll get worse performance. Keep discrete GPU at 100% intensity.

**Q: What if I have multiple GPUs?**  
A: HyprBrowser auto-selects the fastest. To use iGPU instead:
```json
{
  "gpu_tier": "LowPower"
}
```

**Q: Does lower effect intensity affect browser functionality?**  
A: No. All features work at any intensity. Only visual quality/smoothness changes.

**Q: How do I test different GPU tiers?**  
A: Use GPU_EXAMPLES.rs:
```bash
cargo test example_gpu_detection -- --nocapture
```

---

## Version Compatibility

- **v1.0.0**: iGPU optimization, auto-detection
- **v1.1.0** (planned): Per-profile GPU settings, dynamic FPS scaling
- **v2.0.0** (planned): DLSS/FSR, real-time GPU monitoring

---

**Last Updated**: December 2024
