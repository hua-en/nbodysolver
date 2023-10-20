from nbodysolver.nbodysolver import *
from nbodysolver.plotting import *
from nbodysolver.example_systems import *
from nbodysolver.nbodywrapper import *


# This code basically adds all the docstrings and functions 
# from the rust library nbodysolver
__doc__ = nbodysolver.__doc__ # type: ignore  # noqa: F405
if hasattr(nbodysolver, "__all__"): # type: ignore # noqa: F405
    __all__ = nbodysolver.__all__ # type: ignore # noqa: F405