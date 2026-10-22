# FEATURES

**Phase 1 builds everything below with all features unlocked and no AI.** The "Intended tier" column is only a note for Phase 2 (free vs. Pro). Do not implement gating in Phase 1.

"Source" values: `inherited (verified: works / partial)` (already in LightCraft, checked against the code in the M00 session — see the verification table at the end), `new` (to build), `later` (not Phase 1). Hands-on checks with real files on Windows are milestone M13; the agent must keep this file honest, do not assume.

## Editing core

| Feature | Source | Intended tier | Phase |
|---|---|---|---|
| RAW, JPEG, PNG, HEIC import | inherited (verified: partial) | Free | 1 |
| Non-destructive edit stack, undo/redo | inherited (verified: works) | Free | 1 |
| Crop, rotate, straighten, perspective | inherited (verified: works) | Free | 1 |
| Basic: exposure, contrast, highlights, shadows, whites, blacks, white balance, saturation, vibrance, sharpening | inherited (verified: works) | Free | 1 |
| Tone curve (RGB and per channel) | inherited (verified: works) | Free | 1 |
| HSL / color mixer, color grading | inherited (verified: works) | Free | 1 |
| Calibration and color profiles | inherited (verified: partial) | Free | 1 |
| Dehaze, clarity, texture, local contrast | inherited (verified: works) | Free | 1 |
| Lens corrections and chromatic aberration (profile based, user supplies lens data) | inherited (verified: works) | Free | 1 |
| Vignette | inherited (verified: works) | Free | 1 |
| Selective adjust, brush, gradients (non-AI masks) | inherited (verified: works) | Free | 1 |
| Spot healing and blemish removal (non-AI patch based) | inherited (verified: works) | Free | 1 |
| JPEG and PNG export, full resolution | inherited (verified: works) | Free | 1 |

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
| Single-folder browse, ratings, flags, filmstrip | inherited (verified: works) | Free | 1 |
| Multi-folder catalog, keywords, collections | inherited (verified: works) | Pro | 1 |
| Batch edit, copy/paste and sync settings | inherited (verified: works) | Pro | 1 |
| Preset create, import, export | inherited (verified: works) | Pro | 1 |
| Batch export with naming templates and resize | inherited (verified: works) | Pro | 1 |
| Soft-proofing, export color profiles | inherited (verified: works) | Pro | 1 |
| Custom keyboard shortcuts | inherited (verified: works) | Pro | 1 |
| XMP-compatible sidecar files | inherited (verified: works) | Pro | 1 |

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
| GPU fallback to CPU when unsupported | inherited (verified: works) | 1 |
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

## Inherited feature verification (M00 session, 2026-10-21)

Statuses are verified at code + test level. `works` = implemented end to end with passing tests in the repo;
`partial` = works with the caveat in the evidence. Hands-on checks with real files on Windows are M13, and the
per-camera colour calibration gap is tracked for M13.

| Row | Status | Evidence (code + tests) |
|---|---|---|
| RAW/JPEG/PNG/HEIC import | partial | `crates/raw`, `crates/codecs`, `library.import` (add/copy/move, `engine/src/cmd/import.rs`). HEIC needs the codecs `heif` cargo feature (`codecs/src/heif.rs`, `codecs/tests/heif.rs`), else a clear error; AVIF not decoded. `engine/src/tests_import.rs` |
| Non-destructive edit stack, undo/redo | works | `edit.undo/redo` (`engine/src/cmd/edit.rs`), `history.list/restore/previous/next/clear`, journal `Op` (`catalog/src/journal.rs`); `tests_history_cursor.rs` |
| Crop, rotate, straighten, perspective | works | `crop.set/drag/angle/rotateAspect`, `photo.rotateLeft/Right`, Upright levels; `geometry.rotate` in `engine/src/crs.rs`; `tests_crop_angle.rs` |
| Basic sliders | works | `light.*` incl. whites/blacks, `wb.temp/tint`, `color.saturation/vibrance`, `detail.sharpen*` (`develop/src/controls.rs`) |
| Tone curve RGB + per channel | works | `curve.*` parametric + per-channel LUTs (`pipeline/src/finish.rs`), `cmd/curves.rs`, `pointColor.*` |
| HSL / color mixer, color grading | works | `mixer.*` 8 bands x h/s/l, `grading.*`, `bw_mix.*` (`develop/src/controls.rs`) |
| Calibration and color profiles | partial | `calibration.*` + `calibration_matrix` (`pipeline/src/colorops.rs`) + LUT profiles, but per-camera colour calibration is a listed gap |
| Dehaze, clarity, texture, local contrast | works | `effects.texture/clarity/dehaze` + local versions through masks (`engine/src/crs_masks.rs`) |
| Lens corrections and CA | works | `optics.*` incl. profile distortion/vignetting, `AutoLateralCA` -> `remove_ca`, defringe (`engine/src/crs.rs`); `crates/lensdb` (user-supplied data); `lens_profiles.rs` |
| Vignette | works | `vignette.*` with 3 styles (`ui-egui/src/panels/edit.rs`); crs `PostCropVignette*` |
| Selective adjust, brush, gradients | works | `mask.add`/`mask.addComponent` (`engine/src/cmd/masks.rs`) brush/linear/radial/luminance/colour-range + mask effects; AI shape suggestions separate |
| Spot healing and blemish removal | works | spot commands (`engine/src/cmd/masks.rs`), heal stage; `spot.findBlemishes` additionally needs the YuNet download (`engine/src/cmd/face_detect.rs`); `tests_spots.rs` |
| JPEG and PNG export | works | `app.export` (`engine/src/cmd/export.rs`), `ExportFormat::{Jpeg,Png,Original,Dng}`; `tests_export.rs` |
| Browse, ratings, flags, filmstrip | works | `library.browse`, `photo.rate/flag/label`, `view.filmstrip`; `tests_filmstrip.rs`, `tests_scroll.rs` |
| Multi-folder catalog, keywords, collections | works | `folder.*`, `album.create/createSmart/setRules` (albums + smart albums), `keyword.*`; `tests_folders.rs`, `tests_organize.rs` |
| Batch edit, copy/paste and sync settings | works | `develop.copy/paste/pastePrevious` + Sync Settings (`engine/src/cmd/develop.rs`), multi-`ids` |
| Preset create, import, export | works | `preset.create/apply/delete/rename/move/update`, `preset.import/export` (`cmd/preset_files.rs`), XMP/lrtemplate importers |
| Batch export, naming templates, resize | works | `app.export` naming shares `photo.renameTokens` tokens (`cmd/manage.rs`), `Resize`/`ResizeMode`, export presets |
| Soft-proofing, export color profiles | works | `view.softProof` (S) srgb/displayP3/adobeRgb/proPhoto/rec2020 + custom ICC + intent + paper/ink (`ui-egui/src/menus.rs:903`, `panels/edit.rs`); `tests_masking.rs:1368`, `pipeline/src/tests.rs` |
| Custom keyboard shortcuts | works | editable keymap (Help > Keyboard Shortcuts), `app.setShortcut/resetShortcuts`, saved in `ui.json`; `tests_keymap.rs` |
| XMP-compatible sidecar files | works | `engine/src/sidecar/` (`sidecar_packet`, `parse_sidecar`, `SidecarNaming`) + two-way `crs:` mapping (`crs.rs`); `tests_xmp.rs` |
| GPU fallback to CPU | works | `try_render` -> `Fallback`, wgpu error scopes, crash sentinel, black/incomplete re-render (docs/gpu-pipeline.md); `crates/gpu/tests/fallback.rs` |
