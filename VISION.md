# VISION

Working name: **<YOUR NAME>** (candidates: Latent, Emulsion, Grainline, Tonal, Stackroom; confirm trademark, domain and app-store availability before committing).

## One line

A Windows-first desktop photo editor with the friendliness of a mobile "tap, swipe, done" editor and the depth of a professional RAW developer, built on the open-source LightCraft engine (MIT OR Apache-2.0).

## Who it is for

1. **Casual and enthusiast shooters** who want great-looking photos in a few minutes without learning a pro tool.
2. **Serious hobbyists and pros** who want precise, non-destructive control, a library, and batch workflow, without a monthly subscription.

## Principles

1. **One interface, layered depth.** There is no Simple or Pro mode. Every tool has one panel with preset tiles and a short row of main controls, plus a collapsible Advanced section with everything else. Every parameter exists once.
2. **Friendly by design.** Big preset tiles with live previews, plain names, tool search, hold-to-compare, an optional scrub gesture (drag up/down to pick a parameter, left/right to change it), and a visible parameter list and keyboard fallbacks.
3. **Non-destructive always.** Edits are parameters, never baked pixels. Originals are never modified.
4. **Local and private.** All processing happens on the user's machine. No telemetry. No network calls without explicit approval.
5. **Honest and calm.** No nag screens, no countdowns, no dark patterns.
6. **Original work.** Original implementations, original names, original assets. See RULES.md.

## Scope by phase

### Phase 1 (current): the full app, no AI, no tiers
Build every feature in FEATURES.md with everything unlocked. No free/Pro gating, no license keys, no AI features. Goal: a complete, polished, stable editor.

- Existing LightCraft AI-dependent code (model-based segmentation, AI denoise, model-based object removal) is placed behind a cargo feature `ai`, **off by default**, and no model weights are downloaded or bundled.
- New effects: glow, bloom, halation, film grain with style strength, original film looks, vintage texture (light leaks and dust), lens blur, three tone tools.
- Friendly single-mode interface (tool rail, preset tiles, Basic/Advanced panels, start screen, Quick Open) and Looks system.
- Windows installer and performance pass.

### Phase 2 (later): free and Pro tiers
Tag features per FEATURES.md "Intended tier" column, add an offline signed license key and a capability check. Not part of Phase 1. Do not add gating code early.

### Phase 3 (optional, later): AI features
Smart suggestions, portrait/face tools, subject and sky masks, AI denoise. Only after Phase 1 and 2 ship, and only after reviewing model licenses, size and privacy. Not part of Phase 1.

## Commercial direction (for later)

Free core that is genuinely good, plus a one-time Pro purchase (price to be tested, likely $79 to $99), no subscription. Revenue ideas after launch: Pro license, creator preset packs.

## Definition of done for Phase 1

- Every feature in FEATURES.md marked "Phase 1" is implemented, tested, and documented.
- `cargo xtask ci` (or its Windows-capable equivalent) passes.
- All TEST_PLAN.md checks pass on the full test image set.
- A Windows installer builds and installs cleanly on a machine without Rust.
- Third-party licenses are bundled and accurate.
