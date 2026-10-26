# TEST_PLAN

## 1. Test image set (human supplies)

Put 15 to 20 photos in `test-images/` (git-ignored if any file is over 5 MB). Use your own photos or CC0/public-domain images only. Record source and license for each in `test-images/SOURCES.md`.

| Category | Count | Why |
|---|---|---|
| Portrait, natural light (various skin tones) | 3 | Skin tones, glow/grain look |
| Landscape, bright sky | 3 | Highlight roll-off, halos, clipping |
| Night / city lights | 3 | Bloom, halation, noise, grain |
| Flat / low-contrast / hazy | 2 | Tone tools, dehaze, curves |
| High-contrast backlit | 2 | Highlights, shadows, local tone mapping |
| Black and white or very desaturated | 1 | Grain, tone behavior |
| Pure test patterns (generated): black, white, gradient, hard edge, fine checkerboard | 5 | Identity, NaN, halo, aliasing checks |
| RAW files from at least 2 different cameras | 3 | Decode and color correctness |
| One 40MP+ file | 1 | Performance |

## 2. Automated checks (every effect)

- Strength/amount 0 is an exact identity.
- Deterministic for the same seed and settings.
- Monotonic response where applicable (more strength = more effect).
- Scale independence: render at full size and at 25% size; downscale the full render; difference below a set threshold.
- No NaN or Inf, no values outside the allowed range on black, white, gradient and noisy inputs.
- CPU vs GPU parity within the repo tolerance.
- Settings serialize and deserialize exactly, including inside sidecar files.
- Undo/redo restores previous state exactly.

## 3. Visual review checklist (human)

For each new effect, on at least 5 test photos at default and at 50% and 100% strength:

- [ ] Looks good at default, not harsh.
- [ ] No halos, banding, color shifts in neutrals, or blocky artifacts.
- [ ] Skin tones stay natural (glow, grain, looks).
- [ ] Highlights do not clip ugly.
- [ ] Effect feels the same on a small image and a big image.
- [ ] Sliders feel smooth, with a live preview responsive enough to drag.

## 4. UI checks

- [ ] Each tool panel: preset tiles work; main controls cover common edits; the Advanced section collapses, expands and remembers its state.
- [ ] Optional scrub input: drag up/down selects a parameter, left/right changes it; visible list and arrow keys work.
- [ ] Tool search (Ctrl+K) finds every tool by name.
- [ ] Hold-to-compare shows the original and returns cleanly.
- [ ] High-DPI and standard displays; window resize; dark theme legibility.
- [ ] Keyboard-only use is possible for core actions.

## 5. Performance targets (proposed, adjust after profiling)

| Task | Target |
|---|---|
| Open a 24MP RAW and show a first preview | under 3 s on a mid-range PC |
| Slider drag preview | feels interactive (aim for 30+ fps on a 24MP image with up to 5 effects active) |
| Full-resolution export, 24MP JPEG | under 10 s with typical edits |
| Memory, 50MP RAW with edits | stays within a reasonable fraction of the machine's RAM; no unbounded growth over a 30 minute session |

## 6. Regression

- Existing LightCraft tests continue to pass.
- A saved edit from before a change renders identically after it (process versions respected).
- A default build (without cargo feature `ai`) compiles and passes tests with no model files present.

## 7. Release checks (Windows)

- [ ] Installer installs on a clean Windows machine with no Rust or dev tools.
- [ ] Portable build runs from a USB-style folder.
- [ ] Graceful behavior with unsupported GPU/driver (falls back to CPU, shows a message, no crash).
- [ ] Uninstall removes the app and leaves user photos and edits alone.
- [ ] `THIRD_PARTY.md` matches the actual bundled dependencies.
- [ ] No network connections during normal use (verify with a network monitor).
