# libraries
import matplotlib.pyplot as plt
import numpy as np
from numpy import genfromtxt
import math
import pandas as pd
from agent_plots import actions_ev_chart

agent_ev = pd.read_csv('../data/sample_agent_ev.csv')
data = genfromtxt('../data/test.csv', delimiter=',')
p_data = pd.read_csv('../data/test.csv')
values = np.cumsum(np.random.randn(1000, 1))


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

def bin_values(values, binsize):
    new_length = math.ceil(len(values) / binsize)
    binned_values = np.zeros((new_length))
    for i in range(new_length):
        binned_values[i] = float(sum(values[i * binsize: i * binsize + binsize])) / binsize
    return binned_values

def plot_with_pandas(data):
    data['MA_epochs'] = data['epochs_stats'].rolling(window=1000).mean()
    data['MA_alive'] = data['agents_alive'].rolling(window=1000).mean()
    plot_quad(data['epochs_stats'], data['MA_epochs'], data['agents_alive'], data['MA_alive'])


def main():

    dataset = data
    print(p_data.head())

    print("Producing plot")
    # plot_performance_over_time(values)
#    binned_values = bin_values(dataset, 10)
    # plot_performance_over_time(binned_values)
#    plot_double(dataset, binned_values)
    # plot_with_pandas(p_data)
    actions_ev_chart(agent_ev)
    print("Done")

if __name__ == "__main__":
    main()
