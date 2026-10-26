# PROMPTS (v2)

Copy-paste prompts for Freebuff (or any coding agent). One milestone per session (Freebuff sessions last about an hour). Milestone numbers match PROGRESS.md.

**Prompts 0 and 1 are unchanged from v1 and have already been run. Start at M02.**

**Design and UI work is parked for now.** Skip M17 and M18. New effects get their controls in the existing panels with existing widgets, and no prompt should restyle the interface.

## Before M02: update the repo's docs

1. Replace the files in your repo's `docs/product/` with this v2 set (DESIGN_DECISIONS.md is new).
2. Commit them on a branch named `docs-v2` and open a pull request. Merge it yourself after a quick look.
3. Keep any file describing another app's reverse engineering OUT of the repo (see RULES.md section 9).

## Which model to pick (check /model; rosters change)

- Hard math and GPU work (M04 to M08, M12 to M14): strongest model your allowance allows.
- Looks, colour, textures, tone tools, lifecycle (M09 to M11, M14 to M16, M19): mid-tier model.
- Renames, docs, cleanup: cheapest model.
- Avoid stealth or anonymous-provider models that retain prompts.
- Bake-off: give the same prompt (for example M04) to two models, run the tests, compare rendered images, keep the winner.

## Session header (paste before EVERY prompt)

```
Follow AGENTS.md and docs/product/RULES.md. Read docs/product/PROGRESS.md,
docs/product/DESIGN_DECISIONS.md and docs/product/EFFECTS_SPEC.md first. Work
ONLY on the milestone below and show a plan before editing files. Do not touch
unrelated files. Use a branch named after the milestone, never push to main, and
open a pull request when done (GitHub MCP). Run `cargo xtask ci` (or the closest
working subset on Windows) before finishing; say what could not run. For visual
work render before/after images into test-images/_renders/. At the end append to
PROGRESS.md: what changed, what is left, known issues, decisions needed from me.
Never read or copy anything from another app's decompiled or extracted files.
Do NOT redesign, restyle or restructure the UI: add any new controls using the
existing panels and widgets. Design work is a later phase.
```

---

## M02: Sync docs and re-create issues

```
Read docs/product/ (v2). Compare the milestone list in PROGRESS.md with the
GitHub issues created earlier (old numbering M01-M15, label "phase-1"). Using the
GitHub MCP: close or relabel the old issues as "superseded", then create one
issue per milestone M03-M22 from PROGRESS.md with a one-paragraph description
copied from the matching prompt in PROMPTS.md. Do not change repo settings or
visibility. List what you created. Update FEATURES.md "inherited (verify)"
statuses only if you have actually verified them.
```

## M03: Stub effect proves the path

```
Add an effects section "Glow" with one slider glow.amount (0 to 100, default 0),
wired end to end like grain.amount: control spec, settings, CPU pipeline, command
registry, UI panel, sidecar. Placeholder CPU math is fine. Existing renders must
stay pixel-identical (respect process versions). Tests: default is identity;
settings round-trip; undo/redo works.
```

## M04: Glow (distinct from Bloom)

```
Implement Glow exactly as in EFFECTS_SPEC.md section 1, CPU reference only, in
linear light: multi-scale, edge-aware softened layer weighted by luminance, mixed
with the original while keeping some fine contrast. Controls: Glow, Saturation,
Warmth, plus 6 original style tiles. Radii are a fraction of the long edge.
Tests from the spec (identity at 0, scale independence, Saturation/Warmth work at
low Glow, no NaN). Render before/after on 5 test photos including 2 portraits and
tell me which to look at. Tune defaults by eye: useful at low values, never milky.
Update the spec with final ranges.
```

## M05: Bloom

