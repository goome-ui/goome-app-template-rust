use goome::{Goome, Options, WindowState};

const APP_NAME: &str = "My Goome App";

fn main() {
    let mut ui = match Options::new().title(APP_NAME).open().and_then(Goome::ui) {
        Ok(u) => u,
        Err(e) => {
            eprintln!("couldnt open an overlay: {}", e);
            std::process::exit(1);
        }
    };
    let _ = ui.hotkey("alt+h", 1);
    let mut win = WindowState::new(60.0, 60.0, 280.0, 0.0);
    let (mut clicks, mut hidden) = (0, false);
    while win.open {
        for ev in ui.pump(16) {
            if ev.kind == goome::QUIT {
                win.open = false;
            }
            if ev.kind == goome::HOTKEY {
                hidden = !hidden;
            }
        }
        if !win.open {
            continue;
        }
        let Some(mut f) = ui.begin(0) else { continue };
        if !hidden {
            let mut w = f.window(APP_NAME, &mut win, goome::WIN_MOVE | goome::WIN_CLOSE);
            if w.shown() {
                w.label("im on top of everything");
                if w.button("Click me") {
                    clicks += 1;
                }
                w.label(&format!("clicked {} times", clicks));
                w.label("alt+h hides me");
            }
        }
    }
}
