import pyo3_rust2numpy_example as rust
import numpy as np
import matplotlib.pyplot as plt

sine_data = rust.sine_wave(48000,10.0)
plt.plot(sine_data)
plt.show()
