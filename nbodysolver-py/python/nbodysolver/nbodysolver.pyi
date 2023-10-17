import numpy as np
import numpy.typing as npt

# type NDArrayFloat = npt.NDArray[np.float64]

def solve_lorenz(coords: tuple[float, float, float], max_time: float, dt: float, 
                 tau: float=10.0, rho: float=28.0, beta: float=8.0/3.0) -> \
                tuple[npt.NDArray[np.float64], npt.NDArray[np.float64], 
                      npt.NDArray[np.float64], npt.NDArray[np.float64]]: ...
                
def simulate_nbody_and_process(r_list: npt.NDArray[np.float64], 
                               v_list: npt.NDArray[np.float64], 
                               m_list: npt.NDArray[np.float64], 
                               dt: float, max_time: float, g: float=6.6743e-11) -> \
    tuple[npt.NDArray[np.float64], npt.NDArray[np.float64], npt.NDArray[np.float64], 
          npt.NDArray[np.float64], npt.NDArray[np.float64], npt.NDArray[np.float64]]: 
          ...
    
def total_energy_nbody(r_list: npt.NDArray[np.float64], 
                       v_list: npt.NDArray[np.float64], 
                       m_list: npt.NDArray[np.float64], g: float=6.6743e-11) -> \
    tuple[float, float, float]: ...
    
def all_planet_acc_nbody(r_list: npt.NDArray[np.float64], 
                         m_list: npt.NDArray[np.float64], g: float=6.6743e-11) -> \
    npt.NDArray[np.float64]: ...