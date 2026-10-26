# RULES

Every agent session must follow these. If a task seems to require breaking one, **stop and ask the human**.

## 1. Clean-room and legal

- Never use, read, paste, or derive from code, shaders, assets, model files, decompiled or extracted material belonging to Snapseed, Lightroom, or any other proprietary app. This includes APK dumps and the output of reverse-engineering tools.
- Implement from general public techniques and our own tuning by eye. Describe the technique in the code comment (for example, "threshold, blur, screen blend").
- Use original names. No brand, trademark or film-stock names (no camera-maker film names, no competitor tool names).
- Keep LightCraft's `NOTICE`, license files and copyright headers. Never reuse the ArtCraft name or logos.
- Check the license of every new dependency, model, dataset, font and icon before adding it. Record it in `THIRD_PARTY.md`. GPL/AGPL dependencies need human approval. Lens data (lensfun database is CC-BY-SA) stays a separate, user-supplied download unless the human approves otherwise.
- Do not copy code from GPL projects (for example darktable, RawTherapee) into this repo.

## 2. No AI in Phase 1

- Do not add new AI/ML features, model downloads, model files, or ML runtime dependencies.
- Existing AI-dependent LightCraft code is placed behind a cargo feature `ai`, **off by default**. A default build must compile, run and pass tests without it and without any model weights.
- Non-AI masks (gradients, brush, luminosity and color range) and non-AI healing stay in the default build.

## 3. Privacy

- All processing is local. No telemetry, analytics, crash uploads or network calls unless the human explicitly approves them in writing in PROGRESS.md.
- Never send user images, paths or metadata off the machine.

## 4. Rendering correctness

- Existing renders must stay pixel-identical unless the human approves. Respect the repo's process-version mechanism; never silently change the look of existing edits.
- Work in linear light; convert to display space only at the end.
- CPU reference and GPU output must match within the repo's parity tolerance. If the GPU path cannot match, fix the GPU, do not loosen the CPU. If tolerance must change, explain why and get approval.
- Every effect: strength 0 is an exact identity; deterministic for a given seed; no NaN or out-of-range output; spatial sizes scale with image size.
- GPU errors or missing GPU support must fall back to CPU, never crash.

## 5. Code quality

- Follow `AGENTS.md` and the repo's layer rules. Lower layers must not depend on UI crates.
- New logic goes in the engine and is exposed through the existing command registry. UI code must not duplicate engine logic, and every parameter exists exactly once (Basic vs. Advanced only changes visibility).
- Small, reviewable diffs. Do not reformat or refactor unrelated files.
- Every new feature gets tests. Run `cargo xtask ci` (or the closest working subset on Windows) before declaring a milestone done. If something cannot run on Windows, say so and list what was skipped.
- If you hit the same error three times, stop, summarize what you tried, and ask.

## 6. Git and GitHub (MCP) rules

- Work on a branch per milestone, for example `m03-glow-bloom`. **Never push directly to `main`.**
- Open a pull request per milestone with: summary, files changed, tests run, screenshots if visual, known issues.
- Create one GitHub issue per milestone (from PROGRESS.md) and link the PR to it.
- Do **not** force-push, delete branches, change repo visibility, change repo settings, or create public repositories without explicit human approval in the current session.
- Never commit secrets, tokens, license-key private material, personal data, or large binaries. Test images over 5 MB stay out of git (list them in TEST_PLAN.md instead).
- Keep the upstream remote (`storytold/lightcraft`) so upstream fixes can be pulled in.
- Commit messages: imperative, concise ("Add halation effect CPU reference").

## 7. Session discipline

- Read `docs/product/PROGRESS.md` first. Work only on the milestone given.
- Show a plan before editing files.
- At the end of every session append to PROGRESS.md: what changed, what is left, known issues, and any decisions needed from the human.
- Do not mark anything done unless tests pass and (for visual work) the human has confirmed the result looks right.

## 8. Quality bar for visual work

- Tests catch math errors, not ugliness. After any visual change, render before/after images of at least five test photos into `test-images/_renders/` (git-ignored) and tell the human which to look at.
- Defaults must look good, not dramatic. If an effect looks harsh at its default, lower the default.

## 9. Reference material

- Allowed: public documentation, behavior notes written from public sources or from our own use of an app, general image-processing literature.
- Not allowed: decompiled, extracted or dumped material from any proprietary app (code, shaders, strings, assets, model files, APK or IPA contents).
- AI-written notes about another app are **unverified hypotheses**. Never treat them as ground truth; verify by observation and tune by eye.
- Do not add files that describe reverse-engineered or "pixel-perfect clone" specifications of another app to this repo. Keep such files outside the repository. Product-relevant behavior is distilled into DESIGN_DECISIONS.md and EFFECTS_SPEC.md.
- Behavior, not appearance: do not replicate another app's tool-grid layout, icons, colors, visual theme, preset names, or film-stock names and codes. Our look and navigation are our own.
- If a file like this is found in the repo, tell the human and remove it from the working tree; do not rewrite git history without approval.
