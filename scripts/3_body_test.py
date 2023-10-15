import nbodysolver
import numpy as np
import matplotlib.pyplot as plt
from timeit import timeit

sun_earth_moon_system = {
    "r_list": [
        [0, 0, 0],          # Position of first body, Sun
        [149.597e9, 0, 0],  # Position of second body, Earth
        [149.981e9, 0, 0]   # Position of third body, Moon
    ],
    "V_list": [
        [0, 0, 0],          # Velocity of first body, Sun
        [0, 29800, 0],      # Velocity of second body, Earth
        [0, 30800, 0]       # Velocity of third body, Moon
    ], 
    "m_list": [
        1.989e30,                     # Mass of first body, Sun
        5.972e24,                     # Mass of second body, Earth
        7.3476e22                     # Mass of third body, Moon
    ]}

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
    "m_list": [
        1.989e30,                     # Mass of first body, Sun
        5.972e24,                     # Mass of second body, Earth
        7.3476e22                     # Mass of third body, Moon
    ]}

def all_planet_acc_nbody(r_list, m_list, G=6.6743e-11):
    acc_list = []
    planet_cnt = len(r_list)

    for i in range(planet_cnt):
        i_acc = np.array([0.0, 0.0, 0.0])
        for j in range(planet_cnt):
            if i != j:
                ri = r_list[i]
                rj = r_list[j]
                mj = m_list[j]
                i_acc += (-G * mj * (ri - rj)) / (np.linalg.norm(ri - rj) ** 3)
        acc_list.append(i_acc)

    return acc_list

def total_energy_nbody(r_list, V_list, m_list, G=6.6743e-11):
    """
    Calculates the kinetic energy, potential energy and total energy in the system, 
    given the current positions, velocities and masses of the objects in the system.
    """
    total_energy = 0
    kinetic_energy = 0
    potential_energy = 0

    # Calculate the total Kinetic Energy (KE)
    for v, m in zip(V_list, m_list):
        calc_kinetic_energy = 0.5 * m * (np.linalg.norm(v) ** 2)
        kinetic_energy += calc_kinetic_energy

    # Calculate the total Potential Energy (PE)
    planet_cnt = len(r_list)
    for i in range(planet_cnt-1):
        for j in range(i+1, planet_cnt):
            mi = m_list[i]
            mj = m_list[j]
            ri = r_list[i]
            rj = r_list[j]
            potential_energy += (-G * mi * mj) / (np.linalg.norm(ri - rj))

    # Return the KE, PE and Total Energy (TE)
    total_energy = kinetic_energy + potential_energy
    return kinetic_energy, potential_energy, total_energy

iterations = 1000

def rust_acc_solver(): nbodysolver.all_planet_acc_nbody_py(sun_earth_moon_system["r_list"], sun_earth_moon_system["m_list"], 6.6743e-11)
def np_acc_solver(): all_planet_acc_nbody(sun_earth_moon_system_np["r_list"], sun_earth_moon_system_np["m_list"], 6.6743e-11)

print("Rust Acceleration Results:", nbodysolver.all_planet_acc_nbody_py(sun_earth_moon_system["r_list"], sun_earth_moon_system["m_list"], 6.6743e-11))
print("Python Acceleration Results:", all_planet_acc_nbody(sun_earth_moon_system_np["r_list"], sun_earth_moon_system_np["m_list"], 6.6743e-11))

time_1 = timeit("rust_acc_solver()", number=iterations, globals=globals())
time_2 = timeit("np_acc_solver", number=iterations, globals=globals())

print(f"Rust Acceleration Solver: {time_1 / iterations} seconds")
print(f"Numpy Acceleration Solver: {time_2 / iterations} seconds")


def rust_energy_solver(): nbodysolver.total_energy_nbody_py(sun_earth_moon_system["r_list"], sun_earth_moon_system["V_list"], sun_earth_moon_system["m_list"], 6.6743e-11)
def np_energy_solver(): total_energy_nbody(sun_earth_moon_system_np["r_list"], sun_earth_moon_system_np["V_list"], sun_earth_moon_system_np["m_list"])

print("Rust Energy Results:", nbodysolver.total_energy_nbody_py(sun_earth_moon_system["r_list"], sun_earth_moon_system["V_list"], sun_earth_moon_system["m_list"], 6.6743e-11))
print("Python Energy Results:", total_energy_nbody(sun_earth_moon_system_np["r_list"], sun_earth_moon_system_np["V_list"], sun_earth_moon_system_np["m_list"]))

time_3 = timeit("rust_energy_solver()", number=iterations, globals=globals())
time_4 = timeit("np_energy_solver()", number=iterations, globals=globals())

print(f"Rust Energy Solver: {time_3 / iterations} seconds")
print(f"Numpy Energy Solver: {time_4 / iterations} seconds")