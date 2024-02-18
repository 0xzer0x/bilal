use crate::DateTime;
use islam::salah::{Prayer, PrayerTimes};
use owo_colors::OwoColorize;
use std::io::{self, Write};

#[derive(Debug)]
pub struct Printer {
    prayers: PrayerTimes,
    show_color: bool,
    json_format: bool,
}

impl Printer {
    pub const fn new(prayers: PrayerTimes, show_color: bool, json_format: bool) -> Self {
        Self {
            prayers,
            show_color,
            json_format,
        }
    }

    fn name(prayer: Prayer) -> String {
        match prayer {
            Prayer::Fajr | Prayer::FajrTomorrow => "Fajr".to_owned(),
            Prayer::Sherook => "Shurook".to_owned(),
            Prayer::Dohr => "Dhuhr".to_owned(),
            Prayer::Asr => "Asr".to_owned(),
            Prayer::Maghreb => "Maghrib".to_owned(),
            Prayer::Ishaa => "Ishaa".to_owned(),
        }
    }

    fn print(prayer_fmt: &str) {
        writeln!(io::stdout(), "{}", prayer_fmt).ok();
    }

    /// Show all prayers info.
    pub fn all(&self) -> Result<(), crate::Error> {
        let prayers = self.prayers;

        let fmt_output = |name: &str, time: DateTime| -> Result<String, crate::Error> {
            Ok(format!("{}: {}", name, time.format("%H:%M")))
        };

        Self::print(&fmt_output(&Self::name(Prayer::Fajr), prayers.fajr)?);
        Self::print(&fmt_output(&Self::name(Prayer::Sherook), prayers.sherook)?);
        Self::print(&fmt_output(&Self::name(Prayer::Dohr), prayers.dohr)?);
        Self::print(&fmt_output(&Self::name(Prayer::Asr), prayers.asr)?);
        Self::print(&fmt_output(&Self::name(Prayer::Maghreb), prayers.maghreb)?);
        Self::print(&fmt_output(&Self::name(Prayer::Ishaa), prayers.ishaa)?);
        Self::print(&fmt_output(
            "Fist third of night",
            prayers.first_third_of_night,
        )?);
        Self::print(&fmt_output("Midnight", prayers.midnight)?);
        Self::print(&fmt_output(
            "Last third of night",
            prayers.last_third_of_night,
        )?);

        Ok(())
    }
    /// Show current prayer info
    pub fn current(&self) -> Result<(), crate::Error> {
        let prayers = self.prayers;
        let prayer = prayers.current();
        let (hour, minute) = prayers.time_remaining();

        let remaining_fmt = {
            if hour == 0 {
                format!("({:0>2} minutes left)", minute)
            } else {
                format!("({:0>2}:{:0>2} hours left)", hour, minute)
            }
        };

        // default
        let mut prayer_fmt = format!("{} {}", Self::name(prayer), remaining_fmt);
        let state = {
            if hour == 0 && minute < 30 {
                "Critical"
            } else {
                "Info"
            }
        };

        // JSON
        if self.json_format {
            prayer_fmt = format!(
                r#"{{"icon": "{}", "state": "{}", "text": "{} {}"}}"#,
                "bilal", state, "\u{23fa} ", prayer_fmt
            );
        }
        // color
        if self.show_color && state == "Critical" && !self.json_format {
            prayer_fmt = format!("{}", prayer_fmt.red());
        }
        Self::print(&prayer_fmt);
        Ok(())
    }

    /// Show next prayer info
    pub fn next(&self) -> Result<(), crate::Error> {
        let prayers = self.prayers;
        let prayer = prayers.next();
        let time = prayers.time(prayer);
        let time = time.format("%I:%M %p").to_string();

        // default
        let mut prayer_fmt = format!("{} ({})", Self::name(prayer), time);

        // JSON
        let state = "Info";
        if self.json_format {
            prayer_fmt = format!(
                r#"{{"icon": "{}", "state": "{}", "text": "{} {}"}}"#,
                "bilal", state, "\u{25b6}", prayer_fmt
            );
        }
        Self::print(&prayer_fmt);
        Ok(())
    }

    pub fn next_remaining(&self) -> Result<(), crate::Error> {
        let prayers = &self.prayers;
        let prayer = prayers.next();
        let (hour, minute) = prayers.time_remaining();

        let prayer_fmt = format!("{} in {:0>2}:{:0>2}", Self::name(prayer), hour, minute);
        Self::print(&prayer_fmt);
        Ok(())
    }
}