```
Implement Bloom per EFFECTS_SPEC.md section 2, CPU only: bright-pass with a soft
tonal curve driven by Range, multi-scale spread driven by Size, scaled by
Strength, composited with highlight-preserving blending. Bloom must look clearly
different from Glow at defaults. Tests: Size enlarges the halo footprint (not just
opacity), Range changes which tones bloom, dark regions without bright detail get
no halo, thin bright edges bloom smoothly. Render night and backlit photos.
```

## M06: Halation

```
Implement Halation per EFFECTS_SPEC.md section 3, reusing the blur helper but
NOT as a tinted Bloom: the warm fringe sits around bright edges (subtract part of
the source footprint from the blurred, tinted layer). Controls: Threshold,
Strength, Radius, Tint. Tests: lowering Threshold changes which sources qualify;
Strength does not change eligibility; flat midtones are not uniformly tinted;
all-white and all-black give no NaN or out-of-range output. Render night photos.
```

## M07: GPU parity

```
Port Glow, Bloom and Halation to WGSL following docs/gpu-pipeline.md. GPU output
must match the CPU reference within the repo's parity tolerance using the existing
parity tests; fall back to CPU on GPU errors without crashing. If parity cannot be
reached, fix the GPU path; do not change CPU behavior or loosen tolerance without
explaining why and asking me. Report preview frame times on a 24MP image with and
without the effects.
```

## M08: Film grain upgrade

```
Extend the existing grain in crates/pipeline/src/finish.rs per EFFECTS_SPEC.md
section 4: luminance-dependent amplitude, independent Style Strength (Style 0 keeps
grain; Grain 0 keeps the look), monochrome vs chroma toggle. Add the new controls
to the existing grain panel using the existing widgets and layout (no redesign).
Keep existing renders identical at defaults (use a process version if behavior must
change). CPU + GPU with parity tests. Tests: deterministic per seed; identity at
amount 0; changing size does not change average brightness; no tiling or banding on
flat skies.
```

## M09: Colour Grading (merge HSL and split toning)

```
Implement EFFECTS_SPEC.md section 10. Merge the HSL mixer and split toning into ONE
panel named "Colour Grading" (UI text uses British spelling "Colour"; code
identifiers keep the existing `color` spelling). Use the existing panel style and
widgets; do not redesign anything. Tabs: Mixer (existing 8-band hue/saturation/
luminance with drag-on-photo targeting), Wheels (shadows, midtones, highlights,
global with hue, saturation, luminance, plus Blending and Balance), Moods (6 to 10
original colour presets with an Amount slider). If a separate split-toning control
exists, migrate it into Wheels with a process-version-safe mapping so old edits
render pixel-identically and old sidecars still load; add a test with a saved old
edit. Tests: identity at defaults; no spill into adjacent hues or skin tones at
moderate values; settings round-trip. Do not remove the engine's existing parameters.
```

## M10: Film looks and Looks system

```
Implement EFFECTS_SPEC.md section 5 and the Looks rules in DESIGN_DECISIONS.md.
Generate 12 original looks procedurally from our own curves and color matrices
(no data from any commercial film, no brand names). A Look stores an ordered set
of global parameter values, NOT a flattened image, and by default excludes local
masks, crop and healing. Apply as ONE undoable step with an Amount slider; users
can save their own looks; applying a Look to many photos never carries local masks
silently and shows what was skipped. Move look application to the GPU if
feasible, otherwise document the limit. Tests: Amount 0 identity; save/apply
round-trip exact; one undo step. Render all 12 looks on 5 photos for my review.
```

## M11: Vintage texture

```
Implement EFFECTS_SPEC.md section 6: seeded procedural light leaks and dust/
scratches, a Randomize command, works independently of any look. CPU + GPU with
parity tests. Tests: same seed identical; different seed differs; amounts 0
identity; scale independent.
```

## M12: Lens blur

