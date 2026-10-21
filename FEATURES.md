# FEATURES

**Phase 1 builds everything below with all features unlocked and no AI.** The "Intended tier" column is only a note for Phase 2 (free vs. Pro). Do not implement gating in Phase 1.

"Status" values: `inherited` (already in LightCraft, verify it works on Windows), `new` (to build), `later` (not Phase 1). The agent must verify every `inherited` item in the repo and update this file; do not assume.

## Editing core

| Feature | Source | Intended tier | Phase |
|---|---|---|---|
| RAW, JPEG, PNG, HEIC import | inherited (verify) | Free | 1 |
| Non-destructive edit stack, undo/redo | inherited (verify) | Free | 1 |
| Crop, rotate, straighten, perspective | inherited (verify) | Free | 1 |
| Basic: exposure, contrast, highlights, shadows, whites, blacks, white balance, saturation, vibrance, sharpening | inherited (verify) | Free | 1 |
| Tone curve (RGB and per channel) | inherited (verify) | Free | 1 |
| HSL / color mixer, color grading | inherited (verify) | Free | 1 |
| Calibration and color profiles | inherited (verify) | Free | 1 |
| Dehaze, clarity, texture, local contrast | inherited (verify) | Free | 1 |
| Lens corrections and chromatic aberration (profile based, user supplies lens data) | inherited (verify) | Free | 1 |
| Vignette | inherited (verify) | Free | 1 |
| Selective adjust, brush, gradients (non-AI masks) | inherited (verify) | Free | 1 |
| Spot healing and blemish removal (non-AI patch based) | inherited (verify) | Free | 1 |
| JPEG and PNG export, full resolution | inherited (verify) | Free | 1 |

## New aesthetic effects (original implementations)

| Feature | Source | Intended tier | Phase |
|---|---|---|---|
| Glow (soft highlight glow) | new | Free | 1 |
| Bloom | new | Free | 1 |
| Halation | new | Free | 1 |
| Film grain with luminance response and style strength | new (extends existing grain) | Free | 1 |
| 12 original film looks plus Looks system (apply, strength, save your own) | new | Free (3 saved) / Pro (unlimited) | 1 |
| Vintage texture: light leaks and dust/scratches | new | Free | 1 |
| Lens blur (linear and elliptical focus, bokeh shapes) | new | Free | 1 |
| Tone tools: Punch, Expanse, Tone Zones (working names) | new | Free | 1 |

## Library and workflow

| Feature | Source | Intended tier | Phase |
|---|---|---|---|
| Single-folder browse, ratings, flags, filmstrip | inherited (verify) | Free | 1 |
| Multi-folder catalog, keywords, collections | inherited (verify) | Pro | 1 |
| Batch edit, copy/paste and sync settings | inherited (verify) | Pro | 1 |
| Preset create, import, export | inherited (verify) | Pro | 1 |
| Batch export with naming templates and resize | inherited (verify) | Pro | 1 |
| Soft-proofing, export color profiles | inherited (verify) | Pro | 1 |
| Custom keyboard shortcuts | inherited (verify) | Pro | 1 |
| XMP-compatible sidecar files | inherited (verify) | Pro | 1 |

## Simple mode (new UI)

| Feature | Source | Intended tier | Phase |
|---|---|---|---|
| Simple view: large photo, tool carousel, preset tiles | new | Free | 1 |
| Drag up/down to select parameter, left/right to change | new | Free | 1 |
| Visible parameter list and keyboard fallbacks | new | Free | 1 |
| Hold-to-compare before/after | new | Free | 1 |
| Toggle between Simple and Pro on the same photo | new | Free | 1 |

## Platform

| Feature | Source | Phase |
|---|---|---|
| Windows installer and portable build | new | 1 |
| GPU fallback to CPU when unsupported | verify | 1 |
| Bundled third-party license file | new | 1 |

## Not in Phase 1

| Feature | Phase |
|---|---|
| Free/Pro gating and license keys | 2 |
| Smart look suggestions (analysis based) | 3 |
| Portrait and face tools | 3 |
| AI subject, sky and person masks | 3 |
| AI denoise and model-based object removal | 3 |
| Cloud sync or accounts | not planned |
