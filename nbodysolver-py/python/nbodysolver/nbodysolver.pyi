import numpy as np
import numpy.typing as npt

type NDArrayFloat = npt.NDArray[np.float64]

def solve_lorenz(coords: tuple[float, float, float], max_time: float, dt: float, 
                 tau: float=10.0, rho: float=28.0, beta: float=8.0/3.0) -> \
                tuple[NDArrayFloat, NDArrayFloat, 
                      NDArrayFloat, NDArrayFloat]: ...
                
def simulate_nbody_and_process(r_list: NDArrayFloat, 
                               v_list: NDArrayFloat, 
                               m_list: NDArrayFloat, 
                               dt: float, max_time: float, g: float=6.6743e-11) -> \
    tuple[NDArrayFloat, NDArrayFloat, NDArrayFloat, 
          NDArrayFloat, NDArrayFloat, NDArrayFloat]: ...
    
def total_energy_nbody(r_list: NDArrayFloat, 
                       v_list: NDArrayFloat, 
                       m_list: NDArrayFloat, g: float=6.6743e-11) -> \
    tuple[float, float, float]: ...
    
def all_planet_acc_nbody(r_list: NDArrayFloat, 
                         m_list: NDArrayFloat, g: float=6.6743e-11) -> \
    NDArrayFloat: ...