import nbodysolver
import numpy as np

sun_earth_moon_system_np = {
    "r_list": [
        # np.array([[0.0, 0, 0], 
        #           [149.597e9, 0, 0], 
        #           [149.981e9, 0, 0]])
        np.array([0.0, 0.0, 0.0]),          # Position of first body, Sun
        np.array([149.597e9, 0.0, 0.0]),  # Position of second body, Earth
        np.array([149.981e9, 0.0, 0.0])   # Position of third body, Moon
    ],
    "V_list": [
        # np.array([[0.0, 0, 0], 
        #           [0.0, 29800, 0], 
        #           [0.0, 30800, 0]])
        np.array([0.0, 0.0, 0.0]),          # Velocity of first body, Sun
        np.array([0.0, 29800.0, 0.0]),      # Velocity of second body, Earth
        np.array([0.0, 30800.0, 0.0])       # Velocity of third body, Moon
    ], 
    "m_list": np.array([
        1.989e30,                     # Mass of first body, Sun
        5.972e24,                     # Mass of second body, Earth
        7.3476e22                     # Mass of third body, Moon
    ])}

results = nbodysolver.simulate_nbody_and_process_py(sun_earth_moon_system_np['r_list'], 
                                                    sun_earth_moon_system_np['V_list'], 
                                                    sun_earth_moon_system_np['m_list'], 
                                                    1000., 31536000., 6.6743e-11)