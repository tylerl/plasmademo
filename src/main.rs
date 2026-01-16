use crossterm::{
    cursor, execute, queue,
    event::{self, Event, KeyCode},
    style::{self, Color, Stylize},
    terminal,
};
use rand::Rng;
use std::io::{stdout, Write, Result};
use std::time::{Duration, Instant};

fn main() -> Result<()> {
    let mut stdout = stdout();
    terminal::enable_raw_mode()?;
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

    let (width, height) = terminal::size()?;
    let (mut frame, start_time, mut last_frame, mut fps) =
        (0.0f32, Instant::now(), Instant::now(), 0.0f32);

    let mut rng = rand::thread_rng();
    let (f1, f2, f3) = (
        rng.gen_range(5.0..15.0),
        rng.gen_range(5.0..15.0),
        rng.gen_range(6.0..10.0),
    );
    let (d1, d2, d3, d4) = (
        rng.gen_range(-1.0..=1.0),
        rng.gen_range(-1.0..=1.0),
        rng.gen_range(-1.0..=1.0),
        rng.gen_range(-1.0..=1.0),
    );
    let phase = rng.gen_range(0.0..6.28);

    loop {
        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(key) = event::read()? {
                if matches!(key.code, KeyCode::Char('q') | KeyCode::Esc) {
                    break;
                }
            }
        }

        for y in 0..height {
            for x in 0..width {
                let (fx, fy) = (x as f32 / width as f32, y as f32 / height as f32);
                let t = frame;
                let v = (
                    (fx * f1 + t * d1).sin() +
                    (fy * f2 + t * d2).sin() +
                    ((fx * f1 + fy * f2 + t * d3) / 2.0).sin() +
                    ((fx * fx + fy * fy).sqrt() * f3 + t * d4).sin()
                ) / 4.0;

                let n = ((v + 1.0) / 2.0 * 255.0) as u8;
                let hue = n as f32 / 255.0 * 6.28 + t + phase;
                let color = Color::Rgb {
                    r: (hue.sin() * 127.5 + 127.5) as u8,
                    g: ((hue + 2.09).sin() * 127.5 + 127.5) as u8,
                    b: ((hue + 4.19).sin() * 127.5 + 127.5) as u8,
                };
                let c = [' ', '░', '▒', '▓'][(n / 64).min(3) as usize];

                queue!(
                    stdout,
                    cursor::MoveTo(x, y),
                    style::PrintStyledContent(c.with(color))
                )?;
            }
        }

        let info = format!(
            " PLASMA WAVE | FPS: {:.1} | Time: {:.1}s | Press Q to exit ",
            fps,
            start_time.elapsed().as_secs_f32()
        );
        queue!(
            stdout,
            cursor::MoveTo((width - info.len() as u16) / 2, height - 1),
            style::PrintStyledContent(info.black().on_white())
        )?;
        stdout.flush()?;

        fps = 1.0 / last_frame.elapsed().as_secs_f32();
        last_frame = Instant::now();
        frame += 0.05;
        std::thread::sleep(Duration::from_millis(16));
    }

    execute!(stdout, cursor::Show, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
