use numpy::{IntoPyArray, PyArray1};
use pyo3::prelude::{pymodule, PyModule, PyResult, Python};
use std::f64;

mod rust_fn {
    use ndarray::Array1;
    pub fn sine_wave(num_samples: usize, freq: f64) -> Array1<f64> {
        let fsample_rate: f64 = 48000.0;
        let result_array: Array1<f64> = (0..num_samples)
            .map(|i| ((2.0 * std::f64::consts::PI * freq * (i as f64) / fsample_rate).sin()))
            .collect::<ndarray::Array1<f64>>()
            .try_into()
            .unwrap();
        return result_array;
    }
}

#[pymodule]
fn rust_numpy_ext(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    #[pyfn(m)]
    fn sine_wave_wrapper<'py>(py: Python<'py>, size: usize, freq: f64) -> &'py PyArray1<f64> {
        rust_fn::sine_wave(size, freq).into_pyarray(py)
    }
    Ok(())
}
