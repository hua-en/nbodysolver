import nbodysolver
import numpy as np
import matplotlib.pyplot as plt

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

sun_earth_moon_system_modified = {
    "r_list": [
        np.array([0, 0, 0]),          # Position of first body, Sun
        np.array([149.597e9, 0, 0]),  # Position of second body, Earth
        np.array([149.981e9, 0, 0]),  # Position of third body, Moon
        np.array([74.435e9, 128.92e9, 0]),# Position of fourth body, L4
        np.array([74.435e9, -128.92e9, 0]) # Position of fifth body, L5
    ],
    "V_list": [
        np.array([0, 0, 240000]),          # Velocity of first body, Sun
        np.array([0, 29800, 240000]),      # Velocity of second body, Earth
        np.array([0, 30800, 240000]),      # Velocity of third body, Moon
        np.array([-25807, 14900, 240000]), # Velocity of fourth body, L4
        np.array([25807, 14900, 240000])  # Velocity of fourth body, L5
    ], 
    "m_list": np.array([
        1.989e30,                     # Mass of first body, Sun
        5.972e24,                     # Mass of second body, Earth
        7.3476e22,                    # Mass of third body, Moon
        6000,                         # Mass of fourth body, a small mass at L4
        6000,                         # Mass of fifth body, a small mass at L5
    ])}

results = nbodysolver.simulate_nbody_and_process_py(sun_earth_moon_system_np['r_list'], 
                                                    sun_earth_moon_system_np['V_list'], 
                                                    sun_earth_moon_system_np['m_list'], 
                                                    1000., 31536000., 6.6743e-11)
results2 = nbodysolver.simulate_nbody_and_process_py(sun_earth_moon_system_modified['r_list'],
                                                    sun_earth_moon_system_modified['V_list'],
                                                    sun_earth_moon_system_modified['m_list'],
                                                    1000., 31536000., 6.6743e-11)

def plot_position_nbody(pos_data, fig_title):
    """
    Simple plotting function that plots the dataset on a 3D figure and axis.
    """
    # Plot data
    fig, ax = plt.subplots(subplot_kw={"projection": "3d"})
    for i in range(0, pos_data.shape[1], 3):
        ax.plot3D(pos_data[:, i], pos_data[:, i+1], pos_data[:, i+2], 
                  label=f"Object {(i+1)//3}")
    ax.set_xlabel('x')
    ax.set_ylabel('y')
    ax.set_zlabel('z')
    ax.set_title(fig_title)
    ax.legend(loc="upper left")

    return fig, ax


fig, ax = plot_position_nbody(results[1], "Sun Earth Moon System")
plt.show()
fign, axn = plot_position_nbody(results2[1], "5 body problem")
plt.show()
