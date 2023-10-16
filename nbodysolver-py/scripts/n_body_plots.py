import nbodysolver
import numpy as np
import matplotlib.pyplot as plt

#------------------------------------------- Simulate --------------------------------------------#
results = nbodysolver.simulate_system(nbodysolver.sun_earth_moon_system, 1000., 31536000., 6.6743e-11)
results2 = nbodysolver.simulate_system(nbodysolver.sun_earth_moon_system_modified, 1000., 31536000., 6.6743e-11)
results3 = nbodysolver.simulate_system(nbodysolver.figure_8_system, 0.01, 30., 1.)
results4 = nbodysolver.simulate_system_np(nbodysolver.sun_earth_moon_system_np, 1000., 31536000., 6.6743e-11)

#--------------------------------------------- Plot ----------------------------------------------#
fig, ax = nbodysolver.plot_position_nbody(results[1], "Sun Earth Moon System")
plt.show()
fign, axn = nbodysolver.plot_position_nbody(results2[1], "5 body problem")
plt.show()
fig8, ax8 = nbodysolver.plot_position_nbody(results3[1], "Figure 8 System")
plt.show()


#-------------------------------------------- Animate --------------------------------------------#
nbody_customoptions = {"line1":  {"label": "Sun", "color": "Orange"},                               # Options for Line 1 (Sun)
                       "line2":  {"label": "Earth", "color": "Darkblue", "lw": 2},                  # Options for Line 2 (Earth)
                       "line3":  {"label": "Moon", "color": "Grey", "lw": 0.75},                    # Options for Line 3 (Moon)
                       "line4":  {"label": "L4 Point", "color": "Grey", "lw": 0.75},                # Options for Line 4 (L4 Point)
                       "line5":  {"label": "L5 Point", "color": "Grey", "lw": 0.75},                # Options for Line 5 (L5 Point)
                       "point1": {"ls": "", "color": "Orange", "marker": "o"},                      # Options for Point 1 (Sun)
                       "point2": {"ls": "", "color": "Darkblue", "marker": "o", "markersize": 5},   # Options for Point 2 (Earth)
                       "point3": {"ls": "", "color": "Grey", "marker": "o", "markersize": 2},       # Options for Point 3 (Moon)
                       "point4": {"ls": "", "color": "Grey", "marker": "o", "markersize": 2},       # Options for Point 4 (L4 Point)
                       "point5": {"ls": "", "color": "Grey", "marker": "o", "markersize": 2}}       # Options for Point 5 (L5 Point)


# Create the animation
nbody_animation = nbodysolver.animate_data_nbody(results2[0], results2[1], 200, "N-Body System", nbody_customoptions)

# Save the animation in a video file
nbody_animation.save("N-Body System.mp4")
