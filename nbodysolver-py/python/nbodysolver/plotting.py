import matplotlib.pyplot as plt
import numpy as np
import numpy.typing as npt
import matplotlib.animation as animation
from typing import Any

def plot_position_nbody(pos_data: npt.NDArray[np.float64], fig_title: str, 
                        line_options: list[dict[str, Any]] | None = None):
    """
    Simple plotting function that plots the dataset on a 3D figure and axis.
    """
    # Check if line_options is initialised
    # If not, initialise default labels
    obj_cnt = pos_data.shape[1]
    if line_options is None:
        line_options = [{"label":f"Object {i+1}"} for i in range(obj_cnt)]
    
    # Plot data
    fig, ax = plt.subplots(subplot_kw={"projection": "3d"})
    for i in range(0, obj_cnt):
        ax.plot3D(pos_data[:, i, 0], pos_data[:, i, 1], pos_data[:, i, 2], 
                  **line_options[i])
    ax.set_xlabel('x')
    ax.set_ylabel('y')
    ax.set_zlabel('z')
    ax.set_title(fig_title)
    ax.legend(loc="upper left")

    return fig, ax

def animate_data_nbody(pos_data: npt.NDArray[np.float64], 
                       animation_frames: int, 
                       fig_title: str, 
                       line_options: list[dict[str, Any]] | None = None,
                       point_options: list[dict[str, Any]] | None = None, 
                       animation_interval: int=25):
    """Animates a dataset using matplotlib.animate, 
    given a dataset and the number of frames to animate.
    
    Returns the matplotlib animation object, 
    which can be rendered into a video with anim.save() 
    and/or embedded as a HTML video with HTML(anim.to_html5_video()).

    The figure title can be specified with fig_title.
    The properties of the lines and points in the plot, such as the colours, 
    can be customised with line_point_options.

    Args:
        timesteps: Time data obtained from simulate_nbody.
        pos_data: Position data obtained from simulate_nbody.
        animation_frames (int): Number of frames to animate the simulation for. 
        40 frames roughly correspond to one second.
        fig_title: The title of the animation.
        line_point_options: Sets customisation options for the lines 
        and points in the animation. 
        animation_interval (optional): Sets the time between animation frames. 
        Default = 25ms.
    """

    # --- Process Dataset ---------------------------------------------#
    # Find number of timesteps in dataset
    no_of_timesteps = pos_data.shape[0]

    # Find number of objects in dataset
    obj_cnt = pos_data.shape[1]

    # Find x, y and z limits in the data by finding the minimum and maximum x, y and z
    min_x, max_x = np.min(pos_data[:, :, 0]), np.max(pos_data[:, :, 0])
    min_y, max_y = np.min(pos_data[:, :, 1]), np.max(pos_data[:, :, 1])
    min_z, max_z = np.min(pos_data[:, :, 2]), np.max(pos_data[:, :, 2])

    # --- Create Figure ----------------------------------------------#
    # Check if options are set
    if line_options is None:
        line_options = [{"label":f"Object {i+1}"} for i in range(obj_cnt)]
    if point_options is None:
        point_options = [{"marker": "o", "ls": ""} for i in range(obj_cnt)]
    
    # Create Lines and Points within the animation
    figanim, axanim = plt.subplots(subplot_kw={"projection": "3d"})

    line3d_lst = []
    point3d_lst = []
    for i in range(1, obj_cnt + 1):
        line3d = axanim.plot([], [], [], **line_options[i-1])
        line3d = line3d[0]
        point3d = axanim.plot([], [], [], **point_options[i-1])
        point3d = point3d[0]
        line3d_lst.append(line3d)
        point3d_lst.append(point3d)

    # Set Axis Limits
    axanim.set_xlim((min_x, max_x))
    axanim.set_ylim((min_y, max_y))
    axanim.set_zlim((min_z, max_z))

    # Add labels to figure
    axanim.grid(alpha=0.25)
    axanim.legend(loc='upper left')
    axanim.set_title(fig_title)
    axanim.set_xlabel('x')
    axanim.set_ylabel('y')
    axanim.set_zlabel('z')

    # --- Define Animation Functions ----------------------------------------#
    # Init Function, called to generate the first frame of the animation

    def init():
        for line3d in line3d_lst:
            line3d.set_data([], [])
            line3d.set_3d_properties([])

        return tuple([*line3d_lst, *point3d_lst])

    # Animate Function, called repeatedly after init() to generate the frames for the animation.
    # Each frame (e.g. the ith frame) of the animation is created by calling animate(i), then the figure is rendered.
    # This process is repeated until the animation is complete.
    def animate(i):
        j = i * (no_of_timesteps // animation_frames)

        for cur_obj, (line3d, point3d) in enumerate(zip(line3d_lst, point3d_lst)):
            line3d.set_data(pos_data[:j, cur_obj, 0], pos_data[:j, cur_obj, 1])
            line3d.set_3d_properties(pos_data[:j, cur_obj, 2])
            point3d.set_data(pos_data[j-1:j, cur_obj, 0], pos_data[j-1:j, cur_obj, 1])
            point3d.set_3d_properties(pos_data[j-1:j, cur_obj, 2])

        # figanim.canvas.draw()
        return tuple([*line3d_lst, *point3d_lst])

    # --- Animate --------------------------------------------------#
    anim = animation.FuncAnimation(figanim, animate, init_func=init, frames=animation_frames,
                                   interval=animation_interval, blit=True)

    return anim

def plot_energy(time: npt.NDArray[np.float64], 
                KE: npt.NDArray[np.float64], 
                PE: npt.NDArray[np.float64], 
                TE: npt.NDArray[np.float64], fig_title: str,
                line_options: list[dict[str, Any]] | None = None):
    """
    Simple plotting function that plots the kinetic energy, potential energy 
    and total energy of the system on a graph.
    """
    # Plot data
    if line_options is None:
        line_options = [{"label":"Kinetic Energy"}, 
                        {"label":"Gravitational Potential Energy"}, 
                        {"label":"Total Energy"}]

    fig, ax = plt.subplots()
    ax.plot(time, KE, **line_options[0])
    ax.plot(time, PE, **line_options[1])
    ax.plot(time, TE, **line_options[2])
    ax.set_title(fig_title)
    ax.set_xlabel("Time/s")
    ax.set_ylabel("Energy/J")
    ax.legend(loc="upper right")
    
    return fig, ax

def plot_velocity_nbody(time: npt.NDArray[np.float64], 
                        velocity_data: npt.NDArray[np.float64], fig_title: str,
                        line_options: list[dict[str, Any]] | None = None):
    """
    Simple plotting function that plots the velocities of the objects on a graph.
    """
    # Check if line_options is initialised
    obj_cnt = velocity_data.shape[1]
    if line_options is None:
        line_options = [{"label":f"Object {i+1}"} for i in range(obj_cnt)]
    
    # Process data
    velocity_data = np.linalg.norm(velocity_data, axis=2)
    
    # Plot data
    fig, ax = plt.subplots()
    for i in range(0, obj_cnt):
        ax.plot(time, velocity_data[:, i], **line_options[i])
    ax.set_xlabel("Time/s")
    ax.set_ylabel("Velocity/$ms^{-1}$")
    ax.set_title(fig_title)
    ax.legend(loc="upper right")
    
    return fig, ax