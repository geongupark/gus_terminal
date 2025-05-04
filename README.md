## 🖥️ gus_terminal

A terminal-style desktop GUI application built with [`egui`](https://github.com/emilk/egui) and [`eframe`](https://github.com/emilk/egui/tree/master/crates/eframe), written in Rust.

This app mimics a retro terminal prompt, supports simple commands, tracks command history, and automatically focuses the input when clicked anywhere. It's themed for dark mode with light green text like a real terminal experience.


### ✨ Features

- ⌨️ Terminal-style interface (like macOS Terminal)
- 🕘 Shows current time on launch as `Last login: ...`
- 🟩 Green-on-black theme in dark mode (LightGreen text)
- 🧠 Command history navigation (`↑` / `↓`)
- 🔍 Auto-focus input on any click
- 💬 Supported commands:
  - `ls` — shows folder list
  - `pwd` — shows current directory
  - `about` — shows developer info
  - `help` — shows available commands
  - others — "command not found" message
