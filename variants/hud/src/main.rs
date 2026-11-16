use goome::{Draw, Options, Pacer};

const APP_NAME: &str = "My Goome App";

fn main() {
    let mut g = match Options::new().title(APP_NAME).open() {
        Ok(g) => g,
        Err(e) => {
            eprintln!("couldnt open an overlay: {}", e);
            std::process::exit(1);
        }
    };
    let _ = g.hotkey("alt+h", 1);
    let start = goome::now_ms();
    let mut last = start;
    let (mut fps, mut hidden, mut running) = (0.0, false, true);
    let mut pace = Pacer::new(30);
    while running {
        for ev in g.events(pace.wait()) {
            if ev.kind == goome::QUIT {
                running = false;
            }
            if ev.kind == goome::HOTKEY {
                hidden = !hidden;
            }
        }
        if !running {
            break;
        }
        let now = goome::now_ms();
        if now > last {
            fps = fps * 0.9 + 100.0 / (now - last) as f64;
        }
        last = now;
        let Some(mut c) = g.begin(0) else { continue };
        if hidden {
            continue;
        }
        c.round_rect(20.0, 20.0, 160.0, 92.0, 10.0, 0xB0141820);
        c.text_big(34.0, 30.0, &chrono::Local::now().format("%H:%M:%S").to_string(), 0xFFFFFFFF);
        c.text(34.0, 62.0, &format!("up {}s", (now - start) / 1000), 0xFFC0C8D0);
        c.text(34.0, 84.0, &format!("{:.0} fps", fps), 0xFF6FDC5F);
    }
}
