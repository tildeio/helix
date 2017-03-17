#[macro_use]
extern crate helix_runtime as helix;

use std::fmt::Write;
use std::cmp::Ordering;
use std::collections::HashMap;
use helix::Symbol;

const SECONDS_PER_MINUTE: i64 = 60;
const SECONDS_PER_HOUR:   i64 = 3600;
const SECONDS_PER_DAY:    i64 = 86400;
const SECONDS_PER_WEEK:   i64 = 604800;
const SECONDS_PER_MONTH:  i64 = 2629746; // 1/12 of a gregorian year
const SECONDS_PER_YEAR:   i64 = 31556952; // length of a gregorian year (365.2425 days)

declare_types! {
    class Duration {
        struct {
            // TODO: Consider implementing a `Number` struct that behaves more like Ruby in coercing between types
            seconds: Option<f64>,
            minutes: Option<i32>,
            hours:   Option<i32>,
            days:    Option<i32>,
            weeks:   Option<i32>,
            months:  Option<i32>,
            years:   Option<i32>,
            value:   f64,
        }

        def initialize(helix, seconds: Option<f64>, minutes: Option<i32>, hours: Option<i32>, days: Option<i32>, weeks: Option<i32>, months: Option<i32>, years: Option<i32>) {
            let mut duration = Duration {
                helix:   helix,
                seconds: seconds,
                minutes: minutes,
                hours:   hours,
                days:    days,
                weeks:   weeks,
                months:  months,
                years:   years,
                value:   0.0,
            };

            duration.value = compute_value(&duration);

            duration
        }

        def parse(string: String) -> Duration {
            Duration::seconds(1_f64)
        }

        def seconds(seconds: f64) -> Duration {
            Duration::new(Some(seconds), None, None, None, None, None, None)
        }

        def minutes(minutes: i32) -> Duration {
            Duration::new(None, Some(minutes), None, None, None, None, None)
        }

        def hours(hours: i32) -> Duration {
            Duration::new(None, None, Some(hours), None, None, None, None)
        }

        def days(days: i32) -> Duration {
            Duration::new(None, None, None, Some(days), None, None, None)
        }

        def weeks(weeks: i32) -> Duration {
            Duration::new(None, None, None, None, Some(weeks), None, None)
        }

        def months(months: i32) -> Duration {
            Duration::new(None, None, None, None, None, Some(months), None)
        }

        def years(years: i32) -> Duration {
            Duration::new(None, None, None, None, None, None, Some(years))
        }

        def value(&self) -> f64 {
            self.value
        }

        // FIXME: Having to convert to f64 isn't ideal
        def parts(&self) -> HashMap<Symbol, f64> {
            let mut map = HashMap::new();
            if let Some(v) = self.seconds { map.insert(Symbol::new(String::from("seconds")), v); }
            if let Some(v) = self.minutes { map.insert(Symbol::new(String::from("minutes")), v as f64); }
            if let Some(v) = self.hours   { map.insert(Symbol::new(String::from("hours")), v as f64); }
            if let Some(v) = self.days    { map.insert(Symbol::new(String::from("days")), v as f64); }
            if let Some(v) = self.weeks   { map.insert(Symbol::new(String::from("weeks")), v as f64); }
            if let Some(v) = self.months  { map.insert(Symbol::new(String::from("months")), v as f64); }
            if let Some(v) = self.years   { map.insert(Symbol::new(String::from("years")), v as f64); }
            map
        }

        def plus(&self, other: &Duration) -> Duration {
            Duration::new(
                sum_float_part(self.seconds, other.seconds),
                sum_part(self.minutes, other.minutes),
                sum_part(self.hours, other.hours),
                sum_part(self.days, other.days),
                sum_part(self.weeks, other.weeks),
                sum_part(self.months, other.months),
                sum_part(self.years, other.years)
            )
        }

        def minus(&self, other: &Duration) -> Duration {
            self.plus(&other.negate())
        }

        def negate(&self) -> Duration {
            Duration::new(
                self.seconds.map(|s| -s),
                negate_part(self.minutes),
                negate_part(self.hours),
                negate_part(self.days),
                negate_part(self.weeks),
                negate_part(self.months),
                negate_part(self.years)
            )
        }

        def eq(&self, other: &Duration) -> bool {
            self.value == other.value
        }

        def cmp(&self, other: &Duration) -> Option<i32> {
            match self.value.partial_cmp(&other.value) {
                Some(Ordering::Less) => Some(-1),
                Some(Ordering::Equal) => Some(0),
                Some(Ordering::Greater) => Some(1),
                None => None
            }
        }

        def to_i(&self) -> i64 {
            self.value.round() as i64
        }

        def to_s(&self) -> String {
            format!("{}", self.value)
        }

        def inspect(&self) -> String {
            let mut parts = Vec::new();

            format_inspect_part(&mut parts, self.years, "year", "years");
            format_inspect_part(&mut parts, self.months, "month", "months");
            format_inspect_part(&mut parts, self.weeks, "week", "weeks");
            format_inspect_part(&mut parts, self.days, "day", "days");
            format_inspect_part(&mut parts, self.hours, "hour", "hours");
            format_inspect_part(&mut parts, self.minutes, "minute", "minutes");
            format_inspect_float_part(&mut parts, self.seconds, "second", "seconds");

            to_sentence(parts)
        }

        def iso8601_precise(&self, precision: Option<i32>) -> String {
            if self.value == 0.0 {
                return "PT0S".to_string();
            }

            let mut output = String::new();

            let sign = if
                self.value < 0.0 &&
                self.seconds.unwrap_or(-1.0) < 0.0 &&
                self.minutes.unwrap_or(-1) < 0 &&
                self.hours.unwrap_or(-1) < 0 &&
                self.days.unwrap_or(-1) < 0 &&
                self.weeks.unwrap_or(-1) < 0 &&
                self.months.unwrap_or(-1) < 0 &&
                self.years.unwrap_or(-1) < 0 {
                -1
            } else {
                1
            };

            if sign == -1 {
                output.push('-');
            }

            output.push('P');

            format_iso8601_part(&mut output, sign, self.years, "Y");
            format_iso8601_part(&mut output, sign, self.months, "M");
            format_iso8601_part(&mut output, sign, self.weeks, "W");
            format_iso8601_part(&mut output, sign, self.days, "D");

            if self.hours.map(|v| v as f64).unwrap_or(0.0) + self.minutes.map(|v| v as f64).unwrap_or(0.0) + self.seconds.unwrap_or(0.0) != 0.0 {
                output.push('T');

                format_iso8601_part(&mut output, sign, self.hours, "H");
                format_iso8601_part(&mut output, sign, self.minutes, "M");
                format_iso8601_float_part(&mut output, sign, self.seconds, "S", precision);
            }

            output
        }
    }
}

