# Cutover: Laravel API → Rust server on Debian 12

How to put the Rust backoffice API (`server/`) on the Debian 12 server, how
to publish content from there, how to upgrade, and how to go back to Laravel.
Decided 2026-11-26 (plan Q3: Debian 12; Q11: systemd + tarball). Every
command is for you to run; nothing here runs automatically.

Placeholders (replace everywhere, also in the packaging files):

- `api.example.com`: the API's host name.
- `EXAMPLE.github.io`: the GitHub Pages host of the site.
- `OWNER`: the GitHub account of the `learning-french` repository.

Default paths (also in `server/packaging/grammaire-server.service`):

| What | Where |
|---|---|
| Binary, one directory per version | `/opt/grammaire-server/<version>/`, with `current` linked to the active one |
| Settings | `/etc/grammaire-server/env` (from `share/env.example`) |
| Database | `/var/lib/grammaire-server/database.sqlite` |
| Site checkout (`REPO_ROOT`) | `/srv/grammaire`: export writes `src/chapters.json`, uploads go to `public/uploads/` |
| Service user | `grammaire` |

## 0. Before: on the Mac

1. All gates green on the commit you release:
   - `cd server && cargo test -p grammaire-server && cargo test -p contract --lib`
   - `server/contract/run-laravel.sh` and `server/contract/run-rust.sh`: the full contract suite
   - `server/contract/export-cmp.sh`
   - `server/contract/rollback-check.sh`
   - `server/scripts/osv-audit.sh`
2. The version in `server/app/Cargo.toml` is bumped for this release (global
   rule: every installable build gets a new version).
3. Push, and let `.github/workflows/server.yml` pass. Its artifact
   `grammaire-server-<commit>` holds
   `grammaire-server-<version>+<commit>-linux-amd64.tar.gz` and its
   `.sha256`. It is built in `rust:1.100.0-bookworm`, so it needs glibc 2.36
   at most (the build checks).
4. The site repository has the content files the server will write
   committed: `src/chapters.json`, and `public/uploads/` if it has files.
5. Make the first database's SQL from the current content (Laravel's
   `migrate --seed` in a sandbox; never from the old local
   `backend/database/database.sqlite`, which is damaged):

   ```bash
   server/scripts/initial-db-sql.sh
   ```

   This writes `server/ci-artifacts/grammaire-initial-<date>.sql` with 27
   chapters, Laravel's schema and `migrations` table (so Laravel can take the
   file back), and the seeder's `editor@local` user. It has no tokens, so
   the editor logs in once more after the cutover.

## 1. Server setup (once)

```bash
sudo apt update
sudo apt install --no-install-recommends nginx sqlite3 git certbot python3-certbot-nginx
sudo useradd --system --home-dir /var/lib/grammaire-server --shell /usr/sbin/nologin grammaire
sudo install -d -o grammaire -g grammaire -m 0750 /var/lib/grammaire-server
sudo install -d -o root -g grammaire -m 0750 /etc/grammaire-server
sudo install -d -m 0755 /opt/grammaire-server
```

The site checkout belongs to `grammaire`, which needs a deploy key with
write access to push content (GitHub → repository → Settings → Deploy keys):

```bash
sudo install -d -o grammaire -g grammaire -m 0700 /var/lib/grammaire-server/.ssh
sudo -u grammaire ssh-keygen -t ed25519 -f /var/lib/grammaire-server/.ssh/id_ed25519 -N ''
sudo cat /var/lib/grammaire-server/.ssh/id_ed25519.pub   # add as a deploy key, "Allow write access"
# Trust GitHub's host key once, so the clone doesn't stop at a prompt.
# Compare the printed fingerprints with GitHub's published ones first.
ssh-keyscan -t ed25519 github.com | sudo -u grammaire tee -a /var/lib/grammaire-server/.ssh/known_hosts
sudo -u grammaire ssh-keygen -lf /var/lib/grammaire-server/.ssh/known_hosts
sudo install -d -o grammaire -g grammaire -m 0755 /srv/grammaire
sudo -u grammaire git clone git@github.com:OWNER/learning-french.git /srv/grammaire
sudo -u grammaire git -C /srv/grammaire config user.name "grammaire backoffice"
sudo -u grammaire git -C /srv/grammaire config user.email "grammaire@api.example.com"
sudo -u grammaire install -d -m 0755 /srv/grammaire/public/uploads
```

The unit's `ReadWritePaths` must exist before the first start:
`/var/lib/grammaire-server`, `/srv/grammaire/src` and
`/srv/grammaire/public/uploads`.

## 2. Install a release

Copy the tarball and its `.sha256` to the server, then:

```bash
sha256sum -c grammaire-server-<version>+<commit>-linux-amd64.tar.gz.sha256
tar -xzf grammaire-server-<version>+<commit>-linux-amd64.tar.gz
cd grammaire-server-<version>+<commit>-linux-amd64
./bin/grammaire-server --version          # must print: grammaire-server <version>
sudo cp -r . /opt/grammaire-server/<version>
sudo ln -sfn /opt/grammaire-server/<version> /opt/grammaire-server/current
```

