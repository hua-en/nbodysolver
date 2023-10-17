import nbodysolver
from nbodysolver import sun_earth_moon_system_modified

nbodysolver.simulate_nbody_and_process(sun_earth_moon_system_modified.r_list,
                                        sun_earth_moon_system_modified.v_list,
                                        sun_earth_moon_system_modified.m_list,
                                        1000., 31536000., g=6.6743e-11)