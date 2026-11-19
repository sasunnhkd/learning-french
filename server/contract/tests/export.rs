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