fn precise_str(val: f64, precision: i32) -> String {
    let mult = 10.0_f64.powi(precision);
    let mut str = format!("{}", ((val * mult).round() / mult));
    if precision > 0 && !str.contains(".") {
        str.push('.');
    }
    let diff = if precision > 0 {
        let parts: Vec<&str> = str.split('.').collect();
        let post = parts.last().unwrap();
        precision - post.len() as i32
    } else {
        0
    };
    for _ in 0..diff {
        str.push('0');
    }
    str
}

fn format_iso8601_part(string: &mut String, sign: i32, value: Option<i32>, unit: &str) {
    if let Some(v) = value {
        if v != 0 {
            write!(string, "{}{}", sign * v, unit).unwrap()
        }
    }
}

fn format_iso8601_float_part(string: &mut String, sign: i32, value: Option<f64>, unit: &str, precision: Option<i32>) {
    if let Some(v) = value {
        if v != 0.0 {
            let signed = (sign as f64) * v;
            match precision {
                Some(p) => write!(string, "{}{}", precise_str(signed, p), unit).unwrap(),
                None    => write!(string, "{}{}", signed, unit).unwrap()
            };
        }
    }
}

// TODO: Avoid duplication here
fn format_inspect_part(parts: &mut Vec<String>, value: Option<i32>, singular: &str, plural: &str) {
    if let Some(v) = value {
        parts.push(format!("{} {}", v, if v == 1 { singular } else { plural }));
    }
}

fn format_inspect_float_part(parts: &mut Vec<String>, value: Option<f64>, singular: &str, plural: &str) {
    if let Some(v) = value {
        parts.push(format!("{} {}", v, if v == 1.0 { singular } else { plural }));
    }
}

// TODO: over/under flow bug
fn sum_part(lhs: Option<i32>, rhs: Option<i32>) -> Option<i32> {
    match lhs {
        Some(lval) => {
            match rhs {
                Some(rval) => Some(lval + rval),
                None => Some(lval)
            }
        },
        None => rhs
    }
}

// TODO: over/under flow bug
// TODO: Avoid duplication here
fn sum_float_part(lhs: Option<f64>, rhs: Option<f64>) -> Option<f64> {
    match lhs {
        Some(lval) => {
            match rhs {
                Some(rval) => Some(lval + rval),
                None => Some(lval)
            }
        },
        None => rhs
    }
}


fn negate_part(part: Option<i32>) -> Option<i32> {
    match part {
        Some(value) => Some(-value),
        None => None
    }
}

fn compute_part_value(part: Option<i32>, unit: i64) -> i64 {
    match part {
        Some(value) => value as i64 * unit,
        None => 0
    }
}

fn compute_value(duration: &Duration) -> f64 {
    let mut value = 0.0;

    value += duration.seconds.unwrap_or(0.0);
    value += compute_part_value(duration.minutes, SECONDS_PER_MINUTE) as f64;
    value += compute_part_value(duration.hours, SECONDS_PER_HOUR) as f64;
    value += compute_part_value(duration.days, SECONDS_PER_DAY) as f64;
    value += compute_part_value(duration.weeks, SECONDS_PER_WEEK) as f64;
    value += compute_part_value(duration.months, SECONDS_PER_MONTH) as f64;
    value += compute_part_value(duration.years, SECONDS_PER_YEAR) as f64;

    value
}

fn to_sentence(mut parts: Vec<String>) -> String {
    match parts.len() {
        0 => "".to_string(),
        1 => parts.pop().unwrap(),
        2 => format!("{} and {}", parts[0], parts[1]),
        _ => {
            let last = parts.pop().unwrap();
            format!("{}, and {}", parts.join(", "), last)
        }
    }
}
