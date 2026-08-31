# Future Implementations

The following IPC calls made by the `skwd-wall` UI currently return dummy values and are not yet connected to the Caelestia Shell backend. They should be implemented when the shell supports these features:

- **Outputs**
  - `wall.outputs`: Fetch the list of connected monitors/displays.

- **Playlists**
  - `playlist.list`: Fetch wallpaper playlists.
  - `playlist.update`: Update a playlist.
  - `playlist.assign`: Assign a playlist to a display.
  - `playlist.create`: Create a playlist.
  - `playlist.delete`: Delete a playlist.

- **Audio/Video**
  - `wall.set_audio`: Change volume and mute settings for video wallpapers.

- **Themes & Effects**
  - `effects.list`: Fetch the list of available shader/compositor effects.
  - `theme.backends`: Fetch the list of theme extraction backends.

- **Metadata & Data Management**
  - `wall.update_tags`: Tag a wallpaper.
  - `optimize.start`: Start optimizing/caching wallpaper thumbnails.
  - `wall.shell_preview`: Send a temporary wallpaper to the shell for preview.
