use goome::{Draw, Goome, Options, WindowState};

const APP_NAME: &str = "My Goome App";

fn main() {
    let mut ui = match Options::new().title(APP_NAME).open().and_then(Goome::ui) {
        Ok(u) => u,
        Err(e) => {
            eprintln!("couldnt open an overlay: {}", e);
            std::process::exit(1);
        }
    };
    let _ = ui.hotkey("alt+m", 1);
    let mut menu = WindowState::new(80.0, 80.0, 300.0, 0.0);
    menu.open = false;
    let (mut fly, mut god, mut speed) = (false, false, 1.0);
    let mut running = true;
    while running {
        for ev in ui.pump(16) {
            if ev.kind == goome::QUIT {
                running = false;
            }
            if ev.kind == goome::HOTKEY {
                menu.open = !menu.open;
            }
        }
        if !running {
            break;
        }
        let Some(mut f) = ui.begin(0) else { continue };
        if menu.open {
            let mut w = f.window(APP_NAME, &mut menu, goome::WIN_MOVE | goome::WIN_CLOSE);
            if w.shown() {
                w.toggle("Fly", &mut fly);
                w.checkbox("God mode", &mut god);
                w.slider_fmt("Speed", &mut speed, 1.0, 10.0, Some("%.1f"));
                if w.button("Respawn") {
                    w.toast("respawned", 2000);
                }
            }
        } else {
            let mut c = f.screen();
            c.round_rect(20.0, 20.0, 170.0, 30.0, 8.0, 0xC0202838);
            c.text(32.0, 26.0, "alt+m opens the menu", 0xFFFFFFFF);
        }
    }
}
