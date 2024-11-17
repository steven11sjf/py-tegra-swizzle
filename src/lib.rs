use std::{char::from_u32, num::NonZeroU32};

use pyo3::{ffi::PyObject, prelude::*, types::{PyByteArray, PyBytes}, wrap_pyfunction};

use tegra_swizzle::{self, surface::BlockDim, BlockHeight};

fn to_nonzero(num: u32) -> NonZeroU32 {
    Option::expect(NonZeroU32::new(num), "Number must be nonzero!")
}

#[pyclass]
#[derive(Clone)]
pub struct PyBlockDim {
    inner: BlockDim
}

#[pymethods]
impl PyBlockDim {
    #[new]
    pub fn new(width: u32, height: u32, depth: u32) -> Self {
        PyBlockDim {
            inner:  BlockDim {
                width: to_nonzero(width),
                height: to_nonzero(height),
                depth: to_nonzero(depth)
            }
        }
    }

    pub fn width(&self) -> u32 {
        self.inner.width.get()
    }

    pub fn height(&self) -> u32 {
        self.inner.height.get()
    }

    pub fn depth(&self) -> u32 {
        self.inner.depth.get()
    }
}


#[pyfunction]
fn block_height_mip0(height: u32) -> PyResult<u32> {
    Ok(tegra_swizzle::block_height_mip0(height) as u32)
}

#[pyfunction]
fn mip_block_height(mip_height: u32, block_height_mip0: u32) -> PyResult<u32> {
    let bh = BlockHeight::new(block_height_mip0).unwrap();
    Ok(tegra_swizzle::mip_block_height(mip_height, bh) as u32)
}

#[pyfunction]
fn swizzled_surface_size(
    width: u32, height: u32, depth: u32, block_dim: PyBlockDim, block_height_mip0: u32, 
    bytes_per_pixel: u32, mipmap_count: u32, layer_count: u32
) -> PyResult<u32> {
    Ok(tegra_swizzle::surface::swizzled_surface_size(
        width, height, depth, block_dim.inner, Some(BlockHeight::new(block_height_mip0).unwrap()), 
        bytes_per_pixel, mipmap_count, layer_count) as u32
    )
}

// TODO: can't figure out how to return a `bytes` LMAO
// fn deswizzle_block_linear<'a>(py: Python<'a>, width: u32, height: u32, depth: u32, source: &PyBytes, block_height: u32, bytes_per_pixel: u32) -> PyResult<&'a PyAny> {
//     let src = source.as_bytes();
//     let bk_height = BlockHeight::new(block_height).unwrap();
//     let d = tegra_swizzle::swizzle::deswizzle_block_linear(width, height, depth, src, bk_height, bytes_per_pixel).unwrap();
//     let des: &[u8] = &d;
//     Ok(PyBytes::new(py, des).into())
// }

/// A Python module implemented in Rust.
#[pymodule]
fn rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(block_height_mip0, m)?)?;
    m.add_function(wrap_pyfunction!(mip_block_height, m)?)?;
    m.add_function(wrap_pyfunction!(swizzled_surface_size, m)?)?;
    //m.add_function(wrap_pyfunction!(deswizzle_block_linear, m)?)?;
    m.add_class::<PyBlockDim>()?;
    Ok(())
}
