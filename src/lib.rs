// use brotli::enc::BrotliEncoderParams;
// use base_encode::{encode, decode, to_string};
// use base64;

mod compression;

use brotli::enc::backward_references::BrotliEncoderMode;
use brotli::enc::BrotliEncoderParams;
use std::io;
use std::io::Cursor;

use pyo3::create_exception;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

// Define exceptions
create_exception!(uzip, EncodingError, PyValueError);
create_exception!(uzip, DecodeError, PyValueError);

/// Accepts python `bytes` and returns a compressed version of it.
#[pyfunction(level = "9")]
fn compress(data: &[u8], level: usize) -> PyResult<PyObject> {
    let res = match compression::compress(data, level) {
        Ok(x) => x,
        Err(e) => return Err(EncodingError::new_err(format!("Error {:?}", e))),
    };
    let result = Python::with_gil(|py| PyBytes::new(py, &res).into());
    Ok(result)
}

/// Accepts python `bytes` and decompress it.
#[pyfunction]
fn decompress(data: &[u8]) -> PyResult<PyObject> {
    let res = match compression::decompress(data) {
        Ok(x) => x,
        Err(e) => return Err(DecodeError::new_err(e)),
    };
    let result = Python::with_gil(|py| PyBytes::new(py, &res).into());
    Ok(result)
}

#[pyfunction]
fn z_compress(data: &[u8]) -> PyResult<PyObject> {
    let mut compressed = Vec::new();

    match zstd::stream::copy_encode(data, &mut compressed, 0) {
        Ok(_) => {}
        Err(e) => return Err(EncodingError::new_err(format!("Error {:?}", e))),
    }

    let result = Python::with_gil(|py| PyBytes::new(py, &compressed).into());

    Ok(result)
}

#[pyfunction]
fn b_compress(data: &[u8]) -> PyResult<PyObject> {
    let mut buffer = Cursor::new(data);
    let mut compressed = Vec::new();

    let mut params = BrotliEncoderParams::default();
    params.quality = 7;
    params.large_window = true;
    params.mode = BrotliEncoderMode::BROTLI_MODE_GENERIC;

    match brotli::BrotliCompress(&mut buffer, &mut compressed, &params) {
        Ok(_) => {}
        Err(e) => return Err(EncodingError::new_err(format!("Error {:?}", e))),
    }

    let result = Python::with_gil(|py| PyBytes::new(py, &compressed).into());

    Ok(result)
}

#[pyfunction]
fn z_dict_compress(data: &[u8]) -> PyResult<PyObject> {
    // Build the dict
    let sample_size = data.len() / 7;
    let mut sizes = [sample_size; 7];
    let sum_sizes = sizes.iter().sum::<usize>();
    sizes[6] += data.len() - sum_sizes;
    let dict = zstd::dict::from_continuous(data, &sizes, data.len()).unwrap();

    let mut compressed = Vec::new();
    let mut encoder = zstd::Encoder::with_dictionary(&mut compressed, 21, &dict).unwrap();
    encoder.multithread(10).unwrap();

    let mut cur = Cursor::new(data);

    match io::copy(&mut cur, &mut encoder) {
        Ok(_) => {}
        Err(e) => return Err(EncodingError::new_err(format!("Error {:?}", e))),
    }

    encoder.finish().unwrap();

    let result = Python::with_gil(|py| PyBytes::new(py, &compressed).into());

    Ok(result)
}

#[pyfunction]
fn z_encode(data: &[u8]) -> PyResult<String> {
    // Build the dict
    let sample_size = data.len() / 7;
    let mut sizes = [sample_size; 7];
    let sum_sizes = sizes.iter().sum::<usize>();
    sizes[6] += data.len() - sum_sizes;
    let dict = match zstd::dict::from_continuous(data, &sizes, data.len()) {
        Ok(x) => x,
        Err(e) => return Err(EncodingError::new_err(format!("Error {:?}", e))),
    };

    let mut compressed = Vec::new();
    let mut encoder = zstd::Encoder::with_dictionary(Vec::new(), 21, &dict).unwrap();
    encoder.multithread(10).unwrap();

    match zstd::stream::copy_encode(data, &mut compressed, 21) {
        Ok(_) => {}
        Err(e) => return Err(EncodingError::new_err(format!("Error {:?}", e))),
    }

    encoder.finish().unwrap();

    let encoded = base2048::encode(&compressed);

    Ok(encoded)
}

#[pyfunction]
fn b2048encode(s: &[u8]) -> PyResult<String> {
    Ok(base2048::encode(s))
}

#[pyfunction]
fn b2048decode(s: &str) -> PyResult<PyObject> {
    let result = match base2048::decode(s) {
        Some(x) => x,
        None => {
            return Err(DecodeError::new_err(format!(
                "Could not decode bytes as valid base-2048"
            )))
        }
    };
    let bytes = Python::with_gil(|py| PyBytes::new(py, &result).into());
    Ok(bytes)
}

/// A Python module implemented in Rust.
#[pymodule]
fn uzip(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(b2048encode, m)?)?;
    m.add_function(wrap_pyfunction!(b2048decode, m)?)?;

    m.add_function(wrap_pyfunction!(compress, m)?)?;
    m.add_function(wrap_pyfunction!(decompress, m)?)?;

    m.add_function(wrap_pyfunction!(z_compress, m)?)?;
    m.add_function(wrap_pyfunction!(z_dict_compress, m)?)?;
    m.add_function(wrap_pyfunction!(b_compress, m)?)?;
    m.add_function(wrap_pyfunction!(z_encode, m)?)?;

    // Add exceptions
    m.add("EncodingError", _py.get_type::<EncodingError>())?;
    Ok(())
}
