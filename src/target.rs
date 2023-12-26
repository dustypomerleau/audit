use crate::case::Axis;

/// A formula for calculating IOL power from biometry.
// Limited to common thick-lens formulas to start.
// Eventually we will add all the formulas commonly in use.
#[derive(Debug, PartialEq)]
pub enum Formula {
    Barrett,
    Kane,
}

#[derive(Debug, PartialEq)]
pub struct TargetCylPower(f32);

impl TargetCylPower {
    pub fn new(value: f32) -> Option<Self> {
        if (0.0..=6.0).contains(&value) {
            Some(Self(value))
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct TargetCyl {
    power: TargetCylPower,
    axis: Axis,
}

// note: This is the second impl of SomeCyl::new()
// If you do this a third time, perhaps abstract it into a generic.
impl TargetCyl {
    fn new(power: Option<f32>, axis: Option<i32>) -> Option<Self> {
        match (power, axis) {
            (Some(power), Some(axis)) => {
                if let (Some(power), Some(axis)) = (TargetCylPower::new(power), Axis::new(axis)) {
                    Some(Self { power, axis })
                } else {
                    None
                }
            }

            (_, _) => None,
        }
    }
}

/// The residual postop refraction predicted by your formula of choice.
// At the start, allow only one formula/target.
#[derive(Debug, PartialEq)]
pub struct Target {
    formula: Option<Formula>,
    se: f32,
    cyl: Option<TargetCyl>, // todo: confirm which plane the biometry is predicting, IOL or corneal
}

impl Target {
    pub fn new(
        formula: Option<Formula>,
        se: f32,
        cyl: Option<f32>,
        axis: Option<i32>,
    ) -> Option<Self> {
        let cyl = TargetCyl::new(cyl, axis);

        if (-6.0..=2.0).contains(&se) {
            Some(Self { formula, se, cyl })
        } else {
            None
        }
    }
}
