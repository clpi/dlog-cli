use std::{collections::HashMap, fmt::Debug};

use clap::Clap;
use super::{Model, Item};
use serde::{Serialize, Deserialize};
use chrono::{Date, DateTime, Duration, Local};

#[derive(Clap, Deserialize, Serialize, Debug)]
pub struct FieldOp {
    pub name: Option<String>,
    pub kind: Option<String>,
    pub item: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clap)]
pub struct Field {
    pub name: String,
    pub created_at: DateTime<Local>
}
impl Model for Field {

}
pub struct FieldType {

}

pub trait FieldValue
/* where
    for<'de> Self: 'static + Deserialize<'de> */
{

}
impl FieldValue for bool {}
/* impl FieldValue for i32 {}
impl FieldValue for f32 {} */
// impl FieldValue for String {}
/* #[derive(Deserialize, Serialize, Debug)]
pub enum FieldValue {
    Boolean(bool),
    Integer(i32),
    Decimal(f32),
    IntRange(i32, i32),
    DecimalRange(f32, f32),
    /// Single index of fieldtype vector of choices
    Choice(usize),
    /// Multiple indices of fieldtype vector of choices
    MultipleChoice(Vec<usize>),
    Text(String),
    MultiText(Vec<String>),
    // Date(Date<Local>),
    DistanceMeters(f32),
    // Duration(Duration),
    DurationSecs(f32),
    DateTime(DateTime<Local>),
    // DateRange(Date<Local>, Date<Local>),
    DateTimeRange(DateTime<Local>, DateTime<Local>)
} */
