import numpy as np
import numpy.typing as npt

type ndarray_float = npt.NDArray[np.float64]

def solve_lorenz(coords: tuple[float, float, float], max_time: float, dt: float, 
                 tau: float=10.0, rho: float=28.0, beta: float=8.0/3.0) -> \
                tuple[ndarray_float, ndarray_float, 
                      ndarray_float, ndarray_float]: ...
                
def simulate_nbody_and_process(r_list: ndarray_float, 
                               v_list: ndarray_float, 
                               m_list: ndarray_float, 
                               dt: float, max_time: float, g: float=6.6743e-11) -> \
    tuple[ndarray_float, ndarray_float, ndarray_float, 
          ndarray_float, ndarray_float, ndarray_float]: ...
    
def total_energy_nbody(r_list: ndarray_float, 
                       v_list: ndarray_float, 
                       m_list: ndarray_float, g: float=6.6743e-11) -> \
    tuple[float, float, float]: ...
    
def all_planet_acc_nbody(r_list: ndarray_float, 
                         m_list: ndarray_float, g: float=6.6743e-11) -> \
    ndarray_float: ...