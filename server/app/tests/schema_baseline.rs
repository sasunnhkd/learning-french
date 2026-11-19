//! The sqlx baseline (`migrations/0001_laravel_baseline.sql`) must produce
//! exactly the schema Laravel's migrations produce. The fixture
//! `tests/fixtures/laravel-schema.tsv` is `sqlite_master` (type, name,
//! tbl_name, sql) from a Laravel `migrate` run, minus Laravel's own
//! `migrations` table. Regenerate it with the command in server/README.md.

mod common;

#[tokio::test]
async fn baseline_matches_laravel_schema() {
    let db = common::fresh_db("schema").await;
    let rows: Vec<(String, String, String, Option<String>)> = sqlx::query_as(
        "select type, name, tbl_name, sql from sqlite_master \
         where name not in ('migrations', 'sqlite_sequence', '_sqlx_migrations') \
         and tbl_name not in ('migrations', '_sqlx_migrations') order by rowid",
    )
    .fetch_all(&db.pool)
    .await
    .expect("read sqlite_master");
    let got: Vec<String> = rows
        .into_iter()
        .map(|(t, n, tbl, sql)| format!("{t}\t{n}\t{tbl}\t{}", sql.unwrap_or_default()))
        .collect();
    let want: Vec<String> = include_str!("fixtures/laravel-schema.tsv")
        .lines()
        .map(str::to_string)
        .collect();
    assert_eq!(got, want);
}
