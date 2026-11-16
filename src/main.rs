use goome::{Goome, Options, Ui, WindowState};

const APP_NAME: &str = "My Goome App";

struct State {
    clicks: i32,
    volume: f64,
    overlay: bool,
    shown: bool,
}

fn open_mode(overlay: bool) -> goome::Result<(Ui, WindowState)> {
    if !overlay {
        return Ok((Goome::app(APP_NAME, 640, 420)?.ui()?, WindowState::new(0.0, 0.0, 0.0, 0.0)));
    }
    let g = Options::new().title(APP_NAME).open()?;
    let x = g.monitors().first().map_or(40, |m| m.area.x + m.area.w - 340);
    Ok((g.ui()?, WindowState::new(x as f64, 60.0, 300.0, 0.0)))
}

fn contents(ui: &mut Ui, s: &mut State) {
    ui.toggle("Overlay mode", &mut s.overlay);
    ui.separator();
    let text = ui.anim_color("hello", if s.shown { 0xFFF2F4F8 } else { 0x00F2F4F8 }, 600.0, goome::EASE_OUT);
    ui.push_color(goome::S_TEXT, text);
    s.shown = true;
    ui.heading(&format!("Hello from {}", APP_NAME));
    ui.pop(1);
    if ui.button("Click me") {
        s.clicks += 1;
    }
    ui.label(&format!("clicked {} times", s.clicks));
    ui.slider_fmt("Volume", &mut s.volume, 0.0, 1.0, Some("%.2f"));
}

fn main() {
    let mut s = State {
        clicks: 0,
        volume: 0.5,
        overlay: false,
        shown: false,
    };
    let mut mode = false;
    let (mut ui, mut win) = match open_mode(false) {
        Ok(o) => o,
        Err(e) => {
            eprintln!("couldnt open a window: {}", e);
            std::process::exit(1);
        }
    };
    let mut running = true;
    while running {
        for ev in ui.pump(16) {
            if ev.kind == goome::QUIT || ev.kind == goome::CLOSE {
                running = false;
            }
        }
        if !running {
            break;
        }
        if s.overlay != mode {
            drop(ui);
            (ui, win) = match open_mode(s.overlay) {
                Ok(o) => o,
                Err(_) => {
                    eprintln!("no overlay here, staying in window mode");
                    s.overlay = false;
                    match open_mode(false) {
                        Ok(o) => o,
                        Err(_) => std::process::exit(1),
                    }
                }
            };
            mode = s.overlay;
            println!("{} mode", if mode { "overlay" } else { "window" });
            continue;
        }
        let Some(mut f) = ui.begin(0) else { continue };
        if mode {
            let mut w = f.window(APP_NAME, &mut win, goome::WIN_MOVE | goome::WIN_COLLAPSE);
            if w.shown() {
                contents(&mut w, &mut s);
            }
        } else {
            let mut a = f.app(Some(APP_NAME), 0);
            contents(&mut a, &mut s);
        }
    }
}
