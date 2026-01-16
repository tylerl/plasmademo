use crossterm::{
    cursor, execute, queue,
    event::{self, Event, KeyCode},
    style::{self, Color, Stylize},
    terminal,
};
use rand::Rng;
use std::io::{stdout, Write, Result};
use std::time::{Duration, Instant};

// Adjust to your heart's content
const TARGET_FPS: u64 = 60;
const TARGET_FRAME_TIME: Duration = Duration::from_nanos(1_000_000_000 / TARGET_FPS);
const FPS_AVG_SAMPLES: usize = 20;
const ANIMATION_SPEED: f32 = 5.0;

fn main() -> Result<()> {
    let mut stdout = stdout();
    terminal::enable_raw_mode()?;
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

    let (width, height) = terminal::size()?;
    let start_time = Instant::now();

    let mut frame = 0.0f32;
    let mut this_frame = Instant::now();
    let mut last_frame = TARGET_FRAME_TIME;
    let mut frame_times: Vec<f32> = Vec::with_capacity(FPS_AVG_SAMPLES);

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

    let mut xma = TARGET_FRAME_TIME.as_secs_f32();

    loop {
        if frame_times.len() == FPS_AVG_SAMPLES {
            frame_times.remove(0);
        }
        frame_times.push(last_frame.as_secs_f32());
        let avg_frame_time = {
            frame_times.iter().sum::<f32>() / frame_times.len() as f32
        };
        xma = xma * 0.99 + last_frame.as_secs_f32() * 0.01;

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
            avg_frame_time.recip(),
            start_time.elapsed().as_secs_f32()
        );
        queue!(
            stdout,
            cursor::MoveTo((width - info.len() as u16) / 2, height - 1),
            style::PrintStyledContent(info.black().on_white())
        )?;
        stdout.flush()?;


        // Measure this frame's time and adjust sleep to hit target
        let render_time = this_frame.elapsed();
        let target_time = TARGET_FRAME_TIME.as_secs_f32();
        let error = xma - target_time;
        let adjusted_sleep = target_time - render_time.as_secs_f32() - error;

        if adjusted_sleep > 0.0 {
            std::thread::sleep(Duration::from_secs_f32(adjusted_sleep));
        }

        frame += last_frame.as_secs_f32() * ANIMATION_SPEED;
        last_frame = this_frame.elapsed();
        this_frame = Instant::now();
    }

    execute!(stdout, cursor::Show, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
