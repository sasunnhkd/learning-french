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
    // Rejected before routing and auth (Laravel ValidatePostSize / PHP),
    // so no token. Over a real socket: the server answers without reading
    // the 9 MB body, and the client must still receive that response.
    let c = Ctx::from_env();
    let r = c
        .post("/api/uploads")
        .multipart(
            "file",
            Some("huge.bin"),
            "application/octet-stream",
            &vec![0u8; 9_000_000],
        )
        .send();
    assert_eq!(r.status, 413);
}

/// A multipart body from raw parts: (Content-Disposition params, optional
/// part Content-Type, bytes).
fn raw_upload(c: &Ctx, t: &str, parts: &[(&str, Option<&str>, &[u8])]) -> contract::Resp {
    let mut body = Vec::new();
    for (disp, ct, data) in parts {
        body.extend_from_slice(b"--XYZ\r\nContent-Disposition: form-data; ");
        body.extend_from_slice(disp.as_bytes());
        body.extend_from_slice(b"\r\n");
        if let Some(ct) = ct {
            body.extend_from_slice(format!("Content-Type: {ct}\r\n").as_bytes());
        }
        body.extend_from_slice(b"\r\n");
        body.extend_from_slice(data);
        body.extend_from_slice(b"\r\n");
    }
    body.extend_from_slice(b"--XYZ--\r\n");
    c.post("/api/uploads")
        .bearer(t)
        .raw("multipart/form-data; boundary=XYZ", body)
        .send()
}

/// The type-list message differs only by SVG (K4, pinned per target in
/// `upload_validation`); these shared tests accept either exact string.
fn assert_errors(r: &contract::Resp, want: &[&str]) {
    let b = r.expect(422);
    let got: Vec<String> = b["errors"]["file"]
        .as_array()
        .unwrap_or_else(|| panic!("file errors: {b}"))
        .iter()
        .map(|m| m.as_str().unwrap().replace(ALL_TYPES, NO_SVG_TYPES))
        .collect();
    let want: Vec<String> = want.iter().map(|s| (*s).to_string()).collect();
    assert_eq!(got, want, "{b}");
    assert_eq!(b.as_object().unwrap().len(), 2);
    assert_eq!(b["errors"].as_object().unwrap().len(), 1, "{b}");
}

fn cleanup(c: &Ctx, t: &str, b: &Value) {
    c.delete(&format!("/api/uploads/{}", b["id"].as_str().unwrap()))
        .bearer(t)
        .send()
        .expect(200);
}

const NOT_A_FILE: &str = "The file field must be a file.";
const REQUIRED: &str = "The file field is required.";

