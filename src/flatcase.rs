use crate::case::{Adverse, Side};
use time::Date;

/// A flattened version of the [`Case`](crate::case::Case) struct for use in database queries and
/// the initial ingestion of CSV data.
// todo: this likely needs to be flattened _completely_, which means bringing target_formula in
// line with the DB by matching on a String value, rather than expecting an enum (Case can keep an
// enum)
#[derive(Debug, PartialEq)]
pub struct FlatCase {
    pub surgeon_email: Option<String>,
    pub surgeon_first_name: Option<String>,
    pub surgeon_last_name: Option<String>,
    pub surgeon_site: Option<String>,
    pub urn: Option<String>,
    pub side: Option<Side>,
    pub target_formula: Option<String>,
    pub target_se: Option<f32>,
    pub target_cyl_power: Option<f32>,
    pub target_cyl_axis: Option<i32>,
    pub date: Option<Date>,
    pub site: Option<String>,
    pub sia_power: Option<f32>,
    pub sia_meridian: Option<i32>,
    pub iol_se: Option<f32>,
    pub iol_cyl_power: Option<f32>,
    pub iol_cyl_axis: Option<i32>,
    pub adverse: Option<Adverse>,

    pub va_best_before_num: Option<f32>,
    pub va_best_before_den: Option<f32>,
    pub va_best_after_num: Option<f32>,
    pub va_best_after_den: Option<f32>,

    pub va_raw_before_num: Option<f32>,
    pub va_raw_before_den: Option<f32>,
    pub va_raw_after_num: Option<f32>,
    pub va_raw_after_den: Option<f32>,

    pub va_best_near_before_num: Option<f32>,
    pub va_best_near_before_den: Option<f32>,
    pub va_best_near_after_num: Option<f32>,
    pub va_best_near_after_den: Option<f32>,

    pub va_raw_near_before_num: Option<f32>,
    pub va_raw_near_before_den: Option<f32>,
    pub va_raw_near_after_num: Option<f32>,
    pub va_raw_near_after_den: Option<f32>,

    pub ref_before_sph: Option<f32>,
    pub ref_before_cyl_power: Option<f32>,
    pub ref_before_cyl_axis: Option<i32>,

    pub ref_after_sph: Option<f32>,
    pub ref_after_cyl_power: Option<f32>,
    pub ref_after_cyl_axis: Option<i32>,
}
