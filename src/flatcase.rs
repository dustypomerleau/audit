/// A flattened version of the [`Case`](crate::case::Case) struct for use in database queries and
/// the initial ingestion of CSV data.
// All fields are optional, to allow using the same struct for any DB query on Case.
// todo: revisit this after you've created your DB model, to see if any of the types need to be
// altered.
pub struct FlatCase {
    pub surgeon_email: Option<String>,
    pub surgeon_first_name: Option<String>,
    pub surgeon_last_name: Option<String>,
    pub surgeon_site: Option<String>,
    pub urn: Option<String>,
    pub side: Option<Side>,
    pub target_formula: Option<Formula>,
    pub target_se: Option<f32>,
    pub target_cyl_power: Option<f32>,
    pub target_cyl_axis: Option<i32>,
    pub date: Option<Date>,
    pub site: Option<String>,
    pub incision_meridian: Option<i32>,
    pub incision_sia: Option<f32>,
    pub iol: Option<String>,
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
