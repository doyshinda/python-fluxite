use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use fluxite::{init_reporter, MetricsConfig, ObserverType};
use fluxite::metrics::{counter, gauge, timing};

#[pyfunction]
/// Initialize the metrics reporter
fn initialize_reporter(endpoint: String, prefix: Option<String>, observer_type: &'static str) -> PyResult<bool> {
    let m = match observer_type {
        "Graphite" | "graphite" => MetricsConfig {
            observer_type: ObserverType::Graphite,
            endpoint,
            prefix,
        },
        _ => MetricsConfig {
            observer_type: ObserverType::Influx,
            endpoint,
            prefix,
        }
    };

    match init_reporter(&m) {
        Ok(_r) => Ok(true),
        Err(e) => {
            println!("error initializing reporter: {:?}", e);
            Ok(false)
        },
    }
}

#[pyfunction]
/// Increment a `counter` by some `value`
fn count(name: String, value: usize) -> PyResult<bool> {
    counter!(name, value as u64);
    Ok(true)
}

#[pyfunction]
/// Update a `gauge` by some `value`
fn gauge(name: String, value: usize) -> PyResult<bool> {
    gauge!(name, value as i64);
    Ok(true)
}

#[pyfunction]
/// Update a `timer` by some `value`
fn timer(name: String, value: usize) -> PyResult<bool> {
    timing!(name, value as u64);
    Ok(true)
}

/// This module is a python module implemented in Rust.
#[pymodule]
fn python_fluxite(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(initialize_reporter))?;
    m.add_wrapped(wrap_pyfunction!(count))?;
    m.add_wrapped(wrap_pyfunction!(gauge))?;
    m.add_wrapped(wrap_pyfunction!(timer))?;
    Ok(())
}
