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
async fn insert_note(n: Note) -> Result<i64, turbosql::Error> {
 n.insert() // returns rowid
}

#[backend]
async fn get_note(rowid: i64) -> Result<Note, turbosql::Error> {
 select!(Note "WHERE rowid = ?", rowid)
}

#[server_only]
#[tokio::main]
async fn main() {
 #[derive(rust_embed::RustEmbed)]
 #[folder = "build"]
 struct Frontend;

 eprintln!("Serving on http://127.0.0.1:8080");
 opener::open("http://127.0.0.1:8080").ok();
 warp::serve(turbocharger::warp_routes(Frontend)).run(([127, 0, 0, 1], 8080)).await;
}