```
Implement EFFECTS_SPEC.md section 7 with an interactive focus shape (elliptical or
linear band; move, resize, rotate), Blur Strength, Transition, bokeh shape (circle
or hexagon), Vignette Strength. Interaction: with a handle selected pinch resizes
it; tap elsewhere to deselect and zoom normally. Store geometry in image
coordinates so it stays put on zoom. Tests: focus area stays sharp; Transition
changes only boundary width; Vignette 0 keeps blur but removes darkening; no
banding; scale independent. Keep preview interactive on a 24MP image (fast preview
while dragging, full quality on release). CPU + GPU with parity tests.
```

## M13: Motion blur

```
Implement EFFECTS_SPEC.md section 9 with NO AI: directional blur with Angle,
Speed, Acceleration (shapes the trail, not just opacity), a region from our
existing mask tools, 4 original presets, and a panning mode (region stays sharp,
outside blurs). Avoid cut-out halos at the region edge. Tests: identity at Speed
0; blur follows Angle; Acceleration changes trail shape; scale independent. CPU +
GPU with parity tests. Do not add subject isolation (Phase 3).
```

## M14: Tone tools

```
Implement EFFECTS_SPEC.md section 8 (Punch, Expanse, Tone Zones) using the
existing local-Laplacian code, 4 preset styles each. CPU + GPU with parity tests.
Tests: identity at 0; no halos on a hard-edge image; neutrals stay neutral. Render
all presets on 5 test photos for my review.
```

## M15: Black & White and Frames

```
Implement EFFECTS_SPEC.md sections 11 and 12. Black & White: colour-filter mixing
with presets, tone controls, optional grain (shares the Film Grain code), film-style
tone curve. Frames: 4 to 6 original styles, width, colour, option to overlay or
expand canvas, always applied after all other effects. Tests: colour filter changes
the grey mapping of a colour-chart image; identity when off; frame never blurred
even with Lens Blur on; export dimensions match the chosen behavior.
```

## M16: Tool lifecycle and edit stack (behavior only)

```
Implement the tool lifecycle in DESIGN_DECISIONS.md as engine behavior: a tool opens
with live preview, then Apply (ONE history item per tool, not per pointer move) or
Cancel (discard only that tool's uncommitted changes); an applied tool can be
reopened with its parameters and mask/geometry restored, toggled on/off, or deleted.
Expose all of this through the command registry. For the UI, use a plain list with
existing widgets; do NOT style, redesign or restructure the interface (design work
comes later). Keep the engine's canonical processing order (DESIGN_DECISIONS.md,
"Processing order"); do not add reordering. Keep linear undo/redo. Tests: one undo
entry per applied tool; cancel leaves no history; reopen restores values; toggle off
equals not applied.
```

## M17: (Parked) Interface foundation

Skipped for now. Design and UI work is a later phase; the original prompt is kept under "Parked: design phase" at the end of this file.

## M18: (Parked) Panel design pass

Skipped for now. See "Parked: design phase" at the end of this file.

## M19: Spot and brush heal (legal review first)

```
First, STOP and write me a short note: tap-to-place local adjustments descend from
a patented control-point idea; I need a patent attorney to review before release.
Then, if I say to continue, build both behind a setting I can turn off:
(a) Spot: tap to place a radial mask with a colour- and luminance-aware falloff and
brightness, contrast, saturation sliders, built on LightCraft's mask system;
(b) brush heal as an additional healing method (keep the existing spot heal with its
adjustable source). Use existing widgets; no redesign. Store everything in image
coordinates. Tests: points stay anchored on zoom; undo restores exactly; no network
calls.
```

## M20: Verify inherited features on Windows

```
Walk through every "inherited" item in FEATURES.md on Windows with the test image
set: RAW/JPEG/PNG/HEIC import, curves, white balance (including a neutral-point
picker and auto), calibration, lens corrections, healing, non-AI masks, library,
batch export, sidecars, soft-proofing, keymap. For each: works / partial / broken.
Fix what is small; for large gaps create a GitHub issue and stop. Update FEATURES.md.
```

## M21: Windows packaging and performance

