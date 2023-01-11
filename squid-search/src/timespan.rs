use std::str::FromStr;

use crate::error::Error;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
pub struct TimeSpan(i64);

impl TimeSpan {
    pub const fn seconds(num: i64) -> Self {
        Self(num)
    }
    pub const fn minutes(num: i64) -> Self {
        Self(num * NUM_SECONDS_IN_ONE_MINUTE)
    }
    pub const fn hours(num: i64) -> Self {
        Self(num * NUM_SECONDS_IN_ONE_HOUR)
    }
    pub const fn days(num: i64) -> Self {
        Self(num * NUM_SECONDS_IN_ONE_DAY)
    }
    pub const fn years(num: i64) -> Self {
        Self(num * NUM_SECONDS_IN_ONE_YEAR)
    }
}

const NUM_SECONDS_IN_ONE_MINUTE: i64 = 60;
const NUM_SECONDS_IN_ONE_HOUR: i64 = NUM_SECONDS_IN_ONE_MINUTE * 60;
const NUM_SECONDS_IN_ONE_DAY: i64 = NUM_SECONDS_IN_ONE_HOUR * 24;
const NUM_SECONDS_IN_ONE_WEEK: i64 = NUM_SECONDS_IN_ONE_DAY * 7;
const NUM_SECONDS_IN_ONE_YEAR: i64 = NUM_SECONDS_IN_ONE_DAY * 365;

impl From<i64> for TimeSpan {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl From<TimeSpan> for i64 {
    fn from(value: TimeSpan) -> Self {
        value.0
    }
}

fn get_value_suffix(s: &str) -> Result<Option<(i64, &str, &str)>, Error> {
    if s.is_empty() {
        return Ok(None);
    }

    let (value, s) = if let Some(non_numeric_index) = s.find(|c: char| !c.is_ascii_digit()) {
        let (value_str, rest) = s.split_at(non_numeric_index);
        (value_str.parse()?, rest)
    } else {
        (1i64, s)
    };

    let (suffix, rest) = if let Some(numeric_index) = s.find(|c: char| c.is_ascii_digit()) {
        s.split_at(numeric_index)
    } else {
        (s, "")
    };

    Ok(Some((value, suffix, rest)))
}

impl FromStr for TimeSpan {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut offset = 0;
        let mut s = s;

        while let Some((value, suffix, rest)) = get_value_suffix(s)? {
            let value = match suffix {
                "" | "s" => value,
                "m" => value * NUM_SECONDS_IN_ONE_MINUTE,
                "h" => value * NUM_SECONDS_IN_ONE_HOUR,
                "d" => value * NUM_SECONDS_IN_ONE_DAY,
                "w" => value * NUM_SECONDS_IN_ONE_WEEK,
                "y" => value * NUM_SECONDS_IN_ONE_YEAR,
                _ => return Err(Error::InvalidTimeSuffix(suffix.into())),
            };
            offset += value;
            s = rest;
        }

        Ok(offset.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_time() {
        let spec = "1d2h03m10s"
            .parse::<TimeSpan>()
            .expect("Invalid spec for TimeSpan?!");
        let offset = (NUM_SECONDS_IN_ONE_DAY
            + 2 * NUM_SECONDS_IN_ONE_HOUR
            + 3 * NUM_SECONDS_IN_ONE_MINUTE
            + 10)
            .into();

        assert_eq!(spec, offset);
    }
}
