# libraries
import matplotlib.pyplot as plt
import numpy as np
from numpy import genfromtxt
import math
import pandas as pd
from agent_plots import actions_ev_chart, state_visits
from experiment_plots import plot_double, plot_quad, plot_performance_over_time
from transform_data import average_results

#agent_ev = pd.read_csv('../data/experiment_1/long/avg_rl_stats.csv')
# agent_ev = pd.read_csv('../data/single_agent/avg_rl_stats.csv')
agent_ev = pd.read_csv('../data/commons_purge_oiv/avg_rl_stats.csv')
data = genfromtxt('../data/test.csv', delimiter=',')
#p_data = pd.read_csv('../data/prelim_final/0/gen_stats.csv')
p_data = average_results('../data/commons_purge_oiv', 'gen_stats.csv', 12)
#values = np.cumsum(np.random.randn(1000, 1))



def bin_values(values, binsize):
    new_length = math.ceil(len(values) / binsize)
    binned_values = np.zeros((new_length))
    for i in range(new_length):
        binned_values[i] = float(sum(values[i * binsize: i * binsize + binsize])) / binsize
    return binned_values

def plot_with_pandas(data):
    data['MA_epochs'] = data['epochs_ran'].rolling(window=1000).mean()
    data['MA_alive'] = data['agents_alive'].rolling(window=1000).mean()
    plot_quad(data['epochs_ran'], data['MA_epochs'], data['agents_alive'], data['MA_alive'])

def main():

    dataset = p_data
    print(p_data.head())

    print("Producing plot")
    # plot_performance_over_time(values)
    #binned_values = bin_values(dataset, 10)
    #plot_performance_over_time(binned_values)
    #plot_double(dataset, binned_values)

    plot_with_pandas(p_data)
    actions_ev_chart(agent_ev)
    state_visits(p_data, 5)
    print("Done")

if __name__ == "__main__":
    main()
