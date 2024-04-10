use pyo3::prelude::*;

#[pyfunction]
pub fn print(msg: PyObject) {
    println!("[JABU]: {msg}");
}

#[pyfunction]
pub fn get_version() -> &'static str {
    crate::VERSION 
}
