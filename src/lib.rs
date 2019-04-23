#![feature(specialization)]
#![feature(type_alias_enum_variants)]

#[macro_use]
extern crate pyo3;

use pyo3::types::{PyDict, PyTuple};

use pyo3::exceptions::TypeError as PyTypeError;
use pyo3::exceptions::ValueError as PyValueError;
extern crate serdesqueezebox;

use pyo3::prelude::*;

use std::fs;
use std::path::PathBuf;

use pyo3::types::PyList;
use pyo3::Python;
use pyo3::{PyTryFrom, ToPyObject};

/// Represents a file that can be searched
#[pyclass]
struct WordCounter {
    path: PathBuf,
}

#[pyclass]
#[derive(Debug)]
pub struct Player {
    defalts: serdesqueezebox::Player,
}

#[pymethods]
impl Player {
    #[getter]
    fn get_name(&self) -> PyResult<String> {
        Ok(self.defalts.name.clone())
    }

    #[getter]
    fn get_uuid(&self) -> PyResult<String> {
        Ok(self.defalts.uuid.clone())
    }

    #[getter]
    fn get_id(&self) -> PyResult<String> {
        Ok(self.defalts.id.clone())
    }

    #[getter]
    fn get_ip(&self) -> PyResult<String> {
        Ok(self.defalts.ip.clone())
    }

    #[getter]
    fn get_model(&self) -> PyResult<String> {
        Ok(self.defalts.model.clone())
    }

    #[getter]
    fn get_firmware_version(&self) -> PyResult<String> {
        Ok(self.defalts.firmware_version.clone())
    }

    #[getter]
    fn get_signal_strength(&self) -> PyResult<u8> {
        Ok(self.defalts.signal_strength.clone())
    }

    #[getter]
    fn get_play_state(&self) -> PyResult<String> {
        Ok(self.defalts.play_state.clone())
    }
}

impl From<serdesqueezebox::Player> for Player {
    fn from(item: serdesqueezebox::Player) -> Self {
        Player { defalts: item }
    }
}

fn matches(word: &str, needle: &str) -> bool {
    let mut needle = needle.chars();
    for ch in word.chars().skip_while(|ch| !ch.is_alphabetic()) {
        match needle.next() {
            None => {
                return !ch.is_alphabetic();
            }
            Some(expect) => {
                if ch.to_lowercase().next() != Some(expect) {
                    return false;
                }
            }
        }
    }
    return needle.next().is_none();
}

/// Count the occurences of needle in line, case insensitive
#[pyfunction]
fn des_ser(json: &str) -> PyResult<String> {
    let ff: Result<Vec<serdesqueezebox::Player>, serde_json::Error> = serde_json::from_str(json);
    let out = match ff {
        Ok(p) => p,
        Err(p) => return Err(PyErr::new::<PyTypeError, _>(format!("{}", p))),
    };

    let serialized = serde_json::to_string(&out).unwrap();
    Ok(serialized)
}

#[pyclass]
struct MyClass {
    pub num: i32,
}

#[pymethods]
impl MyClass {
    #[getter]
    fn get_num(&self) -> PyResult<i32> {
        Ok(self.num)
    }
}

#[pyfunction]
fn return_myclass() -> Py<MyClass> {
    let gil = Python::acquire_gil();
    let py = gil.python();
    Py::new(py, MyClass { num: 1 }).unwrap()
}

#[pyfunction]
fn return_myclasslist() -> PyResult<Vec<MyClass>> {
    let v = vec![MyClass { num: 1 }];
    return Ok(v);
}

#[pyfunction]
fn json2PlayerList(json: &str) -> PyResult<Vec<Player>> {
    let request: Result<Vec<serdesqueezebox::Player>, serde_json::error::Error> =
        serde_json::from_str(json);
    let mut output = vec![];
    let p = match request {
        Ok(p) => p,
        Err(p) => {
            return Err(PyErr::new::<PyTypeError, _>(format!("{}", p)));
        }
    };
    for item in p {
        output.push(Player { defalts: item })
    }
    return Ok(output);
}

#[pymodule]
fn pyserdesqueezebox(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add_wrapped(wrap_pyfunction!(des_ser))?;
    m.add_wrapped(wrap_pyfunction!(return_myclass))?;
    m.add_wrapped(wrap_pyfunction!(return_myclasslist))?;
    m.add_wrapped(wrap_pyfunction!(json2PlayerList))?;
    m.add_class::<WordCounter>()?;
    m.add_class::<Player>()?;
    Ok(())
}
