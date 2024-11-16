use pyo3::{prelude::*, wrap_pyfunction};

use tegra_swizzle::{self, BlockHeight};

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

fn block_height_to_u32(bh: BlockHeight) -> u32 {
    match bh {
        BlockHeight::One => 1,
        BlockHeight::Two => 2,
        BlockHeight::Four => 4,
        BlockHeight::Eight => 8,
        BlockHeight::Sixteen => 16,
        BlockHeight::ThirtyTwo => 32
    }
}

#[pyfunction]
fn block_height_mip0(height: u32) -> PyResult<u32> {
    let bh = tegra_swizzle::block_height_mip0(height);
    Ok(block_height_to_u32(bh))
}

/// A Python module implemented in Rust.
#[pymodule]
fn rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(block_height_mip0, m)?)?;
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
