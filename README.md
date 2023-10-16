# nbodysolver

An n-body solver written in Rust with both Rust and Python bindings.  

This repo is split into two sections: `nbodysolver`, containing the pure Rust code and Rust crate, and `nbodysolver-py`, containing the python package with bindings to the rust crate.

## Initialising the python project

Run `pdm install` within the `nbodysolver-py` directory. This will automatically initialise a virtual environment, install all dependencies as well as install the `nbodysolver` python package as an editable installation.
