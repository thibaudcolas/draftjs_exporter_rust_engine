use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::wrap_pyfunction;

use htmlescape::{encode_attribute, encode_minimal};

#[pyfunction]
/// Formats the sum of two numbers as string.
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
  Ok((a + b).to_string())
}

#[pyfunction]
fn render_attr(key: &str, value: &str) -> PyResult<String> {
  Ok(format!(" {}=\"{}\"", key, encode_attribute(value)))
}

#[pyfunction]
fn render_attrs(attrs: &PyDict) -> PyResult<String> {
  let mut out: String = "".to_owned();

  for (key, value) in attrs.iter() {
    out.push_str(&format!(" {}=\"{}\"", key, value));
  }
  Ok(out)
}

#[pyfunction]
fn escape(html: &str) -> PyResult<String> {
  Ok(encode_minimal(html))
}

#[pymodule]
/// A Python module implemented in Rust.
fn string_engine(py: Python, m: &PyModule) -> PyResult<()> {
  m.add_wrapped(wrap_pyfunction!(sum_as_string))?;
  m.add_wrapped(wrap_pyfunction!(render_attr))?;
  m.add_wrapped(wrap_pyfunction!(render_attrs))?;
  m.add_wrapped(wrap_pyfunction!(escape))?;

  Ok(())
}
