# libraries
import matplotlib.pyplot as plt
import numpy as np
from numpy import genfromtxt
import math

#data = genfromtxt('my_file.csv', delimiter=',')
values = np.cumsum(np.random.randn(1000, 1))


def plot_double(data_1, data_2):
    plt.subplot(1,2,1)
    plt.plot(data_1)
    plt.subplot(1, 2, 2)
    plt.plot(data_2)
    plt.show()

def plot_performance_over_time(data):
    plt.plot(data)
    plt.show()

def bin_values(values, binsize):
    new_length = math.ceil(len(values) / binsize)
    binned_values = np.zeros((new_length))
    for i in range(new_length):
        binned_values[i] = float(sum(values[i * binsize: i * binsize + binsize])) / binsize
    return binned_values

def main():
    print("Producing plot")
    # plot_performance_over_time(values)
    binned_values = bin_values(values, 10)
    # plot_performance_over_time(binned_values)
    plot_double(values, binned_values)
    print("Done")

if __name__ == "__main__":
    main()
