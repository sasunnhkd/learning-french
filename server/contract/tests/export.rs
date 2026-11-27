//! R12 export: response shape, and the bytes written to `src/chapters.json`
//! in the sandbox (PHP pretty print, 4-space indent, unescaped, trailing \n).

use contract::{Ctx, keys, php_pretty};
use serde_json::json;
use std::os::unix::fs::PermissionsExt;

#[test]
fn export_writes_file_and_returns_chapters() {
    let c = Ctx::from_env();
    let t = c.login();
    let body = c.post("/api/export").bearer(&t).send().expect(200);
    assert_eq!(keys(&body), ["count", "path", "written", "chapters"]);
    assert_eq!(body["written"], true);

    let chapters = body["chapters"].as_array().expect("chapters array");
    assert_eq!(body["count"], chapters.len());

    // Same rows, order and shape as index + show.
    let index = c.get("/api/chapters").bearer(&t).send().expect(200);
    let ids: Vec<_> = index
        .as_array()
        .unwrap()
        .iter()
        .map(|r| r["id"].clone())
        .collect();
    let exported: Vec<_> = chapters.iter().map(|r| r["id"].clone()).collect();
    assert_eq!(exported, ids, "export order = index order");
    for ch in chapters {
        let id = ch["id"].as_str().unwrap();
        let shown = c
            .get(&format!("/api/chapters/{id}"))
            .bearer(&t)
            .send()
            .expect(200);
        assert_eq!(ch, &shown, "{id}");
        assert_eq!(keys(ch), keys(&shown), "{id} key order");
    }

    // K6 / plan Q10 (default): Laravel returns the absolute path.
    let path = body["path"].as_str().unwrap();
    if c.is_laravel() {
        assert!(
            path.starts_with('/') && path.ends_with("/src/chapters.json"),
            "{path}"
        );
    } else {
        assert_eq!(path, "src/chapters.json");
    }

    let written = std::fs::read(c.sandbox.join("src/chapters.json")).expect("export file");
    let want = php_pretty(&body["chapters"]);
    assert!(
        written == want,
        "export file differs from PHP pretty print (first diff at byte {:?})",
        written.iter().zip(&want).position(|(a, b)| a != b)
    );
}

/// K6: a failed write still answers 200, with `written: false`.
#[test]
fn export_reports_unwritable_file() {
    let c = Ctx::from_env();
    let t = c.login();
    let file = c.sandbox.join("src/chapters.json");
    // Make sure the file exists, then make it read-only.
    c.post("/api/export").bearer(&t).send().expect(200);
    let before = std::fs::read(&file).unwrap();
    std::fs::set_permissions(&file, std::fs::Permissions::from_mode(0o444)).unwrap();
    let r = c.post("/api/export").bearer(&t).send();
    std::fs::set_permissions(&file, std::fs::Permissions::from_mode(0o644)).unwrap();
    let body = r.expect(200);
    assert_eq!(body["written"], json!(false));
    assert_eq!(std::fs::read(&file).unwrap(), before, "file untouched");
}

/// Deletes `contract-export` and re-exports on drop, so a failed assertion
/// can't leave a text `position` behind (it would make `max(position)` read
/// as 0 in later tests) or the export file out of seed state.
struct ExportFixture<'a> {
    c: &'a Ctx,
    token: &'a str,
}

impl Drop for ExportFixture<'_> {
    fn drop(&mut self) {
        let _ = self
            .c
            .delete("/api/chapters/contract-export")
            .bearer(self.token)
            .send();
        let _ = self.c.post("/api/export").bearer(self.token).send();
    }
}

/// The file is PHP's pretty print byte for byte, also for strings that
/// need escaping (controls, U+2028/U+2029) and ones that don't (`/`, é,
/// emoji, DEL). A chapter whose `position` reorder left as text (K8) is
/// exported last with position 0, like the index.
#[test]
fn export_escaping_and_text_position() {
    let c = Ctx::from_env();
    let t = c.login();
    let tricky =
        "a/b é 😀 x\u{2028}y\u{2029}z \"q\" \\ n\nt\tr\rb\u{8}f\u{c} \u{1}\u{1f}\u{7f} end";
    let chapter = json!({
        "id": "contract-export", "number": 950, "section": tricky, "title": tricky, "titleEm": tricky,
        "lesson": { "intro": tricky, "formation": [tricky], "irregulars": [], "triggers": [] },
        "items": [{ "type": "qcm", "rule": tricky }]
    });
    let _guard = ExportFixture { c: &c, token: &t };
    c.post("/api/chapters")
        .bearer(&t)
        .json(&chapter)
        .send()
        .expect(201);
    c.post("/api/chapters/reorder")
        .bearer(&t)
        .json(&json!({ "order": { "x": "contract-export" } }))
        .send()
        .expect(200);
    let body = c.post("/api/export").bearer(&t).send().expect(200);
    let written = std::fs::read(c.sandbox.join("src/chapters.json")).expect("export file");
    let last = body["chapters"].as_array().unwrap().last().unwrap().clone();
    assert_eq!(
        (&last["id"], &last["position"]),
        (&json!("contract-export"), &json!(0))
    );
    assert_eq!(last["title"], tricky);
    let want = php_pretty(&body["chapters"]);
    assert!(
        written == want,
        "export file differs from PHP pretty print (first diff at byte {:?})",
        written.iter().zip(&want).position(|(a, b)| a != b)
    );
    let text = String::from_utf8(written).unwrap();
    let want_line = "\"a/b é 😀 x\\u2028y\\u2029z \\\"q\\\" \\\\ n\\nt\\tr\\rb\\bf\\f \\u0001\\u001f\u{7f} end\"";
    assert!(text.contains(want_line), "escaped section line");
}
