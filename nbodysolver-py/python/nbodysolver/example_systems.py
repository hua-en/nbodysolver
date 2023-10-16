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


def simulate_system(system: NBodySystem, dt: float, 
                    max_time: float, g: float = 6.6743e-11) -> \
    tuple[list[float], npt.NDArray[np.float64], npt.NDArray[np.float64], 
          list[float], list[float], list[float]]:
    return simulate_nbody_and_process(system.r_list, 
                                      system.v_list, 
                                      system.m_list, dt, max_time, g)


sun_earth_moon_system = NBodySystem(
    np.array([
        [0.0, 0.0, 0.0],          # Position of first body, Sun
        [149.597e9, 0.0, 0.0],  # Position of second body, Earth
        [149.981e9, 0.0, 0.0]   # Position of third body, Moon
    ]),
    np.array([
        [0.0, 0.0, 0.0],          # Velocity of first body, Sun
        [0.0, 29800.0, 0.0],      # Velocity of second body, Earth
        [0.0, 30800.0, 0.0]       # Velocity of third body, Moon
    ]),
    np.array([
        1.989e30,                     # Mass of first body, Sun
        5.972e24,                     # Mass of second body, Earth
        7.3476e22                     # Mass of third body, Moon
    ]))

sun_earth_moon_system_modified = NBodySystem(
    np.array([
        [0., 0., 0.],          # Position of first body, Sun
        [149.597e9, 0., 0.],  # Position of second body, Earth
        [149.981e9, 0., 0.],  # Position of third body, Moon
        [74.435e9, 128.92e9, 0.],  # Position of fourth body, L4
        [74.435e9, -128.92e9, 0.]  # Position of fifth body, L5
    ]),
    np.array([
        [0., 0., 240000.],          # Velocity of first body, Sun
        [0., 29800., 240000.],      # Velocity of second body, Earth
        [0., 30800., 240000.],      # Velocity of third body, Moon
        [-25807., 14900., 240000.],  # Velocity of fourth body, L4
        [25807., 14900., 240000.]  # Velocity of fourth body, L5
    ]),
    np.array([
        1.989e30,                     # Mass of first body, Sun
        5.972e24,                     # Mass of second body, Earth
        7.3476e22,                    # Mass of third body, Moon
        6000.,                         # Mass of fourth body, a small mass at L4
        6000.,                         # Mass of fifth body, a small mass at L5
    ]))

# Should be used with dt = 0.01, max_time = 30, g = 1
figure_8_system = NBodySystem(
    np.array([
        [0.97000436, -0.24308753, 0.],      # Position of first body
        [0., 0., 0.],                         # Position of second body
        [-0.97000436, 0.24308753, 0.]       # Position of third body
    ]),
    np.array([
        [0.4662036850,  0.4323657300, 0.],  # Velocity of first body
        [-0.93240737, -0.86473146, 0.],     # Velocity of second body
        [0.4662036850, 0.4323657300, 0.]    # Velocity of third body
    ]),
    np.array([
        1.,                                           # Mass of first body
        1.,                                           # Mass of second body
        1.,                                            # Mass of third body
    ]))
