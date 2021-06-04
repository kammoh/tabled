use std::ops::RangeBounds;

use papergrid::Grid;

use crate::{bounds_to_usize, TableOption};

/// Disable represent a disable setting for a [`table` macros](./macro.table.html)
///
/// ```rust,no_run
///   # use tabled::{Disable, table};
///   # let data: Vec<&'static str> = Vec::new();
///     let table = table!(&data, Disable::Row(..1));
/// ```
///
#[derive(Debug)]
pub enum Disable<R: RangeBounds<usize>> {
    /// Columns of the grid. Range is used to locate columns.
    Column(R),
    /// Rows of the grid. Range is used to locate rows.
    Row(R),
}

impl<R: RangeBounds<usize>> TableOption for Disable<R> {
    fn change(&self, grid: &mut Grid) {
        match self {
            Self::Column(range) => {
                let (x, y) =
                    bounds_to_usize(range.start_bound(), range.end_bound(), grid.count_columns());
                for (shifted, i) in (x..y).enumerate() {
                    grid.remove_column(i - shifted);
                }
            }
            Self::Row(range) => {
                let (x, y) =
                    bounds_to_usize(range.start_bound(), range.end_bound(), grid.count_rows());

                // It's kindof a bad design that we must controll shift.
                // It basically unveils an implementation...
                for (shifted, i) in (x..y).enumerate() {
                    grid.remove_row(i - shifted);
                }
            }
        }
    }
}
