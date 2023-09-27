//! 7 days to die のsettimeコマンドによる日付設定を楽にします。
//!
//! # Install
//!
//! ```cmd
//! git clone <URL>
//! ```
//!
//! ```cmd
//! cargo install --path .
//! ```
//!
//! # Examples
//!
//! ```cmd
//! 7dtt 3 16 22
//! 64367
//! settime 64367
//! st 64367
//! ```
//!
//! # Uninstall
//!
//! ```cmd
//! cargo uninstall seven_days_to_time_rs
//! ```

use clap::CommandFactory;
use clap::Parser;
use std::fmt;

#[derive(Parser, Debug)]
struct Arguments {
    /// 日を指定します。
    day: usize,

    /// 時間を指定します。
    #[arg(default_value = "0")]
    hour: usize,

    /// 分を指定します。
    #[arg(default_value = "0")]
    minute: usize,
}

struct SevendaysDatetimes {
    day: u8,
    hour: u8,
    minute: u8,
}

impl SevendaysDatetimes {
    fn new(day: usize, hour: usize, minute: usize) -> Self {
        Self {
            day: day as u8,
            hour: hour as u8,
            minute: minute as u8,
        }
    }
}

trait Calc {
    /// convert `sevendays_datetimes` to utc time.
    fn to_utctime(&self) -> Option<u32>;
}

impl Calc for SevendaysDatetimes {
    fn to_utctime(&self) -> Option<u32> {
        let day = (self.day as u32 - 1) * 24000;
        let hour = self.hour as u32 * 1000;
        let minute = (self.minute as f32 * (1000.0 / 60.0)).ceil();
        Some(day + hour + minute as u32)
    }
}

impl fmt::Display for SevendaysDatetimes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "SevenDaysDatetimes")?;
        writeln!(f, "day=\"{}\"", self.day)?;
        writeln!(f, "hour=\"{}\"", self.hour)?;
        writeln!(f, "minute=\"{}\"", self.minute)
    }
}

fn main() {
    let arguments = match Arguments::try_parse() {
        Ok(ok) => ok,
        Err(_err) => {
            Arguments::command()
                .name("7dtt")
                .print_long_help()
                .unwrap();
            return;
        }
    };
    match arguments.day {
        0 => {
            println!("arguments \"day\" must be set to higher than 0 (1~)");
            return;
        }
        _ => {}
    }
    let svdtt = SevendaysDatetimes::new(arguments.day, arguments.hour, arguments.minute);
    if let Some(utc) = svdtt.to_utctime() {
        println!("{}", utc);
        println!("settime {}", utc);
        println!("st {}", utc);
    }
}
