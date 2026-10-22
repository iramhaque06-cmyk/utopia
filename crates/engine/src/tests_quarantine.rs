//! M01 AI quarantine (RULES section 2): without the `ai` cargo feature the AI commands stay
//! registered — listings stay truthful, scripts get a reason — but every call answers with a
//! clear "unsupported in this build" error, never "unknown command" and never a panic.

use serde_json::json;

use crate::Session;

/// The wording every quarantined AI command answers with (see `cmd::AI_UNSUPPORTED`).
const AI_UNSUPPORTED: &str = "built without AI";

#[cfg(not(feature = "denoise"))]
#[test]
fn denoise_commands_stay_registered_and_say_the_build_has_no_ai() {
    let mut s = Session::new();
    for id in ["denoise.models.list", "denoise.settings", "denoise.models.install"] {
        assert!(crate::cmd::command_specs().iter().any(|c| c.id == id), "{id} must stay registered");
    }
    for (id, params) in [
        ("denoise.models.list", json!({})),
        ("denoise.settings", json!({"amount": 0.5})),
        ("denoise.models.install", json!({"path": "x.onnx", "acknowledged": true})),
    ] {
        let e = s.execute(id, &params).expect_err("quarantined: must not run").to_string();
        assert!(e.contains(AI_UNSUPPORTED), "{id} answered `{e}`");
    }
}

#[cfg(not(feature = "faces"))]
#[test]
fn face_commands_stay_registered_and_say_the_build_has_no_ai() {
    let mut s = Session::new();
    for id in ["faces.enable", "faces.models.install", "faces.detect", "spot.findBlemishes"] {
        assert!(crate::cmd::command_specs().iter().any(|c| c.id == id), "{id} must stay registered");
    }
    for (id, params) in [
        ("faces.enable", json!({"enabled": true})),
        ("faces.models.install", json!({"path": "m.onnx", "acknowledged": true})),
        ("faces.detect", json!({})),
        ("spot.findBlemishes", json!({})),
    ] {
        let e = s.execute(id, &params).expect_err("quarantined: must not run").to_string();
        assert!(e.contains(AI_UNSUPPORTED), "{id} answered `{e}`");
    }
}

#[cfg(not(feature = "sam"))]
#[test]
fn ai_masks_say_the_build_has_none() {
    // `mask.add {kind: "object" | "prompt"}` lands here first, so the shape cannot fail later.
    let e = crate::segment::Segmenter::default().require_model().expect_err("no SAM in this build").to_string();
    assert!(e.contains("AI masks are not available in this build"), "{e}");
}