```
Build a Windows installer and a portable zip. Profile with a 24MP and a 40MP+ RAW:
report open time, preview frame times, export time and peak memory against
TEST_PLAN.md section 5. Fix the top three bottlenecks. Verify graceful CPU fallback
on unsupported GPU/driver. Generate THIRD_PARTY.md from the real dependency tree
(licenses included) and bundle it. Verify no network connections in normal use.
```

## M22: Rebrand, QA, release candidate

```
Rename the app to <YOUR NAME> everywhere (binary, window title, installer, README,
About). Remove ArtCraft names and logos but keep NOTICE, license files and
copyright. Run the full TEST_PLAN.md on every test image. Produce a known-issues
list and release checklist. List every file changed. Open a pull request and a
draft GitHub release (do not publish).
```

---

## Parked: design phase (do not run yet)

When design work starts, revisit these. They were written for the single-mode interface in DESIGN_DECISIONS.md and may be revised first.

### Interface foundation (was M17)

```
Build the single-mode editing interface described in DESIGN_DECISIONS.md "Layout".
There is NO Simple/Pro toggle. Create: a start screen (recent files grid, Add
Photo, drag-and-drop, Quick Open with no catalog import, clear states for loading,
missing file and no permission); an edit view with a tool rail and one panel at a
time; tool search (Ctrl+K); a one-line description and "?" tip per tool; the
optional scrub input (drag up/down selects a parameter, left/right changes it,
visible parameter list and arrow keys always work; a setting turns it off). Layout
and look are OUR OWN: do not copy another app's grid, icons or colours. Share the
engine's command registry (no duplicated logic; respect layer rules). Headless
snapshot test. Describe how mouse, trackpad and touch input are handled.
```

### Panel design pass (was M18)

```
For every tool built so far (Tune, Detail, White Balance, Colour Grading, Looks,
Glow, Bloom, Halation, Film Grain, Vintage Texture, Lens Blur, Motion Blur, Punch,
Expanse, Tone Zones, Black & White, Frames, crop/straighten, vignette, healing)
give the panel this structure: preset tiles with live thumbnails (where the tool
has presets), a short row of the most-used controls, and a collapsible Advanced
section (collapsed by default, state remembered per tool) holding everything else.
Every parameter must exist exactly once: Basic vs. Advanced only changes
visibility. Use the Tune and Detail arrangement in DESIGN_DECISIONS.md (new
"Richness" and "Structure" are real engine parameters, not aliases). Add Auto to
Tune, hold-to-compare and split view, and a Save and Share button with an "Export
options" link to the full dialog. Tests: Richness helps a flat and a backlit
image without just brightening; Structure adds texture without edge halos while
Sharpen does not; no parameter appears twice; Advanced state persists.
```


## Later phases (do not run during Phase 1)

**Phase 2: Free/Pro tiers**
```
Tag every feature Free or Pro per the "Intended tier" column in FEATURES.md. Add an
engine-level capability check and an offline signed license key (public-key
verification, no network, private key never in the repo). Paid-tier features are visible
with a short description but no pop-ups or countdowns ("Pro" is only the paid tier,
not an interface mode). Free features never degrade.
Edits made with Pro features stay viewable in Free. Tests for key verification,
tamper detection and gating.
```

**Phase 3: AI features (optional)**
```
Before writing code, produce a report on candidate AI features (smart suggestions,
portrait tools, subject/sky masks, motion-blur subject isolation, denoise): model
options, license, download size, accuracy, hardware needs, privacy impact.
Recommend what to ship and what to skip. Stop and wait for my decision.
```

## Habits that keep agents on track

- If an agent loops on the same compile error three times, start a fresh session with only the error and the one relevant file.
- Look at every visual result yourself. Tests cannot tell you if a glow is ugly.
- Never paste anything from another app's decompiled or extracted files into a prompt.
- Merge pull requests yourself after reviewing.
