import matplotlib.pyplot as plt
import numpy as np

def plot_double(data_1, data_2):
    plt.subplot(1,2,1)
    plt.plot(data_1)
    plt.subplot(1, 2, 2)
    plt.plot(data_2)
    plt.show()

def plot_epochs_ran(data):
    data['MA_epochs'] = data['epochs_ran'].rolling(window=10000).mean()
    
    fig, ax = plt.subplots()
    ax.plot(data['MA_epochs'])
    ax.set(xlabel = "Generation #", ylabel = "(averaged) Epochs Survived", 
        title = "Average epochs ran per generation")
    #ax.set_ylim([0, 200])
    plt.show()

def plot_agents_alive(data):
    data['MA_alive'] = data['agents_alive'].rolling(window=10000).mean()
    
    fig, ax = plt.subplots()
    ax.plot(data['MA_alive'])
    ax.set(xlabel = "Generation #", ylabel = "(averaged) Agents Alive", 
        title = "Average agents alive per generation")
    #ax.set_ylim([0, 200])
    plt.show()



def plot_quad(data_1, data_2, data_3, data_4):
    plt.subplot(2,2,1)
    plt.plot(data_1)
    plt.title("Epochs alive")
    plt.subplot(2, 2, 2)
    plt.plot(data_2)
    plt.title("Epochs alive - average")
    plt.subplot(2,2,3)
    plt.plot(data_3)
    plt.title("Agents alive")
    plt.subplot(2, 2, 4)
    plt.plot(data_4)
    plt.title("Agents alive - average")
    plt.show()


def plot_performance_over_time(data):
    plt.plot(data)
    plt.show()
