# example-polars

An example of writing a polars plugin using [setuptools-rust](https://github.com/PyO3/setuptools-rust).

## Setup

Assuming Rust is installed, the project can be setup in the usual manner.

```bash
$ python -m venv .venv
$ source .venv/bin/activate
(.venv) $ pip install --editable '.[dev]'
```

Note this builds the rust library.

## Debugging

The `.vscode` folder has a launch target "Python/Rust Debugger: Current File", where breakpoints can
be set in either Python or Rust source files.

## Building a Release

A project can be built with the standard build module.

```bash
(.venv) $ pip install build
(.venv) $ python -m build
```

The packages in the `dist` folder could be then uploaded with `twine`.
