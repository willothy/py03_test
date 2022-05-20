use pyo3::prelude::*;
use pyo3::types::IntoPyDict;

fn main() -> PyResult<()> {
    pyo3::prepare_freethreaded_python();
    let gil = Python::acquire_gil();
    let py = gil.python();
    let sys = py.import("sys")?;
    let version = sys.getattr("version")?;
    let locals = [("os", py.import("os")?)].into_py_dict(py);
    let user = py.eval("os.getenv('USER') or os.getenv('USERNAME')", None, Some(&locals)).unwrap();

    println!("Hello {}, I'm python {}", user, version);
    Ok(())
}
