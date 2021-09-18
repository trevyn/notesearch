use turbocharger::{backend, server_only};
use wasm_bindgen::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
use turbosql::{select, Turbosql};

#[backend]
#[cfg_attr(not(target_arch = "wasm32"), derive(Turbosql))]
pub struct Note {
 pub rowid: Option<i64>,
 pub text: Option<String>,
}

#[backend]
#[cfg_attr(not(target_arch = "wasm32"), derive(Turbosql))]
pub struct NoteLog {
 pub rowid: Option<i64>,
 pub note_rowid: Option<i64>,
 pub text: Option<String>,
 pub timestamp: Option<f64>,
}

#[backend]
async fn note_insert(n: Note) -> Result<i64, turbosql::Error> {
 n.insert() // returns rowid
}

#[backend]
async fn note_get(rowid: i64) -> Result<Note, turbosql::Error> {
 select!(Note "WHERE rowid = ?", rowid)
}

#[backend]
async fn note_update(rowid: i64, text: String) -> Result<usize, turbosql::Error> {
 NoteLog {
  rowid: None,
  note_rowid: Some(rowid),
  text: Some(text.clone()),
  timestamp: Some(
   std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs_f64(),
  ),
 }
 .insert()?;
 Note { rowid: Some(rowid), text: Some(text) }.update()
}

#[server_only]
#[tokio::main]
async fn main() {
 #[derive(rust_embed::RustEmbed)]
 #[folder = "build"]
 struct Frontend;

 pretty_env_logger::init_timed();

 log::warn!("warn enabled");
 log::info!("info enabled");
 log::debug!("debug enabled");
 log::trace!("trace enabled");

 eprintln!("Serving on http://127.0.0.1:8080");
 opener::open("http://127.0.0.1:8080").ok();
 warp::serve(turbocharger::warp_routes(Frontend)).run(([127, 0, 0, 1], 8080)).await;
}
