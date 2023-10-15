import nbodysolver
import numpy as np
import matplotlib.pyplot as plt
from timeit import timeit

result = nbodysolver.solve_lorenz((0, 1, 0), 50, 0.001)

ax = plt.axes(projection='3d')
ax.plot3D(result['x'], result['y'], result['z'])
plt.show()

iterations = 1000
total_time = timeit("nbodysolver.solve_lorenz((0, 1, 0), 50, 0.001)", number=iterations, globals=globals())

print(f"Average time is {total_time / iterations} seconds")

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

def rust_solver(): nbodysolver.all_planet_acc_nbody_py(sun_earth_moon_system["r_list"], sun_earth_moon_system["m_list"], 6.6743e-11)
def np_solver(): all_planet_acc_nbody(sun_earth_moon_system_np["r_list"], sun_earth_moon_system_np["m_list"], 6.6743e-11)

time_1 = timeit("rust_solver()", number=iterations, globals=globals())
time_2 = timeit("np_solver", number=iterations, globals=globals())

print(f"Rust Solver: {time_1 / iterations} seconds")
print(f"Numpy Solver: {time_2 / iterations} seconds")