use std::f64::consts::PI;
use std::io::{Error, ErrorKind};
use std::time::{Duration, Instant};

use ratatui::style::Color;
use ratatui::symbols::Marker;
use ratatui::widgets::canvas::{Canvas, Line};
use ratatui::widgets::Block;
use ratatui::{crossterm::event, Frame};

fn main() {
    let mut terminal = ratatui::init();
    let mut i = 0;

    let tick_rate = Duration::from_millis(1000 / 60);
    let mut last_tick = Instant::now();
    let result = loop {
        if let Err(e) = terminal.draw(|frame| draw(frame, i)) {
            break Err(e);
        }
        let timeout = tick_rate.saturating_sub(last_tick.elapsed());

        i += 1;
        if i > 1000 {
            break Err(Error::new(ErrorKind::TimedOut, "timeout"));
        }

        match event::poll(timeout) {
            Err(e) => break Err(e),
            Ok(true) => {
                let key = event::read().expect("should not have failed");
                break Ok(key);
            }
            Ok(false) => {}
        }

        if last_tick.elapsed() >= tick_rate {
            // self.on_tick();
            last_tick = Instant::now();
        }
    };

    ratatui::restore();
    println!("{result:?}");
}

fn draw(frame: &mut Frame, _i: usize) {
    let area = frame.area();
    let widget = Canvas::default()
        .block(Block::bordered().title("Whatever"))
        .marker(Marker::Braille)
        .paint(|ctx| {
            let (hour, min, sec) = get_time();
            let hour = get_hand_pos(hour * 60. + min, 12. * 60.);
            let min = get_hand_pos(min * 60. + sec, 60. * 60.);
            let sec = get_hand_pos(sec, 60.);

            let hour_x = hour.sin() * 70.;
            let hour_y = hour.cos() * 70.;
            ctx.draw(&Line::new(0., 0., hour_x, hour_y, Color::Red));

            let min_x = min.sin() * 170.;
            let min_y = min.cos() * 170.;
            ctx.draw(&Line::new(0., 0., min_x, min_y, Color::Yellow));

            let sec_x = sec.sin() * 150.;
            let sec_y = sec.cos() * 150.;
            ctx.draw(&Line::new(0., 0., sec_x, sec_y, Color::Green));

            for hour in 1..=12 {
                let radians = 2. * PI * (hour as f64 / 12.);
                let x = radians.sin() * 190.;
                let y = radians.cos() * 190.;
                ctx.print(x, y, format!("{hour}"));
            }
        })
        .x_bounds([-200., 200.])
        .y_bounds([-200., 200.]);

    frame.render_widget(widget, area);
}

fn get_hand_pos(time: f64, range: f64) -> f64 {
    2. * PI * (time / range)
}

// returns tuple (hours, minutes, seconds)
fn get_time() -> (f64, f64, f64) {
    let time = unsafe { libc::time(std::ptr::null_mut()) };
    let localtime = unsafe { libc::localtime(&time as *const i64) };

    unsafe {
        (
            (*localtime).tm_hour as f64,
            (*localtime).tm_min as f64,
            (*localtime).tm_sec as f64,
        )
    }
}