First install only:

```bash
sudo install -o root -g grammaire -m 0640 share/env.example /etc/grammaire-server/env
sudoedit /etc/grammaire-server/env        # EDITOR_PASSWORD, FRONTEND_BASE_URL, FRONTEND_ORIGIN
sudo -u grammaire sqlite3 /var/lib/grammaire-server/database.sqlite < grammaire-initial-<date>.sql
sudo -u grammaire sqlite3 /var/lib/grammaire-server/database.sqlite 'pragma integrity_check; select count(*) from chapters;'
sudo cp share/grammaire-server.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable --now grammaire-server
curl -s http://127.0.0.1:8000/up           # 200
journalctl -u grammaire-server -n 20       # "listening", no warnings
```

nginx and TLS (first install only). The file has `limit_req` for login
(audit F3), the security headers and slow-client timeouts (F8), and `/up`
for the host only:

```bash
sudo cp share/nginx-grammaire.conf /etc/nginx/sites-available/grammaire-api
sudo sed -i 's/api.example.com/<your host>/g' /etc/nginx/sites-available/grammaire-api
sudo ln -s /etc/nginx/sites-available/grammaire-api /etc/nginx/sites-enabled/
sudo certbot certonly --nginx -d <your host>
sudo nginx -t && sudo systemctl reload nginx
```

## 3. Point the site's backoffice at the server

The SPA reads the API address at build time (`VITE_API_URL`,
`src/admin/api.js`; the default is `http://localhost:8000`). For the
GitHub Pages build:

- set `VITE_API_URL=https://<your host>` in `.github/workflows/deploy.yml`
  (`env:` of the `npm run build` step), or as a repository variable;
- check that `FRONTEND_ORIGIN` in `/etc/grammaire-server/env` is the Pages
  origin (`https://EXAMPLE.github.io`), so CORS allows it.

This changes the SPA build, which is outside `server/`. It is not done in
the migration commits; make it when you cut over.

Then open `https://EXAMPLE.github.io/learning-french/#/admin`, log in, open a
chapter and save it unchanged, and upload an image.

## 4. Publishing content

The server writes into its checkout:

- "exporter chapters.json" writes `/srv/grammaire/src/chapters.json`
  (4-space JSON, as Laravel did: K13);
- uploads land in `/srv/grammaire/public/uploads/`.

GitHub Pages only sees what is pushed. To publish:

```bash
sudo -u grammaire git -C /srv/grammaire pull --rebase
sudo -u grammaire git -C /srv/grammaire add src/chapters.json public/uploads
sudo -u grammaire git -C /srv/grammaire commit -m "content: export $(date -u +%F)"
sudo -u grammaire git -C /srv/grammaire push
```

Pull before every editing session too, so the checkout has the latest code
and content. A "publish" button in the backoffice would replace these
commands. It is a new feature (the plan's "replaced by publish"), not part
of the 1:1 port.

## 5. Backups

The database is one file. A consistent copy while the server runs:

```bash
sudo -u grammaire sqlite3 /var/lib/grammaire-server/database.sqlite ".backup /var/lib/grammaire-server/backup-$(date -u +%F).sqlite"
```

The journal mode stays the default (rollback journal; audit P1), so there are
no `-wal`/`-shm` files to copy.

## 6. Upgrade

Install the new release as in §2 into its own `/opt/grammaire-server/<new>`,
switch `current`, then `sudo systemctl restart grammaire-server`. In-flight
requests finish first (SIGTERM, graceful shutdown). To undo, link
`current` back to the previous version and restart. The database schema
never changes in the Rust port; if a release ever needs a schema change, it
ships as an SQL file for you to apply.

## 7. Rollback to Laravel

The Rust server never changes the schema, and Laravel reads a database Rust
wrote. `server/contract/rollback-check.sh` proves this (passed 2026-11-26):

- Rust logs in, creates, updates and reorders a chapter, and uploads an image.
- Laravel, started on a copy of that file, reads all of it back.
- Laravel accepts the token Rust issued, then deletes, logs out and rejects the revoked token.

To go back:

1. `sudo systemctl stop grammaire-server`
2. Take a backup (§5) and point Laravel's `DB_DATABASE` at the file. On the
   Mac this means `php artisan serve` as before, with the file copied back.
3. Tokens stay valid (Q5), so the editor stays logged in.

What Laravel then does differently (`KNOWN_ISSUES.md`):

- **Saving a chapter in Laravel loses content again** (K1, K2): it drops exercise fields and trims or nulls strings.
- Export answers with the absolute path (K6).
- SVG uploads are accepted again (K4).
- Uploads stop at PHP's 2 MB (K7).

Avoid saving chapters while on Laravel, or keep the rollback short.

## 8. Docs to switch at cutover

`docs/migration/DOC_SWAP.md` has the replacement text for the README,
the project `CLAUDE.md` and `spec/SPEC.md` §6. Apply it the day the Rust
server goes live, not before: until then those files correctly describe
the Laravel setup you use.
