<p align="center">
  <img src="https://raw.githubusercontent.com/goome-ui/.github/main/profile/logo.png" alt="goome" width="150">
</p>

<h1 align="center">🦀 goome app template: Rust 🦀</h1>

<p align="center">
<b>A tiny goome app in Rust to start from. A normal window, and one toggle turns it into an overlay on top of everything.</b>
</p>

<p align="center">
  <img src="https://goome-ui.github.io/docs/assets/badges/rust-177.svg" alt="Rust 1.77+">
  <img src="https://goome-ui.github.io/docs/assets/badges/template-use-me.svg" alt="template">
  <img src="https://img.shields.io/badge/license-GPL--3.0-1b1d22?style=flat-square" alt="GPL-3.0">
</p>

<p align="center">
  Hit <b>Use this template</b> up top and you get your own copy that builds right away, CI included. Or let <a href="https://github.com/goome-ui/goome-cli">goome-cli</a> make it with your own name already in it.<br>
  Its <a href="https://github.com/goome-ui/rust-goome">rust-goome</a> underneath, the docs for the whole ui kit are at <a href="https://goome-ui.github.io/docs/">goome-ui.github.io/docs</a>.<br> <br>
  Do you like this project? If so, please let me know!
  <a href="https://sevvy-wevvy.com/discord">Discord</a>
</p>

<p align="center">
  <a href="https://goome-ui.github.io/docs/">Docs</a> &nbsp;·&nbsp;
  <a href="https://github.com/goome-ui/goome-cli">goome-cli</a> &nbsp;·&nbsp;
  <a href="https://github.com/goome-ui/rust-goome">rust-goome</a> &nbsp;·&nbsp;
  <a href="https://github.com/goome-ui/examples">Examples</a>
</p>

<p align="center">
  The <a href="https://goome-ui.github.io/docs/downloads/">ready made apps</a> on the <a href="https://github.com/goome-ui/examples/releases/latest">examples release</a> show what you can build from here.<br>
  If you get stuck, ask in <a href="https://github.com/goome-ui/c-goome/discussions">Discussions</a>. The <a href="https://github.com/goome-ui/rust-goome/labels/good%20first%20issue">good first issues</a> on rust-goome are small ones if you want to help.
</p>

<details>
<summary align="center"><b>🚀 Make Your Own 🚀</b></summary>

<br>

<p align="center">
<b>Use this template</b> on GitHub gives you this repo as it is. With goome-cli you pick the name and which template you want:
</p>

```sh
pip install git+https://github.com/goome-ui/goome-cli
goome new my-thing --lang rust
goome new my-thing --lang rust --template hud
```

<p align="center">
It swaps "My Goome App" and friends for your name everywhere, files too, and leaves out the template only stuff.
</p>
</details>

<details>
<summary align="center"><b>🔨 Build And Run 🔨</b></summary>

<br>

<p align="center">
You need Rust 1.77 or newer (<a href="https://rustup.rs/">rustup</a>) and a C compiler, cargo builds the goome library for your computer the first time (a few seconds). Cargo already needs one for a lot of crates so you probably have it. If not, on Debian/Ubuntu its <code>sudo apt install build-essential</code>, on Mac <code>xcode-select --install</code> and on windows the Visual Studio build tools (the same ones rustup asks for).
</p>

```sh
cargo run
```

<p align="center">
<code>cargo run --release</code> for the fast build. <code>Cargo.toml</code> takes goome from <a href="https://crates.io/crates/goome">crates.io</a>, pinned to <code>0.5.2</code>. Change the number to update.
</p>
</details>

<details>
<summary align="center"><b>🧩 Other Templates 🧩</b></summary>

<br>

<p align="center">
<code>variants/</code> has the other starting points. Each one has the files that replace the app ones, <code>src/main.rs</code> (and <code>Cargo.toml</code> for the hud, it needs chrono for the clock):<br> <br>
<b>overlay</b> - a see-through overlay over the whole desktop, only the window takes clicks. alt+h hides it<br>
<b>mod-menu</b> - a menu with toggles and sliders over whatever game is running, alt+m opens and closes it<br>
<b>hud</b> - a little panel in the corner with the time and fps, every click goes straight through it<br><br>
<code>goome new</code> puts the one you pick in place for you. By hand, copy whats in the folder over the top of this one and delete <code>variants/</code>.
</p>
</details>

<details>
<summary align="center"><b>🧪 Testing Without A Screen 🧪</b></summary>

<br>

<p align="center">
goome has a test backend that fakes a whole screen, so the app runs in CI with nothing on screen. The script says what the "user" does and you get a screenshot of every frame in <code>shots/</code>. This one clicks the toggle:
</p>

```sh
GOOME_BACKEND=test GOOME_TEST_SHOTS=shots/ GOOME_TEST_SCRIPT="screen 1280x720 1; 3 click 60,28; end 6" cargo run
```

<p align="center">
CI does that on linux, windows and mac on every push. The script format is in the <a href="https://github.com/goome-ui/c-goome/blob/main/docs/DESIGN.md">c-goome docs</a> under Test.
</p>
</details>

<details>
<summary align="center"><b>🛠 Pull Requests 🛠</b></summary>

<br>

<p align="center">
Want to help with this template? Make A pull request to <b>dev</b>!<br>
Keep it small, its supposed to be the least code you need to get going. Bugs in goome itself go to <a href="https://github.com/goome-ui/rust-goome">rust-goome</a>.<br> <br>
As long as it works, and doesnt vary off too far from the main idea its very likely it get accepted!<br>
Read <a href="CONTRIBUTING.md">CONTRIBUTING.md</a> before you send one.
</p>
</details>

<details>
<summary align="center"><b>⚖️ License ⚖️</b></summary>

<br>

<p align="center">
This template is under the <a href="LICENSE">GPL-3.0</a>, same as goome.<br> <br>
<sub>
• You can use it, change it and share it<br>
• If you share something made with it, you have to share your source too, under the same license<br>
• Keep the credits
</sub><br> <br>
In short: play fair, keep it open, and give credit where it's due.
</p>
</details>

<details>
<summary align="center"><b>🩷 Donate 🩷</b></summary>

<br>

<p align="center">
<a href="https://sevvy-wevvy.com/kofi">Ko-fi</a> (Preferred)<br>
<a href="https://sevvy-wevvy.com/patreon">Patreon</a><br>
<a href="https://sevvy-wevvy.com/throne">Throne</a>
</p>
</details>

<p align="center">
<b>🫶 Contributors 🫶</b><br>
Thanks to everyone who helped out!<br> <br>
<a href="https://github.com/goome-ui/goome-app-template-rust/graphs/contributors"><img src="https://contrib.rocks/image?repo=goome-ui/goome-app-template-rust" alt="contributors"></a>
</p>