/// Probe captures from slice 3.4: which part counts as `file`, and how
/// non-file values fail. A text field beats a file part of the same name
/// (`array_replace_recursive($input, $files, $input)`); the last of several
/// file parts wins; an empty file name means no file.
#[test]
fn which_part_is_the_file() {
    let c = Ctx::from_env();
    let t = c.login();
    let f = r#"name="file""#;
    let png = (
        r#"name="file"; filename="b.png""#,
        Some("image/png"),
        TINY_PNG,
    );
    let txt = (
        r#"name="file"; filename="a.txt""#,
        Some("text/plain"),
        &b"hi"[..],
    );

    let b = raw_upload(&c, &t, &[txt, png]).expect(201);
    assert_eq!(b["mime"], "image/png", "last file part wins");
    cleanup(&c, &t, &b);
    assert_errors(&raw_upload(&c, &t, &[png, txt]), &[NO_SVG_TYPES]);

    for parts in [
        [(f, None, &b"text"[..]), png],
        [png, (f, None, &b"text"[..])],
    ] {
        assert_errors(&raw_upload(&c, &t, &parts), &[NOT_A_FILE, NO_SVG_TYPES]);
    }
    assert_errors(
        &raw_upload(&c, &t, &[(f, None, &vec![b'a'; 6000])]),
        &[
            NOT_A_FILE,
            "The file field must not be greater than 5120 characters.",
            NO_SVG_TYPES,
        ],
    );
    assert_errors(&raw_upload(&c, &t, &[(f, None, b"")]), &[REQUIRED]);
    assert_errors(
        &raw_upload(
            &c,
            &t,
            &[(r#"name="file"; filename="""#, Some("image/png"), TINY_PNG)],
        ),
        &[REQUIRED],
    );
    assert_errors(
        &raw_upload(
            &c,
            &t,
            &[(
                r#"name="image"; filename="b.png""#,
                Some("image/png"),
                TINY_PNG,
            )],
        ),
        &[REQUIRED],
    );
    for name in ["file[]", "file[x]"] {
        let disp = format!(r#"name="{name}"; filename="b.png""#);
        assert_errors(
            &raw_upload(&c, &t, &[(&disp, Some("image/png"), TINY_PNG)]),
            &[NOT_A_FILE, NO_SVG_TYPES],
        );
    }
    // RFC 5987 `filename*` alone isn't a file name: the part is a text field.
    assert_errors(
        &raw_upload(
            &c,
            &t,
            &[(
                "name=\"file\"; filename*=UTF-8''%C3%A9.png",
                Some("image/png"),
                TINY_PNG,
            )],
        ),
        &[NOT_A_FILE, NO_SVG_TYPES],
    );
    for body in [json!({ "file": "x" }), json!({ "file": ["x"] })] {
        let r = c.post("/api/uploads").bearer(&t).json(&body).send();
        assert_errors(&r, &[NOT_A_FILE, NO_SVG_TYPES]);
    }
    let r = c
        .post("/api/uploads")
        .bearer(&t)
        .raw("application/x-www-form-urlencoded", b"file=".to_vec())
        .send();
    assert_errors(&r, &[REQUIRED]);
}

/// PHP's `finfo` (libmagic 5.46) decides the type, from the bytes only.
/// Minimal signatures that pass, near misses that don't, and PHP file
/// extensions, which Laravel refuses whatever the content.
#[test]
fn sniffing_edges_and_php_names() {
    let c = Ctx::from_env();
    let t = c.login();
    let png16 = &TINY_PNG[..16];
    let ok: &[(&str, &[u8], &str, &str)] = &[
        ("m.png", png16, "image/png", "png"),
        ("m.jpg", b"\xff\xd8\xff\x00", "image/jpeg", "jpg"),
        ("m.gif", b"GIF8", "image/gif", "gif"),
        ("m.webp", b"RIFF\0\0\0\0WEBP", "image/webp", "webp"),
    ];
    for (name, data, mime, ext) in ok {
        let b = upload(&c, &t, name, "application/octet-stream", data).expect(201);
        assert_eq!(
            (b["mime"].as_str().unwrap(), b["size"].as_u64().unwrap()),
            (*mime, data.len() as u64)
        );
        assert!(b["path"].as_str().unwrap().ends_with(&format!(".{ext}")));
        cleanup(&c, &t, &b);
    }
    let bad: &[&[u8]] = &[
        &TINY_PNG[..8],
        &TINY_PNG[..15],
        b"\xff\xd8\xff",
        b"GIF9a",
        b"RIFF\0\0\0\0WAVE",
        b"",
    ];
    for data in bad {
        assert_errors(&upload(&c, &t, "x.png", "image/png", data), &[NO_SVG_TYPES]);
    }
    for name in ["x.php", "x.PHTML", "x.php ", "x.phar"] {
        assert_errors(
            &upload(&c, &t, name, "image/png", TINY_PNG),
            &[NO_SVG_TYPES],
        );
    }
}

/// `original_name` is the client name after the last `/` or `\`, with
/// quoted-string escapes undone.
#[test]
fn original_names() {
    let c = Ctx::from_env();
    let t = c.login();
    let cases = [
        (r#"filename="a/b\\c.png""#, "c.png"),
        (r#"filename="dir\x.png""#, "x.png"),
        (r#"filename="a\"b.png""#, "a\"b.png"),
        ("filename=plain.png", "plain.png"),
        (r#"filename="é ü.png""#, "é ü.png"),
        (r#"filename="a;b.png""#, "a;b.png"),
        (r#"filename=".png""#, ".png"),
        (r#"filename="noext""#, "noext"),
    ];
    for (param, want) in cases {
        let disp = format!(r#"name="file"; {param}"#);
        let b = raw_upload(&c, &t, &[(&disp, None, TINY_PNG)]).expect(201);
        assert!(b["path"].as_str().unwrap().ends_with(".png"), "{param}");
        let row = list(&c, &t)
            .into_iter()
            .find(|r| r["id"] == b["id"])
            .expect("listed");
        assert_eq!(row["original_name"], want, "{param}");
        cleanup(&c, &t, &b);
    }
}

/// Delete succeeds when the file is already gone (`is_file` check).
#[test]
fn delete_when_file_is_gone() {
    let c = Ctx::from_env();
    let t = c.login();
    let b = upload(&c, &t, "g.png", "image/png", TINY_PNG).expect(201);
    let on_disk = c.sandbox.join("public").join(b["path"].as_str().unwrap());
    std::fs::remove_file(&on_disk).expect("remove uploaded file");
    let id = b["id"].as_str().unwrap();
    let r = c.delete(&format!("/api/uploads/{id}")).bearer(&t).send();
    assert_eq!(r.expect(200), json!({ "ok": true }));
    assert!(list(&c, &t).iter().all(|r| r["id"] != id));
}

/// PHP's multipart parser matches `name` and `filename` in any case and
/// accepts single-quoted values (slice 3.4 probe). Rust parses the header
/// the same way (`input::php_disposition`), not with multer's rules.
#[test]
fn content_disposition_parsing() {
    let c = Ctx::from_env();
    let t = c.login();
    for disp in [
        r#"Name="file"; filename="b.png""#,
        r#"name="file"; FILENAME="b.png""#,
        r#"NAME="file"; FileName="b.png""#,
        "name='file'; filename='b.png'",
    ] {
        let b = raw_upload(&c, &t, &[(disp, Some("image/png"), TINY_PNG)]).expect(201);
        let row = list(&c, &t)
            .into_iter()
            .find(|r| r["id"] == b["id"])
            .expect("listed");
        assert_eq!(row["original_name"], "b.png", "{disp}");
        cleanup(&c, &t, &b);
    }
}

/// K19: a file name that isn't UTF-8 (`caf\xe9.png`, Latin-1). Laravel
/// stores the raw bytes, and then `GET /api/uploads` fails with 500 until
/// that upload is deleted (`json_encode` can't encode the name). Rust
/// stores the name with U+FFFD for the bad byte, so the list keeps working.
#[test]
fn non_utf8_file_name() {
    let c = Ctx::from_env();
    let t = c.login();
    let mut body = b"--XYZ\r\nContent-Disposition: form-data; name=\"file\"; filename=\"caf\xe9.png\"\r\nContent-Type: image/png\r\n\r\n".to_vec();
    body.extend_from_slice(TINY_PNG);
    body.extend_from_slice(b"\r\n--XYZ--\r\n");
    let b = c
        .post("/api/uploads")
        .bearer(&t)
        .raw("multipart/form-data; boundary=XYZ", body)
        .send()
        .expect(201);
    let r = c.get("/api/uploads").bearer(&t).send();
    let status = r.status;
    let name = if status == 200 {
        r.json()
            .as_array()
            .unwrap()
            .iter()
            .find(|row| row["id"] == b["id"])
            .map(|row| row["original_name"].clone())
    } else {
        None
    };
    // Delete first, so a failed assertion can't leave the list broken.
    cleanup(&c, &t, &b);
    assert_eq!(status, c.expect_by_target("K19", 500, 200));
    if !c.is_laravel() {
        assert_eq!(name, Some(json!("caf\u{fffd}.png")));
    }
}

/// Phase 5: PHP's `max_file_uploads` (20), probed on Laravel. File parts
/// after the 20th are ignored, so a 21st `file` part never wins; parts with
/// an empty file name don't count toward the 20. (`php -S` prints a warning
/// before the JSON when the cap is hit, so only statuses and types are
/// compared here.)
#[test]
fn file_part_cap_like_php() {
    let c = Ctx::from_env();
    let t = c.login();
    let mime_of = |r: &contract::Resp| {
        let text = r.text();
        let at = text.find('{').expect("JSON body");
        serde_json::from_str::<Value>(&text[at..]).unwrap()["mime"].clone()
    };
    let txt = (r#"name="file"; filename="a.txt""#, None, &b"hi"[..]);
    let png = (r#"name="file"; filename="p.png""#, None, TINY_PNG);
    let gif = (
        r#"name="file"; filename="g.gif""#,
        None,
        &include_bytes!("../fixtures/tiny.gif")[..],
    );
    // The 20th part (PNG) wins; the 21st (GIF) is ignored.
    let mut parts = vec![txt; 19];
    parts.extend([png, gif]);
    let r = raw_upload(&c, &t, &parts);
    assert_eq!((r.status, mime_of(&r)), (201, json!("image/png")));
    let id = {
        let text = r.text();
        serde_json::from_str::<Value>(&text[text.find('{').unwrap()..]).unwrap()["id"].clone()
    };
    c.delete(&format!("/api/uploads/{}", id.as_str().unwrap()))
        .bearer(&t)
        .send()
        .expect(200);
    // 20 files under another name use up the cap.
    let others: Vec<String> = (0..20)
        .map(|i| format!(r#"name="other{i}"; filename="a.txt""#))
        .collect();
    let mut parts: Vec<(&str, Option<&str>, &[u8])> = others
        .iter()
        .map(|d| (d.as_str(), None, &b"hi"[..]))
        .collect();
    parts.push(png);
    assert_eq!(raw_upload(&c, &t, &parts).status, 422);
    // 20 parts with an empty file name don't.
    let empties: Vec<String> = (0..20)
        .map(|i| format!(r#"name="x{i}"; filename="""#))
        .collect();
    let mut parts: Vec<(&str, Option<&str>, &[u8])> = empties
        .iter()
        .map(|d| (d.as_str(), None, &b""[..]))
        .collect();
    parts.push(png);
    let r = raw_upload(&c, &t, &parts);
    assert_eq!((r.status, mime_of(&r)), (201, json!("image/png")));
    c.delete(&format!(
        "/api/uploads/{}",
        r.json()["id"].as_str().unwrap()
    ))
    .bearer(&t)
    .send()
    .expect(200);
}
