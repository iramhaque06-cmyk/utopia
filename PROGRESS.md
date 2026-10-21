# PROGRESS

The agent reads this at the start of every session and appends to it at the end. This file is the project's memory because the agent forgets between sessions.

## Human decisions (fill in)

| Decision | Value |
|---|---|
| App name | TBD |
| GitHub repo (fork of storytold/lightcraft) | TBD (name, visibility: private recommended until ready) |
| Target Windows version and GPU | TBD |
| Test images supplied | no |
| Price / tiers | deferred to Phase 2 |

## Milestones (Phase 1: full app, no AI, no tiers)

Status: `[ ]` not started, `[~]` in progress, `[x]` done and human-approved.

- [ ] M00 Orientation, toolchain check, repo and GitHub setup
- [ ] M01 AI quarantine behind cargo feature `ai` (default off)
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

(no sessions yet)
