import numpy as np
import numpy.typing as npt
from nbodysolver.nbodysolver import simulate_nbody_and_process

class NBodySystem:
    def __init__(self, 
                 r_list: npt.NDArray[np.float64], 
                 v_list: npt.NDArray[np.float64], 
                 m_list: npt.NDArray[np.float64]):
        self.r_list = r_list
        self.v_list = v_list
        self.m_list = m_list

class NBodyResults:
    def __init__(self, 
                 all_time: npt.NDArray[np.float64], 
                 all_r: npt.NDArray[np.float64], 
                 all_v: npt.NDArray[np.float64], 
                 all_ke: npt.NDArray[np.float64], 
                 all_pe: npt.NDArray[np.float64], 
                 all_te: npt.NDArray[np.float64]):
        self.all_time = all_time
        self.all_r = all_r
        self.all_v = all_v
        self.all_ke = all_ke
        self.all_pe = all_pe
        self.all_te = all_te
        
def simulate_system(system: NBodySystem, dt: float, 
                    max_time: float, g: float = 6.6743e-11) -> NBodyResults:
    return NBodyResults(*simulate_nbody_and_process(system.r_list, 
                                      system.v_list, 
                                      system.m_list, dt, max_time, g))