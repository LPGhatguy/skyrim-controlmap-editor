//! Pretty column printer akin to how controlmap.txt is normally formatted.
//!
//! Uses 4-space tabs to align columns.

use std::fmt::{self, Display, Write};

const TAB_WIDTH: usize = 4;

pub struct ColumnPrinter {
    rows: Vec<Vec<String>>,
}

impl ColumnPrinter {
    pub fn new() -> Self {
        Self { rows: Vec::new() }
    }

    pub fn row(&mut self) {
        self.rows.push(Vec::new());
    }

    pub fn add(&mut self, value: impl Display) {
        self.rows.last_mut().unwrap().push(value.to_string());
    }

    pub fn finish(self, mut output: impl Write) -> fmt::Result {
        let num_cols = self.rows.iter().map(|row| row.len()).max().unwrap_or(0);

        let widths: Vec<_> = (0..num_cols)
            .map(|i| {
                let widest = self
                    .rows
                    .iter()
                    .filter_map(|row| Some(row.get(i)?.len()))
                    .max()
                    .unwrap_or(0);

                // Round each width up to the nearest tab stop
                widest + 4 - (widest % TAB_WIDTH)
            })
            .collect();

        for (row_num, row) in self.rows.iter().enumerate() {
            for (i, value) in row.iter().enumerate() {
                write!(&mut output, "{}", value)?;

                if i < row.len() - 1 {
                    let diff = widths[i] - value.len();

                    // Apply as many tabs as is necessary to reach the next tab
                    // stop, but at least one.
                    let num_tabs = (diff / TAB_WIDTH + (diff % TAB_WIDTH != 0) as usize).max(1);

                    write!(&mut output, "{}", "\t".repeat(num_tabs))?;
                }
            }

            if row_num < self.rows.len() - 1 {
                writeln!(&mut output)?;
            }
        }

        Ok(())
    }
}
