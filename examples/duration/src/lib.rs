#[macro_use]
extern crate helix_runtime as helix;

use std::fmt::Write;
use std::cmp::Ordering;

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

        def initialize(helix, seconds: Option<i32>, minutes: Option<i32>, hours: Option<i32>, days: Option<i32>, weeks: Option<i32>, months: Option<i32>, years: Option<i32>) {
            let mut duration = Duration {
                helix:   helix,
                seconds: seconds,
                minutes: minutes,
                hours:   hours,
                days:    days,
                weeks:   weeks,
                months:  months,
                years:   years,
                value:   0,
            };

            duration.value = compute_value(&duration);

            duration
        }

        def seconds(seconds: i32) -> Duration {
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

        def value(&self) -> i64 {
            self.value
        }

        def plus(&self, other: &Duration) -> Duration {
            Duration::new(
                sum_part(self.seconds, other.seconds),
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
                negate_part(self.seconds),
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

        def cmp(&self, other: &Duration) -> i32 {
            match self.value.cmp(&other.value) {
                Ordering::Less => -1,
                Ordering::Equal => 0,
                Ordering::Greater => 1
            }
        }

        def to_i(&self) -> i64 {
            self.value
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
            format_inspect_part(&mut parts, self.seconds, "second", "seconds");

            to_sentence(parts)
        }

        def iso8601(&self) -> String {
            if self.value == 0 {
                return "PT0S".to_string();
            }

            let mut output = String::new();

            let sign = if
                self.value < 0 &&
                self.seconds.unwrap_or(-1) < 0 &&
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

            if self.hours.unwrap_or(0) + self.minutes.unwrap_or(0) + self.seconds.unwrap_or(0) != 0 {
                output.push('T');

                format_iso8601_part(&mut output, sign, self.hours, "H");
                format_iso8601_part(&mut output, sign, self.minutes, "M");
                format_iso8601_part(&mut output, sign, self.seconds, "S");
            }

            output
        }
    }
}

fn format_iso8601_part(string: &mut String, sign: i32, value: Option<i32>, unit: &str) {
    if let Some(v) = value {
        if v != 0 {
            write!(string, "{}{}", sign * v, unit).unwrap();
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
        2 => format!("{} and {}", parts[0], parts[1]),
        _ => {
            let last = parts.pop().unwrap();
            format!("{}, and {}", parts.join(", "), last)
        }
    }
}
