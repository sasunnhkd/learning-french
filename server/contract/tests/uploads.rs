//! R9 list, R10 upload, R11 delete. Files land in the sandbox's
//! `public/uploads/`; every test deletes what it uploads.

use contract::{Ctx, TINY_PNG, assert_validation, keys};
use serde_json::{Value, json};

const URL_BASE: &str = "http://localhost:5173/learning-french/";
const ALL_TYPES: &str = "The file field must be a file of type: image/png, image/jpeg, image/webp, image/gif, image/svg+xml.";
const NO_SVG_TYPES: &str =
    "The file field must be a file of type: image/png, image/jpeg, image/webp, image/gif.";

fn upload(c: &Ctx, t: &str, name: &str, mime: &str, data: &[u8]) -> contract::Resp {
    c.post("/api/uploads")
        .bearer(t)
        .multipart("file", Some(name), mime, data)
        .send()
}

fn list(c: &Ctx, t: &str) -> Vec<Value> {
    c.get("/api/uploads")
        .bearer(t)
        .send()
        .expect(200)
        .as_array()
        .unwrap()
        .clone()
}

fn is_ulid(s: &str) -> bool {
    s.len() == 26 && s.chars().all(|ch| ch.is_ascii_alphanumeric())
}

#[test]
fn upload_validation() {
    let c = Ctx::from_env();
    let t = c.login();
    let r = c.post("/api/uploads").bearer(&t).send();
    assert_validation(&r, &[("file", &["The file field is required."])]);

    let r = upload(&c, &t, "a.txt", "text/plain", b"hello");
    let types = c.expect_by_target("K4", ALL_TYPES, NO_SVG_TYPES);
    assert_validation(&r, &[("file", &[types])]);

    // A plain form field instead of a file: two errors on the same key.
    let r = c
        .post("/api/uploads")
        .bearer(&t)
        .multipart("file", None, "", b"notafile")
        .send();
    assert_validation(&r, &[("file", &["The file field must be a file.", types])]);
}

#[test]
fn upload_list_delete_png() {
    let c = Ctx::from_env();
    let t = c.login();
    let r = upload(&c, &t, "My Pic.PNG", "image/png", TINY_PNG);
    let b = r.expect(201);
    assert_eq!(keys(&b), ["id", "path", "url", "mime", "size"]);
    assert_eq!(b["mime"], "image/png");
    assert_eq!(b["size"], TINY_PNG.len());

    let id = b["id"].as_str().unwrap().to_string();
    let path = b["path"].as_str().unwrap();
    let file = path.strip_prefix("uploads/").expect("uploads/ prefix");
    let (stem, ext) = file.split_once('.').expect("name.ext");
    assert_eq!(ext, "png", "extension comes from the sniffed MIME");
    assert!(
        is_ulid(stem) && stem == stem.to_uppercase(),
        "uppercase ULID file name: {stem}"
    );
    assert!(
        is_ulid(&id) && id == id.to_lowercase(),
        "lowercase ULID id: {id}"
    );
    // K3 / plan Q7 (default): Laravel's row id is a different ULID.
    assert_eq!(
        id.to_uppercase() == stem,
        c.expect_by_target("K3", false, true)
    );
    assert_eq!(b["url"], format!("{URL_BASE}{path}"));

    let on_disk = c.sandbox.join("public").join(path);
    assert_eq!(std::fs::read(&on_disk).expect("uploaded file"), TINY_PNG);

    let rows = list(&c, &t);
    let row = rows
        .iter()
        .find(|r| r["id"] == id.as_str())
        .expect("listed");
    assert_eq!(
        keys(row),
        [
            "id",
            "path",
            "url",
            "original_name",
            "mime",
            "size",
            "created_at"
        ]
    );
    assert_eq!(row["original_name"], "My Pic.PNG");
    assert_eq!(row["path"], path);
    let created = row["created_at"].as_str().unwrap();
    assert_eq!(created.len(), 27, "Y-m-dTH:i:s.000000Z: {created}");
    assert!(
        created.ends_with(".000000Z") && &created[10..11] == "T",
        "{created}"
    );

    assert_eq!(
        c.delete(&format!("/api/uploads/{id}"))
            .bearer(&t)
            .send()
            .expect(200),
        json!({ "ok": true })
    );
    assert!(!on_disk.exists(), "file removed");
    assert!(
        list(&c, &t).iter().all(|r| r["id"] != id.as_str()),
        "row removed"
    );
    let again = c
        .delete(&format!("/api/uploads/{id}"))
        .bearer(&t)
        .send()
        .expect(404);
    assert_eq!(
        again,
        json!({ "message": format!("No query results for model [App\\Models\\Upload] {id}") })
    );
}

