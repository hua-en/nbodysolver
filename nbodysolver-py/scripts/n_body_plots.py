import nbodysolver
import matplotlib.pyplot as plt

#------------------------------------------- Simulate ---------------------------------#
line_options=[
    {"label": "Sun", "color": "Orange"},                             # Options for Line 1 (Sun)
    {"label": "Earth", "color": "Darkblue", "lw": 2},                # Options for Line 2 (Earth)
    {"label": "Moon", "color": "Grey", "lw": 0.75},                  # Options for Line 3 (Moon)
]

results = nbodysolver.simulate_system(nbodysolver.sun_earth_moon_system, 
                                      1000., 31536000., 6.6743e-11)
results2 = nbodysolver.simulate_system(nbodysolver.sun_earth_moon_system_modified, 
                                       1000., 31536000., 6.6743e-11)
results3 = nbodysolver.simulate_system(nbodysolver.figure_8_system, 0.01, 30., 1.)

#--------------------------------------------- Plot -----------------------------------#
fig, ax = nbodysolver.plot_position_nbody(results[1], "Sun Earth Moon System", line_options)
plt.show()
fign, axn = nbodysolver.plot_position_nbody(results2[1], "5 body problem")
plt.show()
fig8, ax8 = nbodysolver.plot_position_nbody(results3[1], "Figure 8 System")
plt.show()

fige, axe = nbodysolver.plot_energy(results2[0], results2[3], 
                                    results2[4], results2[5], 
                                    "Energy in 5 body problem")
plt.show()
figv, axv = nbodysolver.plot_velocity_nbody(results2[0], results2[2], 
                                            "Velocities in 5 body problem")
plt.show()

#-------------------------------------------- Animate ---------------------------------#
line_options=[
    {"label": "Sun", "color": "Orange"},                             # Options for Line 1 (Sun)
    {"label": "Earth", "color": "Darkblue", "lw": 2},                # Options for Line 2 (Earth)
    {"label": "Moon", "color": "Grey", "lw": 0.75},                  # Options for Line 3 (Moon)
    {"label": "L4 Point", "color": "Grey", "lw": 0.75},              # Options for Line 4 (L4 Point)
    {"label": "L5 Point", "color": "Grey", "lw": 0.75},              # Options for Line 5 (L5 Point)
]
point_options=[
    {"ls": "", "color": "Orange", "marker": "o"},                    # Options for Point 1 (Sun)
    {"ls": "", "color": "Darkblue", "marker": "o", "markersize": 5}, # Options for Point 2 (Earth)
    {"ls": "", "color": "Grey", "marker": "o", "markersize": 2},     # Options for Point 3 (Moon)
    {"ls": "", "color": "Grey", "marker": "o", "markersize": 2},     # Options for Point 4 (L4 Point)
    {"ls": "", "color": "Grey", "marker": "o", "markersize": 2}      # Options for Point 5 (L5 Point)
]


# Create the animation
nbody_animation = nbodysolver.animate_data_nbody(results2[1], 200, 
                                                 "N-Body System", 
                                                 line_options,
                                                 point_options)

# Save the animation in a video file
nbody_animation.save("N-Body System.mp4")
