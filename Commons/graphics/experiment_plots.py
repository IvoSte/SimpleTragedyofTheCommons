import matplotlib.pyplot as plt
import numpy as np

def plot_double(data_1, data_2):
    plt.subplot(1,2,1)
    plt.plot(data_1)
    plt.subplot(1, 2, 2)
    plt.plot(data_2)
    plt.show()

def plot_quad(data_1, data_2, data_3, data_4):
    plt.subplot(2,2,1)
    plt.plot(data_1)
    plt.subplot(2, 2, 2)
    plt.plot(data_2)
    plt.subplot(2,2,3)
    plt.plot(data_3)
    plt.subplot(2, 2, 4)
    plt.plot(data_4)
    plt.show()


def plot_performance_over_time(data):
    plt.plot(data)
    plt.show()
