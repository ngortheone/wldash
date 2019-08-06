use crate::buffer::Buffer;
use crate::color::Color;
use crate::draw::{Font, ROBOTO_REGULAR};
use crate::module::{Input, ModuleImpl};

use std::cell::RefCell;
use std::sync::mpsc::Sender;

use chrono::{DateTime, Datelike, Duration, Local, Timelike};

pub struct Clock {
    cur_time: DateTime<Local>,
    first_draw: bool,
    clock_cache: RefCell<Font>,
    date_cache: RefCell<Font>,
}

impl Clock {
    pub fn new(ch: Sender<bool>) -> Clock {
        std::thread::spawn(move || loop {
            let n = Local::now();
            let target = (n + Duration::seconds(60))
                .with_second(0)
                .unwrap()
                .with_nanosecond(0)
                .unwrap();

            let d = target - n;

            std::thread::sleep(d.to_std().unwrap());
            ch.send(true).unwrap();
        });

        Clock {
            cur_time: Local::now(),
            first_draw: true,
            date_cache: RefCell::new(Font::new(&ROBOTO_REGULAR, 64.0)),
            clock_cache: RefCell::new(Font::new(&ROBOTO_REGULAR, 256.0)),
        }
    }
}

impl ModuleImpl for Clock {
    fn draw(
        &self,
        buf: &mut Buffer,
        bg: &Color,
        time: &DateTime<Local>,
    ) -> Result<Vec<(i32, i32, i32, i32)>, ::std::io::Error> {
        buf.memset(bg);

        self.date_cache.borrow_mut().draw_text(
            &mut buf.subdimensions((0, 0, 448, 64))?,
            bg,
            &Color::new(1.0, 1.0, 1.0, 1.0),
            &format!(
                "{:?}, {:02}/{:02}/{:4}",
                time.weekday(),
                time.day(),
                time.month(),
                time.year()
            ),
        )?;

        self.clock_cache.borrow_mut().draw_text_fixed_width(
            &mut buf.subdimensions((0, 64, 288 * 2 + 64, 256))?,
            bg,
            &Color::new(1.0, 1.0, 1.0, 1.0),
            &[120, 120, 64, 120, 120],
            &format!("{:02}:{:02}", time.hour(), time.minute()),
        )?;

        Ok(vec![buf.get_signed_bounds()])
    }

    fn update(&mut self, time: &DateTime<Local>, force: bool) -> Result<bool, ::std::io::Error> {
        if time.date() != self.cur_time.date()
            || time.hour() != self.cur_time.hour()
            || time.minute() != self.cur_time.minute()
            || force || self.first_draw
        {
            self.cur_time = time.clone();
            self.first_draw = false;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn input(&mut self, _input: Input) {}
}