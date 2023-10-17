from nbodysolver.nbodysolver import *
from nbodysolver.plotting import *
from nbodysolver.example_systems import *


print(nbodysolver.__module__)
__doc__ = nbodysolver.__doc__ # type: ignore
if hasattr(nbodysolver, "__all__"): # type: ignore
    __all__ = nbodysolver.__all__ # type: ignore