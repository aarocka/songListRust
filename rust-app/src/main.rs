use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Song {
    id: u32,
    title: String,
    artist: String,
    year: u32,
}

#[derive(Deserialize)]
struct CreateSong {
    title: String,
    artist: String,
    year: Option<u32>,
}

struct AppState {
    songs: Mutex<Vec<Song>>,
}

async fn index() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "message": "Welcome to Song List API (Rust)"
    }))
}

async fn get_songs(data: web::Data<AppState>) -> impl Responder {
    let songs = data.songs.lock().unwrap();
    HttpResponse::Ok().json(&*songs)
}

async fn get_song(path: web::Path<u32>, data: web::Data<AppState>) -> impl Responder {
    let songs = data.songs.lock().unwrap();
    let id = path.into_inner();
    
    match songs.iter().find(|s| s.id == id) {
        Some(song) => HttpResponse::Ok().json(song),
        None => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Song not found"
        })),
    }
}

async fn create_song(song: web::Json<CreateSong>, data: web::Data<AppState>) -> impl Responder {
    let mut songs = data.songs.lock().unwrap();
    
    let new_id = songs.iter().map(|s| s.id).max().unwrap_or(0) + 1;
    let new_song = Song {
        id: new_id,
        title: song.title.clone(),
        artist: song.artist.clone(),
        year: song.year.unwrap_or_else(|| 2025),
    };
    
    songs.push(new_song.clone());
    HttpResponse::Created().json(new_song)
}

async fn delete_song(path: web::Path<u32>, data: web::Data<AppState>) -> impl Responder {
    let mut songs = data.songs.lock().unwrap();
    let id = path.into_inner();
    
    let initial_len = songs.len();
    songs.retain(|s| s.id != id);
    
    if songs.len() < initial_len {
        HttpResponse::NoContent().finish()
    } else {
        HttpResponse::NotFound().json(serde_json::json!({
            "error": "Song not found"
        }))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let initial_songs = vec![
        Song {
            id: 1,
            title: "Bohemian Rhapsody".to_string(),
            artist: "Queen".to_string(),
            year: 1975,
        },
        Song {
            id: 2,
            title: "Stairway to Heaven".to_string(),
            artist: "Led Zeppelin".to_string(),
            year: 1971,
        },
        Song {
            id: 3,
            title: "Hotel California".to_string(),
            artist: "Eagles".to_string(),
            year: 1976,
        },
    ];

    let app_state = web::Data::new(AppState {
        songs: Mutex::new(initial_songs),
    });

    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let bind_address = format!("0.0.0.0:{}", port);

    println!("Rust Song List API server is running on {}", bind_address);

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/", web::get().to(index))
            .route("/songs", web::get().to(get_songs))
            .route("/songs", web::post().to(create_song))
            .route("/songs/{id}", web::get().to(get_song))
            .route("/songs/{id}", web::delete().to(delete_song))
    })
    .bind(bind_address)?
    .run()
    .await
}
