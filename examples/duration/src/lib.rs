#[macro_use]
extern crate helix;
extern crate rand;

use std::fmt::Write;
use rand::random;

const SECONDS_PER_MINUTE: i64 = 60;
const SECONDS_PER_HOUR:   i64 = 3600;
const SECONDS_PER_DAY:    i64 = 86400;
const SECONDS_PER_WEEK:   i64 = 604800;
const SECONDS_PER_MONTH:  i64 = 2629746; // 1/12 of a gregorian year
const SECONDS_PER_YEAR:   i64 = 31556952; // length of a gregorian year (365.2425 days)

declare_types! {
    class Duration {
        struct {
            seconds: Option<i32>,
            minutes: Option<i32>,
            hours:   Option<i32>,
            days:    Option<i32>,
            weeks:   Option<i32>,
            months:  Option<i32>,
            years:   Option<i32>,
            value:   i64,
        }

        def initialize(helix: helix::Metadata) {
            Duration {
                helix:   helix,
                seconds: Some(0),
                minutes: None,
                hours:   None,
                days:    None,
                weeks:   None,
                months:  None,
                years:   None,
                value:   0,
            }
        }

        def seconds(&self) -> Option<i32> {
            self.seconds
        }

        def set_seconds(&mut self, seconds: Option<i32>) {
            self.seconds = seconds;
        }

        def minutes(&self) -> Option<i32> {
            self.minutes
        }

        def set_minutes(&mut self, minutes: Option<i32>) {
            self.minutes = minutes;
        }

        def hours(&self) -> Option<i32> {
            self.hours
        }

        def set_hours(&mut self, hours: Option<i32>) {
            self.hours = hours;
        }

        def days(&self) -> Option<i32> {
            self.days
        }

        def set_days(&mut self, days: Option<i32>) {
            self.days = days;
        }

        def weeks(&self) -> Option<i32> {
            self.weeks
        }

        def set_weeks(&mut self, weeks: Option<i32>) {
            self.weeks = weeks;
        }

        def months(&self) -> Option<i32> {
            self.months
        }

        def set_months(&mut self, months: Option<i32>) {
            self.months = months;
        }

        def years(&self) -> Option<i32> {
            self.years
        }

        def set_years(&mut self, years: Option<i32>) {
            self.years = years;
        }

        def value(&self) -> i64 {
            self.value
        }

        def plus(&mut self, other: &Duration) {
            self.seconds = sum_part(self.seconds, other.seconds);
            self.minutes = sum_part(self.minutes, other.minutes);
            self.hours = sum_part(self.hours, other.hours);
            self.days = sum_part(self.days, other.days);
            self.weeks = sum_part(self.weeks, other.weeks);
            self.months = sum_part(self.months, other.months);
            self.years = sum_part(self.years, other.years);

            self.value += other.value;
        }

        def negate(&mut self) {
            self.seconds = negate_part(self.seconds);
            self.minutes = negate_part(self.minutes);
            self.hours = negate_part(self.hours);
            self.days = negate_part(self.days);
            self.weeks = negate_part(self.weeks);
            self.months = negate_part(self.months);
            self.years = negate_part(self.years);

            self.value *= -1;
        }

        def randomize(&mut self) {
            self.seconds = randomize_part();
            self.minutes = randomize_part();
            self.hours = randomize_part();
            self.days = randomize_part();
            self.weeks = randomize_part();
            self.months = randomize_part();
            self.years = randomize_part();

            self.value = compute_value(&self);
        }

        def iso8601(&self) -> String {
            if self.value == 0 {
                return "PT0S".to_string();
            }

            let mut output = String::new();

            let sign = if
                self.value < 0 &&
                self.seconds.unwrap_or(0) < 0 &&
                self.minutes.unwrap_or(0) < 0 &&
                self.hours.unwrap_or(0) < 0 &&
                self.days.unwrap_or(0) < 0 &&
                self.weeks.unwrap_or(0) < 0 &&
                self.months.unwrap_or(0) < 0 &&
                self.years.unwrap_or(0) < 0 {
                -1
            } else {
                1
            };

            if sign == -1 {
                output.push('-');
            }

            output.push('P');

            format_iso8601_part(&mut output, self.years, "Y");
            format_iso8601_part(&mut output, self.months, "M");
            format_iso8601_part(&mut output, self.weeks, "W");
            format_iso8601_part(&mut output, self.days, "D");

            if self.hours.unwrap_or(0) + self.minutes.unwrap_or(0) + self.seconds.unwrap_or(0) != 0 {
                output.push('T');

                format_iso8601_part(&mut output, self.hours, "H");
                format_iso8601_part(&mut output, self.minutes, "M");
                format_iso8601_part(&mut output, self.seconds, "S");
            }

            output
        }

        def inspect(&self) -> String {
            let mut parts = Vec::new();

            format_inspect_part(&mut parts, self.years, "year", "years");
            format_inspect_part(&mut parts, self.months, "month", "months");
            format_inspect_part(&mut parts, self.weeks, "week", "weeks");
            format_inspect_part(&mut parts, self.days, "day", "days");
            format_inspect_part(&mut parts, self.hours, "hour", "hours");
            format_inspect_part(&mut parts, self.minutes, "minute", "minutes");
            format_inspect_part(&mut parts, self.seconds, "second", "seconds");

            to_sentence(parts)
        }
    }
}

fn format_iso8601_part(string: &mut String, value: Option<i32>, unit: &str) {
    if let Some(v) = value {
        if v != 0 {
            write!(string, "{}{}", v, unit).unwrap();
        }
    }
}

fn format_inspect_part(parts: &mut Vec<String>, value: Option<i32>, singular: &str, plural: &str) {
    if let Some(v) = value {
        parts.push(format!("{} {}", v, if v == 1 { singular } else { plural }));
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

fn negate_part(part: Option<i32>) -> Option<i32> {
    match part {
        Some(value) => Some(-value),
        None => None
    }
}

fn randomize_part() -> Option<i32> {
    if random() {
        Some(random::<i8>() as i32)
    } else {
        None
    }
}

fn compute_part_value(part: Option<i32>, unit: i64) -> i64 {
    match part {
        Some(value) => value as i64 * unit,
        None => 0
    }
}

fn compute_value(duration: &Duration) -> i64 {
    let mut value = 0;

    value += compute_part_value(duration.seconds, 1);
    value += compute_part_value(duration.minutes, SECONDS_PER_MINUTE);
    value += compute_part_value(duration.hours, SECONDS_PER_HOUR);
    value += compute_part_value(duration.days, SECONDS_PER_DAY);
    value += compute_part_value(duration.weeks, SECONDS_PER_WEEK);
    value += compute_part_value(duration.months, SECONDS_PER_MONTH);
    value += compute_part_value(duration.years, SECONDS_PER_YEAR);

    value
}

fn to_sentence(mut parts: Vec<String>) -> String {
    match parts.len() {
        0 => "".to_string(),
        1 => parts.pop().unwrap(),
        _ => {
            let last = parts.pop().unwrap();
            format!("{} and {}", parts.join(", "), last)
        }
    }
}
