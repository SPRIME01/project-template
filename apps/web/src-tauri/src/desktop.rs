//! Desktop persistence adapter. The browser/API app retains Postgres.js.
//! Only narrow Tauri commands can invoke this native SQLx/PostgreSQL adapter.
use chrono::{DateTime, SecondsFormat, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::{
    fs::{self, OpenOptions},
    io::Write,
    net::TcpListener,
    path::{Path, PathBuf},
    process::Command,
    time::Duration,
};
use tokio::sync::Mutex;
use url::Url;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    pub id: String,
    pub title: String,
    pub body: String,
    pub status: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateNote {
    pub title: String,
    pub body: String,
}

#[derive(sqlx::FromRow)]
struct NoteRow {
    id: Uuid,
    title: String,
    body: String,
    status: String,
    created_at: DateTime<Utc>,
}

impl From<NoteRow> for Note {
    fn from(value: NoteRow) -> Self {
        Self {
            id: value.id.to_string(),
            title: value.title,
            body: value.body,
            status: value.status,
            created_at: value.created_at.to_rfc3339_opts(SecondsFormat::Millis, true),
        }
    }
}

const INSTANCE: &str = "godspeed-project-template-desktop";
const USER: &str = "templateapp";
const DATABASE: &str = "template_notes";

pub struct DesktopDatabase {
    binary: PathBuf,
    app_data: PathBuf,
    instance: String,
    pool: Mutex<Option<PgPool>>,
    // pg0 owns and tracks the postmaster PID. We only stop a cluster we started.
    started_here: Mutex<bool>,
}

impl DesktopDatabase {
    pub fn new(binary: PathBuf, app_data: PathBuf, instance: String) -> Self {
        Self { binary, app_data, instance, pool: Mutex::new(None), started_here: Mutex::new(false) }
    }

    pub fn for_app(binary: PathBuf, app_data: PathBuf) -> Self {
        Self::new(binary, app_data, INSTANCE.into())
    }

    fn password(&self) -> Result<String, String> {
        fs::create_dir_all(&self.app_data).map_err(|e| e.to_string())?;
        let secret_file = self.app_data.join("database.secret");
        if secret_file.exists() {
            let value = fs::read_to_string(&secret_file).map_err(|e| e.to_string())?;
            if value.len() < 24 { return Err("Invalid stored desktop database credential".into()); }
            return Ok(value);
        }
        let random = format!("{}{}", Uuid::new_v4().simple(), Uuid::new_v4().simple());
        let mut options = OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        { use std::os::unix::fs::OpenOptionsExt; options.mode(0o600); }
        match options.open(&secret_file) {
            Ok(mut file) => {
                file.write_all(random.as_bytes()).map_err(|e| e.to_string())?;
                file.sync_all().map_err(|e| e.to_string())?;
                Ok(random)
            }
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists =>
                fs::read_to_string(secret_file).map_err(|e| e.to_string()),
            Err(e) => Err(e.to_string()),
        }
    }

    fn invoke_pg0(&self, args: &[&str]) -> Result<String, String> {
        let output = Command::new(&self.binary).args(args).output()
            .map_err(|e| format!("Cannot launch packaged pg0: {e}"))?;
        if !output.status.success() {
            // Never emit stderr/stdout from pg0 start: arguments may include a credential.
            return Err(format!("pg0 failed (exit status: {})", output.status));
        }
        String::from_utf8(output.stdout).map_err(|e| e.to_string())
    }

    fn running_uri(&self) -> Result<Option<String>, String> {
        let response = self.invoke_pg0(&["info", "--name", &self.instance, "-o", "json"])?;
        let info: serde_json::Value = serde_json::from_str(&response).map_err(|e| e.to_string())?;
        if info["running"] != true { return Ok(None); }
        let uri = info["uri"].as_str().ok_or("pg0 did not return a connection URI")?;
        let parsed = Url::parse(uri).map_err(|e| e.to_string())?;
        if parsed.host_str() != Some("127.0.0.1") || parsed.scheme() != "postgresql" {
            return Err("Refusing to connect to a non-loopback database".into());
        }
        if parsed.username() != USER || parsed.path() != format!("/{DATABASE}") {
            return Err("Unexpected pg0 database identity".into());
        }
        Ok(Some(uri.to_string()))
    }

    async fn connect(&self) -> Result<PgPool, String> {
        let mut guard = self.pool.lock().await;
        if let Some(ref pool) = *guard { return Ok(pool.clone()); }
        if !self.binary.is_file() { return Err("Bundled pg0 binary was not found".into()); }
        let password = self.password()?;
        let data_dir = self.app_data.join("postgres");
        let mut started_now = false;
        let uri = if let Some(existing) = self.running_uri()? {
            existing
        } else {
            let port = TcpListener::bind("127.0.0.1:0")
                .map_err(|e| e.to_string())?.local_addr().map_err(|e| e.to_string())?.port();
            self.invoke_pg0(&[
                "start", "--name", &self.instance, "--data-dir",
                data_dir.to_str().ok_or("Invalid desktop data directory")?,
                "--port", &port.to_string(), "--username", USER,
                "--password", &password, "--database", DATABASE,
                "--config", "listen_addresses=127.0.0.1",
                "--config", "max_connections=20",
                "--config", "shared_buffers=32MB",
                "--config", "maintenance_work_mem=32MB",
                "--config", "work_mem=4MB",
            ])?;
            started_now = true;
            self.running_uri()?.ok_or("PostgreSQL failed to become healthy")?
        };
        let pool = match PgPoolOptions::new().max_connections(5)
            .acquire_timeout(Duration::from_secs(15)).connect(&uri).await {
                Ok(pool) => pool,
                Err(err) => {
                    if started_now { let _ = self.invoke_pg0(&["stop", "--name", &self.instance]); }
                    return Err(format!("Desktop PostgreSQL connection failed: {err}"));
                },
            };
        let migration = sqlx::query(
            "CREATE TABLE IF NOT EXISTS notes (
                id uuid PRIMARY KEY,
                title varchar(120) NOT NULL,
                body text NOT NULL,
                status varchar(16) NOT NULL CHECK (status IN ('open','done')),
                created_at timestamptz NOT NULL
            )"
        ).execute(&pool).await;
        if let Err(err) = migration {
            pool.close().await;
            if started_now { let _ = self.invoke_pg0(&["stop", "--name", &self.instance]); }
            return Err(format!("Desktop schema migration failed: {err}"));
        }
        // This app owns the dedicated instance even after reattaching following a crash.
        // The Tauri single-instance guard prevents competing app owners.
        *self.started_here.lock().await = true;
        *guard = Some(pool.clone());
        Ok(pool)
    }

    pub async fn list(&self) -> Result<Vec<Note>, String> {
        let pool = self.connect().await?;
        let rows: Vec<NoteRow> = sqlx::query_as(
            "SELECT id, title, body, status, created_at FROM notes ORDER BY created_at DESC"
        ).fetch_all(&pool).await.map_err(|e| format!("Read failed: {e}"))?;
        Ok(rows.into_iter().map(Note::from).collect())
    }

    pub async fn create(&self, input: CreateNote) -> Result<Note, String> {
        let title = input.title.trim();
        if title.is_empty() || title.chars().count() > 120 || input.body.chars().count() > 4000 {
            return Err("Invalid note title or body".into());
        }
        let pool = self.connect().await?;
        let row: NoteRow = sqlx::query_as(
            "INSERT INTO notes (id, title, body, status, created_at)
             VALUES ($1, $2, $3, 'open', now())
             RETURNING id, title, body, status, created_at"
        ).bind(Uuid::new_v4()).bind(title).bind(&input.body)
         .fetch_one(&pool).await.map_err(|e| format!("Write failed: {e}"))?;
        Ok(row.into())
    }

    pub async fn shutdown(&self) -> Result<(), String> {
        if let Some(pool) = self.pool.lock().await.take() { pool.close().await; }
        if *self.started_here.lock().await {
            // An application instance never deletes durable database content on exit.
            self.invoke_pg0(&["stop", "--name", &self.instance, "--timeout", "15"])?;
            *self.started_here.lock().await = false;
        }
        Ok(())
    }

    pub fn binary_path(&self) -> &Path { &self.binary }
}

#[cfg(test)]
mod tests {
    use super::CreateNote;

    #[test]
    fn input_structure_is_strictly_typed() {
        let model = CreateNote { title: "hello".into(), body: String::new() };
        assert_eq!(model.title, "hello");
    }
}
