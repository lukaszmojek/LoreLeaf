# LoreLeaf

What you would call a ebook reading app. But better. 'Loreful' if you will.

![LoreLeaf_logo](logo_320.jpeg)

## The why

### Inspiration
What are ebooks missing? Every time I am reading one of those I feel left out due to the lack of drawings and maps, that are most of the time present in their paper equivalents.
Other problem that I couldn't see being addressed properly are:
- No access to the whole world described in the book (lore), which does not boil down to simple searching the text
- The lack of specifically selected soundtracks to accompany the reading
- No way quickly access artwork for the book (if it exists)

### Technology considerations
- [Bevy](https://bevyengine.org/) - open source game engine written in Rust
- [Godot](https://godotengine.org/) (with gdext plugin) - open source game engine with extension for supporting Rust as a scripting language
- [Tauri](https://tauri.app/) - open source framework for developing desktop applications with a web frontend

Since from all of those, the one that is giving the most freedom is **Bevy**, I decided to go with it.
