use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use ::dissimilar::Chunk;
use pyo3::class::basic::CompareOp;
use pyo3::prelude::*;

#[pyclass(subclass, name = "Chunk", module = "dissimilar")]
struct PyChunk;

#[pyclass(extends = PyChunk, name = "Equal", module = "dissimilar")]
struct PyEqual(String);

#[pymethods]
impl PyEqual {
    #[new]
    fn new(s: String) -> (Self, PyChunk) {
        (Self(s), PyChunk)
    }

    fn __str__(&self) -> &str {
        &self.0
    }

    fn __repr__(&self) -> String {
        format!("Equal(\"{}\")", self.0)
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp, py: Python<'_>) -> PyObject {
        match op {
            CompareOp::Eq => (self.0 == other.0).into_py(py),
            CompareOp::Ne => (self.0 != other.0).into_py(py),
            _ => py.NotImplemented(),
        }
    }

    fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish()
    }
}

#[pyclass(extends = PyChunk, name = "Delete", module = "dissimilar")]
struct PyDelete(String);

#[pymethods]
impl PyDelete {
    #[new]
    fn new(s: String) -> (Self, PyChunk) {
        (Self(s), PyChunk)
    }

    fn __str__(&self) -> &str {
        &self.0
    }

    fn __repr__(&self) -> String {
        format!("Delete(\"{}\")", self.0)
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp, py: Python<'_>) -> PyObject {
        match op {
            CompareOp::Eq => (self.0 == other.0).into_py(py),
            CompareOp::Ne => (self.0 != other.0).into_py(py),
            _ => py.NotImplemented(),
        }
    }

    fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish()
    }
}

#[pyclass(extends = PyChunk, name = "Insert", module = "dissimilar")]
struct PyInsert(String);

#[pymethods]
impl PyInsert {
    #[new]
    fn new(s: String) -> (Self, PyChunk) {
        (Self(s), PyChunk)
    }

    fn __str__(&self) -> &str {
        &self.0
    }

    fn __repr__(&self) -> String {
        format!("Insert(\"{}\")", self.0)
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp, py: Python<'_>) -> PyObject {
        match op {
            CompareOp::Eq => (self.0 == other.0).into_py(py),
            CompareOp::Ne => (self.0 != other.0).into_py(py),
            _ => py.NotImplemented(),
        }
    }

    fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish()
    }
}

/// Diff two strings with semantic cleanup
#[pyfunction]
fn diff(py: Python<'_>, a: &str, b: &str) -> PyResult<Vec<PyObject>> {
    let chunks = py.allow_threads(move || ::dissimilar::diff(a, b));
    let mut py_chunks = Vec::with_capacity(chunks.len());
    for chunk in chunks {
        let py_chunk = match chunk {
            Chunk::Equal(s) => {
                let initializer =
                    PyClassInitializer::from(PyChunk).add_subclass(PyEqual(s.to_string()));
                Py::new(py, initializer)?.into_py(py)
            }
            Chunk::Delete(s) => {
                let initializer =
                    PyClassInitializer::from(PyChunk).add_subclass(PyDelete(s.to_string()));
                Py::new(py, initializer)?.into_py(py)
            }
            Chunk::Insert(s) => {
                let initializer =
                    PyClassInitializer::from(PyChunk).add_subclass(PyInsert(s.to_string()));
                Py::new(py, initializer)?.into_py(py)
            }
        };
        py_chunks.push(py_chunk);
    }
    Ok(py_chunks)
}

/// A Python module implemented in Rust.
#[pymodule]
fn dissimilar(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyChunk>()?;
    m.add_class::<PyEqual>()?;
    m.add_class::<PyDelete>()?;
    m.add_class::<PyInsert>()?;
    m.add_function(wrap_pyfunction!(diff, m)?)?;
    Ok(())
}
