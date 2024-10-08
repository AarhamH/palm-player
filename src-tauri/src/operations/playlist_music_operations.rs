use diesel::prelude::*;
use diesel::SqliteConnection;
use crate::db::establish_connection;
use crate::models::playlist_music_model::NewPlaylistMusic;

#[tauri::command]
pub fn insert_song_into_playlist(playlist_id_arg: i32, music_id_arg: i32) -> Result<(), String> {
  use crate::schema::playlist_music::dsl::*;

  let mut connection: SqliteConnection = establish_connection();

  let new_playlist_music: NewPlaylistMusic = NewPlaylistMusic {
    playlist_id: playlist_id_arg,
    music_id: music_id_arg,
  };

  let result: Result<usize, diesel::result::Error> = diesel::insert_into(playlist_music)
    .values(&new_playlist_music)
    .execute(&mut connection);

  match result {
      Ok(_) => Ok(()),
      Err(diesel::result::Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _)) => {
          Err("Error: Duplicate key value pair for playlist entry".to_string())
      }
      Err(err) => {
          Err(format!("Error: {}", err))
      }
  }
}

#[tauri::command]
pub fn destroy_song_from_playlist(playlist_id_arg: i32, music_id_arg: i32) -> Result<(), String> {
  use crate::schema::playlist_music::dsl::*;

  let mut connection: SqliteConnection = establish_connection();

  let result: Result<usize, diesel::result::Error> = diesel::delete(playlist_music
    .filter(playlist_id.eq(playlist_id_arg))
    .filter(music_id.eq(music_id_arg)))
    .execute(&mut connection);

  match result {
      Ok(_) => Ok(()),
      Err(err) => {
          Err(format!("Error: {}", err))
      }
  }
}