#[test]
fn list_is_newest_first() {
    let c = Ctx::from_env();
    let t = c.login();
    let a = upload(&c, &t, "a.png", "image/png", TINY_PNG).expect(201);
    // created_at has one-second resolution.
    std::thread::sleep(std::time::Duration::from_millis(1100));
    let b = upload(&c, &t, "b.png", "image/png", TINY_PNG).expect(201);
    let ids: Vec<Value> = list(&c, &t).iter().map(|r| r["id"].clone()).collect();
    let pa = ids.iter().position(|i| *i == a["id"]).unwrap();
    let pb = ids.iter().position(|i| *i == b["id"]).unwrap();
    assert!(pb < pa, "newer upload first");
    for x in [a, b] {
        c.delete(&format!("/api/uploads/{}", x["id"].as_str().unwrap()))
            .bearer(&t)
            .send()
            .expect(200);
    }
}

/// K4 / plan Q8 (default: drop SVG). Laravel accepts SVG, script included.
#[test]
fn svg_upload() {
    let c = Ctx::from_env();
    let t = c.login();
    let svg = br#"<svg xmlns="http://www.w3.org/2000/svg" width="1" height="1"><script>alert(1)</script></svg>"#;
    let r = upload(&c, &t, "x.svg", "image/svg+xml", svg);
    if c.is_laravel() {
        let b = r.expect(201);
        assert_eq!(b["mime"], "image/svg+xml");
        assert!(b["path"].as_str().unwrap().ends_with(".svg"));
        c.delete(&format!("/api/uploads/{}", b["id"].as_str().unwrap()))
            .bearer(&t)
            .send()
            .expect(200);
    } else {
        assert_validation(&r, &[("file", &[NO_SVG_TYPES])]);
    }
}

/// K7 / plan Q15 (default: honour the 5120 KB rule). Laravel under
/// `php -S` is capped by `upload_max_filesize` (2M locally), so it can never
/// produce the `max` message; Rust's expected strings come from
/// `vendor/laravel/framework/src/Illuminate/Translation/lang/en/validation.php`
/// (`max.file`: "The :attribute field must not be greater than :max kilobytes.").
#[test]
fn large_uploads() {
    let c = Ctx::from_env();
    let t = c.login();
    let failed: &[&str] = &["The file failed to upload."];

    let mut data = TINY_PNG.to_vec();
    data.resize(3_000_000, 0);
    let r = upload(&c, &t, "big.png", "image/png", &data);
    if c.is_laravel() {
        assert_validation(&r, &[("file", failed)]);
    } else {
        let b = r.expect(201);
        c.delete(&format!("/api/uploads/{}", b["id"].as_str().unwrap()))
            .bearer(&t)
            .send()
            .expect(200);
    }

    data.resize(6_000_000, 0);
    let r = upload(&c, &t, "bigger.png", "image/png", &data);
    let too_big: &[&str] = &["The file field must not be greater than 5120 kilobytes."];
    assert_validation(&r, &[("file", c.expect_by_target("K7", failed, too_big))]);
}

/// The type is sniffed from the bytes (PHP finfo), not taken from the part's
/// Content-Type or the file name, and the extension follows the sniffed type.
#[test]
fn types_are_sniffed_from_content() {
    let c = Ctx::from_env();
    let t = c.login();
    let cases: &[(&str, &str, &[u8], &str, &str)] = &[
        (
            "a.jpeg",
            "image/jpeg",
            include_bytes!("../fixtures/tiny.jpg"),
            "image/jpeg",
            "jpg",
        ),
        (
            "a.gif",
            "image/gif",
            include_bytes!("../fixtures/tiny.gif"),
            "image/gif",
            "gif",
        ),
        (
            "a.webp",
            "image/webp",
            include_bytes!("../fixtures/tiny.webp"),
            "image/webp",
            "webp",
        ),
        // A GIF that claims to be a PNG is stored as a GIF.
        (
            "x.png",
            "image/png",
            include_bytes!("../fixtures/tiny.gif"),
            "image/gif",
            "gif",
        ),
    ];
    for (name, claimed, data, mime, ext) in cases {
        let b = upload(&c, &t, name, claimed, data).expect(201);
        assert_eq!(b["mime"], *mime, "{name}");
        assert_eq!(b["size"], data.len(), "{name}");
        assert!(
            b["path"].as_str().unwrap().ends_with(&format!(".{ext}")),
            "{name}: {}",
            b["path"]
        );
        c.delete(&format!("/api/uploads/{}", b["id"].as_str().unwrap()))
            .bearer(&t)
            .send()
            .expect(200);
    }
    // Text claiming to be a PNG is rejected.
    let r = upload(&c, &t, "fake.png", "image/png", b"hello");
    let types = c.expect_by_target("K4", ALL_TYPES, NO_SVG_TYPES);
    assert_validation(&r, &[("file", &[types])]);
}

/// Bodies above 8 MB (PHP post_max_size) are rejected with 413.
#[test]
fn oversized_body_is_413() {
    let c = Ctx::from_env();
    let t = c.login();
    let r = upload(
        &c,
        &t,
        "huge.bin",
        "application/octet-stream",
        &vec![0u8; 9_000_000],
    );
    assert_eq!(r.status, 413);
}
