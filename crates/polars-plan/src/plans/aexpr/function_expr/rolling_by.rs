use polars_time::chunkedarray::*;

use super::*;

#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "ir_serde", derive(serde::Serialize, serde::Deserialize))]
pub enum IRRollingFunctionBy {
    MinBy(RollingOptionsDynamicWindow),
    MaxBy(RollingOptionsDynamicWindow),
    MeanBy(RollingOptionsDynamicWindow),
    SumBy(RollingOptionsDynamicWindow),
    QuantileBy(RollingOptionsDynamicWindow),
    VarBy(RollingOptionsDynamicWindow),
    StdBy(RollingOptionsDynamicWindow),
}

impl Display for IRRollingFunctionBy {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use IRRollingFunctionBy::*;

        let name = match self {
            MinBy(_) => "rolling_min_by",
            MaxBy(_) => "rolling_max_by",
            MeanBy(_) => "rolling_mean_by",
            SumBy(_) => "rolling_sum_by",
            QuantileBy(_) => "rolling_quantile_by",
            VarBy(_) => "rolling_var_by",
            StdBy(_) => "rolling_std_by",
        };

        write!(f, "{name}")
    }
}

impl Hash for IRRollingFunctionBy {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
    }
}

pub(super) fn rolling_min_by(
    s: &[Column],
    options: RollingOptionsDynamicWindow,
) -> PolarsResult<Column> {
    // @scalar-opt
    s[0].as_materialized_series()
        .rolling_min_by(s[1].as_materialized_series(), options)
        .map(Column::from)
}

pub(super) fn rolling_max_by(
    s: &[Column],
    options: RollingOptionsDynamicWindow,
) -> PolarsResult<Column> {
    // @scalar-opt
    s[0].as_materialized_series()
        .rolling_max_by(s[1].as_materialized_series(), options)
        .map(Column::from)
}

pub(super) fn rolling_mean_by(
    s: &[Column],
    options: RollingOptionsDynamicWindow,
) -> PolarsResult<Column> {
    // @scalar-opt
    s[0].as_materialized_series()
        .rolling_mean_by(s[1].as_materialized_series(), options)
        .map(Column::from)
}

pub(super) fn rolling_sum_by(
    s: &[Column],
    options: RollingOptionsDynamicWindow,
) -> PolarsResult<Column> {
    // @scalar-opt
    s[0].as_materialized_series()
        .rolling_sum_by(s[1].as_materialized_series(), options)
        .map(Column::from)
}

pub(super) fn rolling_quantile_by(
    s: &[Column],
    options: RollingOptionsDynamicWindow,
) -> PolarsResult<Column> {
    // @scalar-opt
    s[0].as_materialized_series()
        .rolling_quantile_by(s[1].as_materialized_series(), options)
        .map(Column::from)
}

pub(super) fn rolling_var_by(
    s: &[Column],
    options: RollingOptionsDynamicWindow,
) -> PolarsResult<Column> {
    // @scalar-opt
    s[0].as_materialized_series()
        .rolling_var_by(s[1].as_materialized_series(), options)
        .map(Column::from)
}

pub(super) fn rolling_std_by(
    s: &[Column],
    options: RollingOptionsDynamicWindow,
) -> PolarsResult<Column> {
    // @scalar-opt
    s[0].as_materialized_series()
        .rolling_std_by(s[1].as_materialized_series(), options)
        .map(Column::from)
}
