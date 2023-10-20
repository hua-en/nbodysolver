import nbodysolver
import matplotlib.pyplot as plt
import numpy as np

a=1.3321302
b=2.3913181

periodic_system = nbodysolver.NBodySystem(
    np.array([
        [0.0, 0.0, 0.0],          # Position of first body, Sun
        [10., 0.0, 0.0],  # Position of second body, Earth
        [-10., 0.0, 0.0]   # Position of third body, Moon
    ]),
    np.array([
        [0.0, 0.0, b],          # Velocity of first body, Sun
        [0.0, a, -b],      # Velocity of second body, Earth
        [0.0, -a, -b]       # Velocity of third body, Moon
    ]),
    np.array([
        200.,                     # Mass of first body, Sun
        100.,                     # Mass of second body, Earth
        100.                     # Mass of third body, Moon
    ]))

# Simulation Variables
dt = 0.0001
max_time = 80

# Simulation Constants
g=1

results = nbodysolver.simulate_system(periodic_system, dt, max_time, g)
fig1, ax1 = nbodysolver.plot_position_nbody(results, "Periodic System")
plt.show()

line_options=[
    {"label": "Object 1", "color": "Orange"},             
    {"label": "Object 2", "color": "Red"},        
    {"label": "Object 3", "color": "Blue"},       
]
point_options=[
    {"ls": "", "color": "Orange", "marker": "o"},  
    {"ls": "", "color": "Red", "marker": "o"},
    {"ls": "", "color": "Blue", "marker": "o"},   
]

periodic_system_animation = nbodysolver.animate_data_nbody(results,
                                                           200, 
                                                           "Periodic System",
                                                           line_options,
                                                           point_options)

periodic_system_animation.save("periodicsystem.mp4")