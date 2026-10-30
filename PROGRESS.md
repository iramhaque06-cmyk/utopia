# PROGRESS

The agent reads this at the start of every session and appends to it at the end. This file is the project's memory because the agent forgets between sessions.

## Human decisions (fill in)

| Decision | Value |
|---|---|
| App name | TBD |
| GitHub repo (fork of storytold/lightcraft) | `iramhaque06-cmyk/utopia` |
| Target Windows version and GPU | Windows 11 Home (build 26200), Intel Iris Xe (this machine) |
| Test images supplied | no |
| Price / tiers | deferred to Phase 2 |

## Milestones (Phase 1: full app, no AI, no tiers)

Status: `[ ]` not started, `[~]` in progress, `[x]` done and human-approved.

Renumbered 2026-10-26 to match PROMPTS.md (v2); GitHub issues #20-#39 track M03-M22.

- [~] M00 Orientation, toolchain check, repo and GitHub setup (PR #17)
- [~] M01 AI quarantine behind cargo feature `ai` (default off) (PR #18)
- [~] M02 Sync docs and re-create issues (issues done; docs-v2 replacement pending the v2 file set)
- [~] M03 Stub effect proves the path (PR #19; branch `m02-stub-effect`, named from the v1 prompt label)
- [ ] M04 Glow (distinct from Bloom)
- [ ] M05 Bloom
- [ ] M06 Halation
- [ ] M07 GPU parity
- [ ] M08 Film grain upgrade
- [ ] M09 Colour Grading (merge HSL and split toning)
- [ ] M10 Film looks and Looks system
- [ ] M11 Vintage texture
- [ ] M12 Lens blur
- [ ] M13 Motion blur
- [ ] M14 Tone tools
- [ ] M15 Black & White and Frames
- [ ] M16 Tool lifecycle and edit stack (behavior only)
- [ ] M17 Interface foundation (PARKED - design phase)
- [ ] M18 Panel design pass (PARKED - design phase)
- [ ] M19 Spot and brush heal (legal review first)
- [ ] M20 Verify inherited features on Windows
- [ ] M21 Windows packaging and performance
- [ ] M22 Rebrand, QA, release candidate

## Phase 2 (later, not started)

- [ ] P2-1 Free/Pro tagging, capability check, offline signed license key

## Phase 3 (optional, not started)

- [ ] P3-1 Evaluate AI features (models, licenses, privacy) and decide

## Session log

Append newest at the bottom. Format:

```
### YYYY-MM-DD  Session N  (model used, milestone)
- Changed:
- Tests run / results:
- Left to do:
- Known issues:
- Decisions needed from human:
```

### 2026-10-26  Session 4  (Buffy / Codebuff, v2 M02 issue sync + v2 M03 verification)
- Changed: (1) Finished the v2 M03 stub-effect work (branch `m02-stub-effect`, commit `9c16ec49`, PR #19 - named "M02" from the v1 prompt label; PROMPTS.md v2 calls it M03). (2) GitHub PRs: PR #18 (M01, opened by the human with an empty body) - filled in its full description; PR #19 (stub effect) created with summary, files, tests and known issues. Both through the GitHub REST API with this repo's git credential because the MCP token lacks `pull-requests: write` ("Resource not accessible by personal access token"); the MCP also denies `update_issue` while `create_issue` works. (3) v2 M02 issue sync with the human-provided PROMPTS.md: created issues #20-#39 = M03-M22, one per milestone, bodies copied from the matching prompt, label `phase-1` (M03 #20 carries a note that it is already implemented in PR #19; M17/M18 #34/#35 marked parked; M19 #36 carries the patent-review warning). Old issues #3-#16 relabelled `superseded` (label created) and closed with per-issue mapping comments (old M03 Glow+Bloom splits into M04 #21 + M05 #22; old M11/M12 Simple mode maps to parked M17 #34 / M18 #35); old #1/#2 closed as completed (PRs #17/#18; v2 keeps M00/M01 unchanged). No repo settings or visibility changed. (4) PROGRESS.md milestone list renumbered to PROMPTS.md v2. FEATURES.md untouched: its `inherited (verify)` rows were all verified with per-row evidence in Session 1, so there was nothing unverified to update.
- Tests run / results: ui-egui full suite 1084 passed / 13 failed / 4 ignored; the 13 are the known background-settle/timing family (before_after x4, labels x2, slideshow x1, preview_limit x5) and all pass under reduced concurrency (7 pass with `--test-threads=1`, the 5 preview_limit pass in isolation) - no M02 regressions. Glow tests 5/5 (pipeline 2, engine 3); fmt and clippy green (Session 3). NOT run here: full `cargo xtask ci` (its wasm step needs toolchain extras on this Windows box).
- Left to do: the PROMPTS.md "Before M02" docs step - replace `docs/product/` with the v2 file set (DESIGN_DECISIONS.md is new), branch `docs-v2`, PR, self-merge - is blocked: only PROMPTS.md was provided and this repo has no `docs/product/` (product docs live at the repo root). Human review of PRs #18 and #19 including the RULES section 8 visual check of the Glow slider.
- Known issues: the ui-egui settle/thumbnail/slideshow tests fail under harness parallelism on this machine (a different subset per run); run those families with `--test-threads=1` or accept a rerun. The GitHub MCP token can create issues but not PRs or issue updates - the REST fallback used the human's own git credential (token never printed or stored). Carried from M01: paging-file exhaustion under parallel rustc (`-j 2` needed); no `CRAFT_FONTS_DIR`; empty `corpus/`.
- Decisions needed from human: (1) merge PRs #18 and #19; (2) provide the v2 docs set (PROGRESS.md, RULES.md, DESIGN_DECISIONS.md, EFFECTS_SPEC.md ...) for the docs-v2 step, or confirm the root-level docs should be updated in place; (3) M01 carry-overs still open (app name for M22; grain retune vs pixel-identity for M08; test photos for the render tasks).

### 2026-10-22  Session 3  (Buffy / Codebuff, M02)
- Changed: M02 stub effect "Glow" (`glow.amount` 0–100, default 0) wired end to end exactly like `grain.amount`: `crates/develop/src/settings.rs` (new `Glow` struct + field on `DevelopSettings`, `skip_serializing_if` at its default so existing settings JSON and `hash64` are byte-identical), `crates/develop/src/controls.rs` (`Section::Glow` + control spec `glow.amount`), `crates/pipeline/src/finish.rs` (`FinishParams::glow` + linear-light placeholder bright-pass lift — pointwise, stands in for the threshold + wide blur + screen blend of EFFECTS_SPEC §1; `None` at amount 0 so the pass is an exact identity and existing renders stay pixel-identical), `crates/ui-egui/src/panels/edit.rs` ("Glow" sub-title + slider in the Effects panel), `crates/engine/src/cmd/develop.rs` (`glow` in the `develop.resetSection` section list). Sidecar round-trip is automatic via `lc:settings`. Tests: `crates/pipeline/src/tests.rs` (`glow_at_zero_is_an_exact_identity_and_at_strength_changes_the_render`), `crates/engine/src/tests_glow.rs` (settings JSON + XMP sidecar round-trip, one undo step with undo/redo, `develop.resetSection {section: glow}`). Also `cargo fmt --all` formatting-only fixes to three M01 files (`cmd/masks.rs`, `cmd/mod.rs`, `tests_export.rs`) that were committed unformatted. Branch `m02-stub-effect`.
- Tests run / results: glow tests 5/5 green (pipeline 2, engine 3); `cargo fmt --check` ok; `cargo clippy --workspace --all-targets -- -D warnings` ok. Full suites of the touched crates (`lightcraft-develop`, `lightcraft-pipeline`, `lightcraft-engine`, `lightcraft-ui-egui`, app `--test '*'`) run per package — see the next entry / PR for the final counts. NOT run on Windows here: `cargo xtask ci` in full (its `wasm` step needs the wasm toolchain extras; the per-package suites are the closest working subset — same policy as M01).
- Left to do: human review that the Glow slider renders correctly in the Effects panel (RULES §8 visual check); real Glow math is M03.
- Known issues: none new from M02. Carried from M01: `cargo test --workspace` in one target dir after a clippy run hits spurious `E0462/E0463 rlib format` errors (clippy/test cache thrash on the shared `D:/rust/target`); per-package runs are reliable — give clippy its own `CARGO_TARGET_DIR` or run it after tests. PR creation via GitHub MCP is blocked by the token's missing `pull-requests: write` scope (branch is pushed; PR body drafted in chat).
- Decisions needed from human: (1) approve the M02 PR (and M01's, still open); (2) the issue task: the requested `PROMPTS.md` and the "docs/product/ (v2)" tree do not exist in this repo (docs live at the repo root; PROGRESS.md has M00–M15, not M03–M22) — where are the milestone prompts, or should the issues be written from PROGRESS.md alone? (3) M01 carry-over decisions still open (app name; grain retune vs pixel-identity; test photos).

### 2026-10-22  Session 2  (Buffy / Codebuff, M01)

- Changed: M01 AI quarantine behind the cargo feature `ai` (off by default), split into the pieces `denoise`, `faces`
  and `sam` (engine, gpu, ui-egui and both apps). Engine: every model-dependent command stays registered and answers
  with a clear "this build cannot run ..." stub error (`engine/src/cmd/mod.rs` `quarantined`); model download/install
  and faces-index scanning are gated too. UI: the AI controls are hidden, not greyed out (Settings Faces / AI Denoise
  tabs, model-setup dialog, AI Denoise block in Detail, Object/Describe mask tiles); brush/linear/radial/luminance/
  colour-range masks, the Subject/Sky/Background heuristic mattes, patch-based spot healing/cloning and blemish
  removal and the manual noise sliders are untouched. No model weights are downloaded or bundled. Full moved-feature
  and file list: FEATURES.md "AI quarantine (M01)". Also fixed two inherited Windows-only test bugs (export path
  comparison in `engine/src/tests_export.rs`, `/`-normalised folder row ids in `ui-egui/src/tests_folder_drop.rs`) and
  added `engine/src/tests_quarantine.rs` (3 regression tests for the stub errors).
- Tests run / results: `cargo clippy --all-targets -D warnings` green in the default build and with `--features ai`;
  both feature sets compile (engine, gpu, ui-egui, apps). Engine suite: 1060 passed (3 timing-sensitive tests flake
  under load on this machine, pass in isolation). ui-egui suite: full run green after fixes (first run: 22 failures =
  4 outdated M01 expectations + the 8 folder-drop path bugs + 10 load-flaky, all resolved or proven flaky).
  `cargo test -p lightcraft -p lightcraft-cli` (app binaries) in the final sweep; results in the PR.
- Left to do: human review of the M01 PR; M02 onward per the prompts.
- Known issues: tests that wait on background work flake when the machine is loaded (a different subset each run;
  they pass in isolation) and wgpu can lose the Intel Iris Xe device under memory pressure; parallel rustc builds can
  die with `0xc000026b` (OOM) - use `-j 2` on this machine. Still: no `CRAFT_FONTS_DIR`, empty `corpus/`.
- Decisions needed from human: the four from Session 1 are still open (app name; grain retune vs pixel identity;
  YuNet faces behind `ai` - done as part of M01, please confirm; test photos). Plus: approve the M01 PR.

### 2026-10-21  Session 1  (Buffy / Codebuff, M00)
- Changed: Windows toolchain installed on D: (rustup `D:/rust`, stable `x86_64-pc-windows-gnu` 1.99.1 + rustfmt/clippy/wasm32; MSYS2 MinGW-w64 gcc 16.2.0 at `D:/msys64`; build target `D:/rust/target`; C: untouched). AGENTS.md: session-start line pointing at the root product docs (they live at the repo root, not `docs/product/` — the line says so now). FEATURES.md: all 22 `inherited (verify)` rows checked against code and tests; statuses rewritten (20 works, 2 partial) with a per-row evidence table. `crates/catalog/src/store.rs`: `tests_staging` gated `#[cfg(all(test, unix))]` (its only test is unix-only; clippy `-D warnings` flagged the dead `use super::*` on Windows). GitHub: Issues enabled on the fork (human approved), issues #1–#16 = M00–M15 filed with scope + acceptance, label `phase-1`; `upstream` remote = storytold/lightcraft confirmed.
- Tests run / results: `cargo xtask ci` on this machine (rustc 1.99.1, MinGW gcc 16.2.0). Run 1: fmt ok, clippy FAIL (store.rs unused import), everything after skipped. Fixed (module cfg gate, no lint suppression). Run 2 at time of writing: fmt ok, clippy ok, heif build ok, test suite 14 binaries ok / 0 failed so far (still compiling the heavy test targets — candle-core, engine, ui; parity, layers, assets and wasm steps run after tests). Final green run pending; Session 2 records `target/ci/report.json` + `summary.md`.
- Left to do: confirm `cargo xtask ci` reaches green end-to-end and record the final step results; then M01 onward per the issues.
- Known issues: no `CRAFT_FONTS_DIR` (Chinese/Japanese text renders as empty boxes; clone craft-fonts next to this repo and rebuild); `corpus/` is empty (0 of 447 pinned raws) so corpus-dependent tests skip; C: has very little free space — keep toolchains, registries and `target/` on D:.
- Decisions needed from human: (1) app name (VISION.md candidates; needed by M15); (2) EFFECTS_SPEC §4 grain retune vs RULES §4 pixel-identity — retire the old grain look behind a new process version, or approve the retune? (3) does the YuNet face detector go behind the `ai` feature in M01 (leaving manual patch healing only)? (4) test photos: proceed with procedural + CC0 corpus (recorded in test-images/SOURCES.md) or will you supply photos before M03?

### 2026-10-30  Session 5  (Buffy / Freebuff, v2 M04 Glow)
- Changed: real Glow (EFFECTS_SPEC §1) replacing the M02 stub, CPU reference then WGSL port. `crates/develop/src/settings.rs`: `Glow` gains `size` (blur radius as fraction of the long edge, ≈0.5%–6%), `range` (display-tone threshold) and `warmth` (−100..100 tint of the glow layer); still omitted from settings JSON at its exact default, so pre-M04 settings hashes are unchanged. `crates/develop/src/controls.rs`: specs for `glow.size/range/warmth` next to `glow.amount`. `crates/pipeline`: the wide blur now produces a dedicated 3-channel `glow_blur` plane in `local.rs` (`Planes`+`Prepared`), `finish.rs` computes the threshold on the blurred plane's **display luminance through the same tone map as the pixels'** (so an exposure drag moves the threshold with the picture), weights with a soft saturate, tilts the blur's chromaticity by Warmth, and screen-blends (`out = d + k − k·d`) capped per channel at HDR headroom; `None` when amount is 0 (exact identity kept). `crates/gpu`: same plane (`Gpu::Planes.glow`), 5 floats added to the params block (`params.rs`, `ctx.rs` module consts + bindings), WGSL mirror of the CPU math in `wgsl/finish.wgsl`; parity test `glow_matches` added to `crates/gpu/tests/equivalence.rs`. `crates/ui-egui/src/panels/edit.rs`: Glow panel gets the four sliders `glow.amount` first) and 6 original style tiles (Soft Haze / Orton / Dreamy / Golden / Ethereal / Neon, each a bundle of the four sliders merged via `develop.merge`; a matching bundle stays highlighted — same pattern as the vignette styles). UI side-only commands in the engine unchanged (`develop.resetSection` already had `glow`). Before/after renders: five demo photos, 1600 px, `test-images/_renders/p1..p5-{before,after}.jpg` (git-ignored), amount 60 / size 40 / range 50 / warmth 20 — soft haze around the mist bands and sun halo, darks untouched. Branch `m04-glow`.
- Tests run / results: `cargo fmt -D warnings` clean; clippy clean. New tests: pipeline — identity at 0, monotone strength lift, range sweep, no NaN on black/white/noise, scale independence (4000 vs 1000 px lifts match); engine (tests_glow.rs) — settings JSON + XMP sidecar round-trip with the three new fields; gpu — `glow_matches` CPU/GPU parity on real hardware. Full per-package suites (Windows-excluded subsets per this machine): pipeline 196 passed, develop 30 passed, engine 1064 passed, gpu 21 lib + 18 equivalence + 5 more all passed, ui-egui 1097 passed / 0 failed (run with `--test-threads=1`; the default-parallel harness runs hit host memory exhaustion — `STATUS_ACCESS_VIOLATION` / 35 MB allocation failure — after ~880 tests, consistent with the machine's known OOM flakiness, no assertion failures in any run). NOT run: full `cargo xtask ci` (wasm step needs the wasm toolchain extras here, as in previous milestones).
- Left to do: human visual review of the after images (RULES §8); numbers above recorded in EFFECTS_SPEC §1 (ranges confirmed as proposed, presets table added).
- Known issues: ui-egui test harness OOMs at default parallelism on this 2 GB–scale box — use `--test-threads=1`; same carried issues as before (no CRAFT_FONTS_DIR, empty corpus/, `-j 2` rustc OOM workaround).
- Decisions needed from human: (1) approve/merge the M04 PR; (2) test-photos decision from Session 1 still open for tuning rounds on real photos.
