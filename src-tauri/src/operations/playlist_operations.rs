use diesel::prelude::*;
use crate::models::playlist_model:: {
    NewPlaylist, Playlist, PlaylistArg 
};
use crate::models::playlist_music_model::NewPlaylistMusic;
use crate::db::establish_connection;

#[tauri::command]
pub fn create_playlist(playlist_arg: PlaylistArg) -> Result<(), String> {
  use crate::schema::playlist::dsl::*;

  let mut connection: SqliteConnection = establish_connection();

  let new_playlist: NewPlaylist<'_> = NewPlaylist {
    title: &playlist_arg.title.unwrap_or_default().to_string(),
    created_on: &playlist_arg.created_on.unwrap_or_default().to_string(),
  };

  let result = diesel::insert_into(playlist)
    .values(&new_playlist)
    .execute(&mut connection);

  match result {
    Ok(_) => Ok(()),
    Err(diesel::result::Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _)) => {
        Err("Error: Duplicate key value pair".to_string())
    }
    Err(err) => {
        Err(format!("Error: {}", err))
    }
  }
}

#[tauri::command]
pub fn get_all_playlists() -> Result<Vec<Playlist>, String> {
  use crate::schema::playlist::dsl::*;

  let mut connection: SqliteConnection = establish_connection();

  let playlists: Vec<Playlist> = match playlist.load::<Playlist>(&mut connection) {
      Ok(result) => result,
      Err(err) => {
          eprintln!("Error loading playlists: {}", err);
          return Err(format!("Error loading playlists: {}", err)); // Return an error message
      }
  };
  
  Ok(playlists)
}

#[tauri::command]
pub fn update_playlist(id_arg: i32, playlist_arg: PlaylistArg) -> Result<(), String> {
  use crate::schema::playlist::dsl::*;

  let mut connection = establish_connection();

  let current_playlist: Playlist = playlist
    .find(id_arg)
    .first(&mut connection)
    .expect("Error loading playlist");

  let new_playlist = Playlist {
    id: id_arg,
    title: playlist_arg.title.unwrap_or(current_playlist.title),
    created_on: playlist_arg.created_on.unwrap_or(current_playlist.created_on),
  };

  let result = diesel::update(playlist.find(id_arg))
    .set(&new_playlist)
    .execute(&mut connection);

  match result {
    Ok(_) => Ok(()),
    Err(err) => Err(format!("Error updating playlist: {}", err)), // Return error to the client
  }
}

#[tauri::command]
pub fn insert_song_into_playlist(playlist_id_arg: i32, music_id_arg: i32) -> Result<(), String> {
  use crate::schema::playlist_music::dsl::*;

  let mut connection = establish_connection();

  let new_playlist_music = NewPlaylistMusic {
    playlist_id: playlist_id_arg,
    music_id: music_id_arg,
  };

  let result: Result<usize, diesel::result::Error> = diesel::insert_into(playlist_music)
    .values(&new_playlist_music)
    .execute(&mut connection);

  match result {
      Ok(_) => Ok(()),
      Err(diesel::result::Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _)) => {
          Err("Error: Duplicate key value pair".to_string()) // Return error to the client
      }
      Err(err) => {
          Err(format!("Error: {}", err))
      }
  }
}