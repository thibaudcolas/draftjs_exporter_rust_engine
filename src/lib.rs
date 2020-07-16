use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::wrap_pyfunction;

use htmlescape::{encode_attribute, encode_minimal};

// http://w3c.github.io/html/single-page.html#void-elements
// https://github.com/html5lib/html5lib-python/blob/0cae52b2073e3f2220db93a7650901f2200f2a13/html5lib/constants.py#L560
const VOID_ELEMENTS: [&str; 14] = [
  "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param", "source",
  "track", "wbr",
];

struct Elt {
  type_: String,
  attr: String,
  children: String,
  markup: String,
}

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
fn render(
  type_: &str,
  attrs: Option<&PyDict>,
  children: Option<&str>,
  markup: Option<&str>,
) -> PyResult<String> {
  // let attr = render_attrs(attrs);
  let mut attr: String = "".to_owned();

  if let Some(attributes) = attrs {
    for (key, value) in attributes.iter() {
      attr.push_str(&format!(" {}=\"{}\"", key, value));
    }
  }

  if type_ == "fragment" {
    return Ok(match children {
      None => "".to_string(),
      Some(c) => c.to_string(),
    });
    // return Ok();
  }

  if VOID_ELEMENTS.contains(&type_) {
    return Ok(format!("<{}{}/>", type_, attr));
  }

  if type_ == "escaped_html" {
    return Ok(match markup {
      None => "".to_string(),
      Some(m) => m.to_string(),
    });
  }

  let child = match children {
    None => "",
    Some(c) => c,
  };

  Ok(format!("<{}{}>{}</{}>", type_, attr, child, type_))
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
  m.add_wrapped(wrap_pyfunction!(render))?;
  m.add_wrapped(wrap_pyfunction!(escape))?;

  Ok(())
}
