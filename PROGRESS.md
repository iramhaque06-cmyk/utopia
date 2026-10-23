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

- [~] M00 Orientation, toolchain check, repo and GitHub setup
- [~] M01 AI quarantine behind cargo feature `ai` (default off)
- [ ] M02 Stub effect proves the end-to-end path
- [ ] M03 Glow and Bloom (CPU reference)
- [ ] M04 Halation
- [ ] M05 GPU parity for effects so far
- [ ] M06 Film grain upgrade
- [ ] M07 Film looks and Looks system
- [ ] M08 Vintage texture (light leaks and dust)
- [ ] M09 Lens blur
- [ ] M10 Tone tools (Punch, Expanse, Tone Zones)
- [ ] M11 Simple mode shell
- [ ] M12 Simple mode tool pages and compare
- [ ] M13 Verify inherited features on Windows, fix gaps
- [ ] M14 Windows packaging and performance
- [ ] M15 Rebrand, QA pass, release candidate

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
