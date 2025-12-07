# Kasumi

*Japanese Music Plays*
Hello fellow ricers! Are y'all tired of some ugly ahh Local music player too?
or Are are you afraid of setting up TUI music applications?

Well I present y'all with **KASUMI** a music player every ricer dreams of.

Kasumi is built for users who care about **speed, aesthetics, and simplicity**... especially on low-end hardware.

---

## Features

* **Fast audio playback** using `rodio`
* **Recursive folder scanning** (MP3, FLAC, WAV, OGG, M4A)
* **Metadata parsing** via `lofty` (title, artist, album, duration, cover art)
* **Auto-play next track in queue**
* **Seekbar** with scrubbing
* **Glass UI with minimal styling**

---

## Tech Stack

* **Svelte 5**
* **Tauri**
* **Rust**
* **Rodio + Symphonia(basically Rust)** 
* **Lofty**
---

## Usage

1. Launch Kasumi
2. Select a music folder
3. Kasumi will automatically scan recursively and load all supported tracks
4. Click a track to play
5. Use the Player Bar for:

   * Previous / Play / Pause / Next
   * Scrubbing with real-time seek

Queue a song via right-click → “Add to Queue”.

---

## Known Limitations

* Seeking backwards fails on some formats (decoder doesn't support random-access).
* No playlists (yet).
* No shuffle/repeat modes.
* REALLY rough and in early stage, I hate the code rn but I will fix it later <3

---

## Planned Stuff

* [ ] Custom themes
* [ ] Lyrics support
* [ ] Smooth waveform seekbar
* [ ] Queue UI
* [ ] Better metadata caching
* [ ] Mini-player mode

---

## 📝 License

GPLv3, keep it free for everyone gang.


