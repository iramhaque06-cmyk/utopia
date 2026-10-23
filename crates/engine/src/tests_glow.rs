//! M02 stub effect (Glow) end to end: `glow.amount` is a real develop control, its value
//! round-trips through the settings JSON and the XMP sidecar (`lc:settings` carries the full
//! settings), and moving the slider is one undo step like every other control.

use serde_json::json;

use lightcraft_catalog::PhotoId;

use crate::Session;

fn select(s: &mut Session, ids: &[PhotoId]) {
    s.execute("library.select", &json!({"ids": ids.iter().map(|i| i.0).collect::<Vec<_>>()})).unwrap();
}

#[test]
fn glow_amount_round_trips_through_the_settings_json_and_the_sidecar() {
    let mut s = Session::with_demo();
    let id = s.visible_cloned()[0];
    select(&mut s, &[id]);
    s.execute("develop.set", &json!({"control": "glow.amount", "value": 35.0})).unwrap();
    let d = (*s.develop_of(id).unwrap()).clone();
    assert_eq!(d.glow.amount, 35.0);
    // the settings JSON (what `lc:settings` stores) round-trips the value
    let back = lightcraft_develop::DevelopSettings::from_json(&d.to_json()).unwrap();
    assert_eq!(back.glow.amount, 35.0);
    // and the whole sidecar packet round-trips it too
    let p = s.catalog.photo(id).unwrap().clone();
    let packet = crate::sidecar::sidecar_packet(&p, &s.catalog);
    let sc = crate::sidecar::parse_sidecar(&packet, crate::crs::Target::Rendered).unwrap();
    let Some(crate::sidecar::DevelopPatch::Full(full)) = sc.develop else {
        panic!("expected the full settings back from lc:settings");
    };
    assert_eq!(full.glow.amount, 35.0);
}

#[test]
fn glow_amount_is_one_undo_step() {
    let mut s = Session::with_demo();
    let id = s.visible_cloned()[0];
    select(&mut s, &[id]);
    assert_eq!(s.develop_of(id).unwrap().glow.amount, 0.0);
    s.execute("develop.set", &json!({"control": "glow.amount", "value": 42.0})).unwrap();
    assert_eq!(s.develop_of(id).unwrap().glow.amount, 42.0);
    s.execute("edit.undo", &json!({})).unwrap();
    assert_eq!(s.develop_of(id).unwrap().glow.amount, 0.0);
    s.execute("edit.redo", &json!({})).unwrap();
    assert_eq!(s.develop_of(id).unwrap().glow.amount, 42.0);
}

#[test]
fn glow_resets_with_its_section() {
    let mut s = Session::with_demo();
    let id = s.visible_cloned()[0];
    select(&mut s, &[id]);
    s.execute("develop.set", &json!({"control": "glow.amount", "value": 20.0})).unwrap();
    s.execute("develop.resetSection", &json!({"section": "glow"})).unwrap();
    assert_eq!(s.develop_of(id).unwrap().glow.amount, 0.0);
}
