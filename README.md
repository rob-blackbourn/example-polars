# example-polars

An example of writing a polars plugin using [setuptools-rust](https://github.com/PyO3/setuptools-rust).

Assuming Rust is installed, the project can be setup in the usual manner.

```bash
$ python -m venv .venv
$ source .venv/bin/activate
(.venv) $ pip install --editable '.[dev]'
```

The `.vscode` folder has a launch target "Python/Rust Debugger: Current File", where breakpoints can
be set in either Python or Rust source files.
