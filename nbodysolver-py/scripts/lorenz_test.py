import nbodysolver
import matplotlib.pyplot as plt
from timeit import timeit

result = nbodysolver.solve_lorenz((0, 1, 0), 50, 0.001)

ax = plt.axes(projection='3d')
ax.plot3D(result[0], result[1], result[2])
plt.show()

iterations = 1000
total_time = timeit("nbodysolver.solve_lorenz((0, 1, 0), 50, 0.001)", 
                    number=iterations, globals=globals())
print(f"Average time is {total_time / iterations} seconds")
