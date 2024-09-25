//! Generate a Ubiquitous General Purpose Tag to use in your notes, zettelkasten,
//! field research or archival project (photos, drafts, scrapbooking). Use it as a
//! date-identifier for your note's header or as a reference. You can use it between
//! media: Digital, typewritten, or handwritten. Reference a note in a book, in a
//! paper, or in your source- code. Everything with the same tag is connected.
//!
//! The tag contains a date, based on base54: The first letter is the year,
//! beginning from 2024, the second letter is the week of the year using ISO weeks.
//! The third letter is a 52nd of a week. It's primarily meant to give the tags an
//! order, but if you use the tags daily, you learn to read them.
//!
//! Example: `aQu-TWr` (a: year 2024, Q: week 39, u: tick 19)
//!
//! This is the no_std implementation.

#![no_std]
use core::fmt::{self, Result as FmtResult};

use chrono::{
    offset::LocalResult, DateTime, Datelike, Duration, NaiveDate, NaiveDateTime, TimeZone, Utc,
};

type Base54Type = [u8; 54];
const BASE54: Base54Type = *b"abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ34689";
const WEEK_TICK: i64 = 11631; // Number of seconds in one tick (a 52nd of a week)

#[derive(Debug)]
pub enum TagError {
    BufferOverflowError,
    DateConversionError,
}

impl fmt::Display for TagError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> FmtResult {
        match self {
            TagError::BufferOverflowError => write!(f, "Buffer overflow error"),
            TagError::DateConversionError => write!(f, "Date conversion error"),
        }
    }
}

#[derive(Default)]
pub struct TagBuf([u8; 3]);

impl TagBuf {
    /// Creates a new `TagBuf` initialized with zeros.
    pub fn new() -> Self {
        TagBuf([0; 3])
    }

    /// Access the internal buffer.
    pub fn as_mut(&mut self) -> &mut [u8; 3] {
        &mut self.0
    }

    /// Access the internal buffer (immutable).
    pub fn as_ref(&self) -> &[u8; 3] {
        &self.0
    }
}

/// Returns the base time (January 1, 2024, at 00:00:00 UTC).
fn base_time() -> Result<DateTime<Utc>, TagError> {
    match Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0) {
        LocalResult::Single(date_time) => Ok(date_time),
        LocalResult::Ambiguous(_, _) => Err(TagError::DateConversionError),
        LocalResult::None => Err(TagError::DateConversionError),
    }
}

pub trait BufAdd {
    /// Add character to the buffer and increment index
    fn add(&mut self, char: u8) -> Result<(), TagError>;
}

struct StrBuf<'a> {
    offset: usize,
    buf: &'a mut TagBuf,
}

impl BufAdd for StrBuf<'_> {
    fn add(&mut self, char: u8) -> Result<(), TagError> {
        if self.offset >= self.buf.as_ref().len() {
            return Err(TagError::BufferOverflowError);
        }
        self.buf.as_mut()[self.offset] = char;
        self.offset += 1;
        Ok(())
    }
}

/// Converts an integer to a string using the custom base54 alphabet.
pub fn base_x(buf: &mut impl BufAdd, mut num: i32, alphabet: &Base54Type) -> Result<(), TagError> {
    let base = alphabet.len() as i32;

    if num == 0 {
        buf.add(alphabet[0])?;
        return Ok(());
    }

    while num > 0 {
        let remainder = (num % base) as usize;
        num /= base;
        buf.add(alphabet[remainder])?;
    }
    Ok(())
}

/// Generates a unique tag from a `NaiveDateTime` object. Provide a struct that implements trait `BufAdd`.
pub fn to_datetag_buf(buf: &mut impl BufAdd, date: NaiveDateTime) -> Result<(), TagError> {
    let base_time = base_time()?;
    let year = date.year() - base_time.year();

    let month = date.month();
    let mut week = date.iso_week().week();

    if month == 1 && week > 51 {
        week = 0;
    }
    if month == 12 && week < 2 {
        let previous_week_date = date - Duration::weeks(1);
        week = previous_week_date.iso_week().week() + 1;
    }

    let day = date.weekday().num_days_from_monday() as i64; // Monday = 0
    let eve_est = date - Duration::days(day);

    let week_eve = NaiveDate::from_ymd_opt(eve_est.year(), eve_est.month(), eve_est.day())
        .ok_or(TagError::DateConversionError)?
        .and_hms_opt(0, 0, 0)
        .ok_or(TagError::DateConversionError)?;

    let delta = date - week_eve;
    let tick = (delta.num_seconds() / WEEK_TICK) as i32;
    base_x(buf, year, &BASE54)?;
    base_x(buf, week as i32, &BASE54)?;
    base_x(buf, tick, &BASE54)?;
    Ok(())
}

/// Generates a unique tag from a `NaiveDateTime` object. Provide a `TagBuf` as buffer.
pub fn to_datetag_array(buf: &mut TagBuf, date: NaiveDateTime) -> Result<(), TagError> {
    let mut str_buf = StrBuf { offset: 0, buf };
    to_datetag_buf(&mut str_buf, date)?;
    Ok(())
}
