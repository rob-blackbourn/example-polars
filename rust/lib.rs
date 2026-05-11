use pyo3::prelude::*;

mod pig_latin;

#[pymodule]
mod _lib {
    use pyo3::prelude::*;
    use std::env;

    /// Formats the sum of two numbers as string.
    #[pyfunction]
    fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
        Ok((a + b).to_string())
    }

    #[pyfunction]
    fn demo(py: Python) -> PyResult<()> {
        let argv = env::args().collect::<Vec<_>>();
        println!("argv: {:?}", argv);

        let numbers: Vec<i32> = argv[2..].iter().map(|s| s.parse().unwrap()).collect();

        let python_sum = PyModule::import(py, "builtins")?.getattr("sum")?;
        let total: i32 = python_sum.call1((numbers,))?.extract()?;
        println!("sum({}) = {:?}", argv[2..].join(", "), total);

        Ok(())
    }
}