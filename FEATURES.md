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
| Colour Grading panel: Mixer (HSL) + Wheels + Moods, merged; legacy split toning migrated | inherited (verify) + UI change | Free | 1 |
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
| Motion blur (directional, no AI) | new | Free | 1 |
| Black & White with colour filters | new | Free | 1 |
| Frames (applied last) | new | Free | 1 |

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

## Interface (single full mode)

| Feature | Source | Intended tier | Phase |
|---|---|---|---|
| Edit view: large photo, tool rail, one panel at a time | new | Free | 1 |
| Every tool panel: preset tiles plus main controls on top, collapsible Advanced section | new | Free | 1 |
| Optional scrub input (drag up/down picks a parameter, left/right changes it), with visible list and arrow keys | new | Free | 1 |
| Tool search (Ctrl+K) and one-line descriptions with "?" tips | new | Free | 1 |
| Hold-to-compare and split view | new | Free | 1 |
| Tool lifecycle: Apply / Cancel, one history item per tool, reopen from a visible stack | new | Free | 1 |
| Start screen: recent files grid, Add Photo, drag-and-drop, Quick Open (no import needed) | new | Free | 1 |
| Tap-to-place Spot and brush heal (needs legal review, see DESIGN_DECISIONS) | new | Free | 1 |

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
| Double Exposure (blend modes) | after release candidate |
| Text overlay | after release candidate |
| Look sharing as portable file or QR | 2 or later |
| Motion blur subject isolation | 3 |
| Cloud sync or accounts | not planned |
