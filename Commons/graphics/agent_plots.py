import matplotlib.pyplot as plt
import numpy as np

def gen_states():
    state = ["LOW", "MEDIUM", "HIGH"]
    states = ["{}_{}".format(state_1, state_2) for state_1 in state for state_2 in state]
    return states

def actions_ev_chart(data):
    #data = (data-data.mean())/data.std()
    data = (data-data.min())/(data.max()-data.min())
    counter = 0
    for state_1 in ["LOW", "MEDIUM", "HIGH"]:
        for state_2 in ["LOW", "MEDIUM", "HIGH"]:
            counter += 1
            plt.subplot(3,3, counter)
            plt.bar(np.arange(len(data["{}_{}".format(state_1, state_2)])), data["{}_{}".format(state_1, state_2)])
            plt.title("c: {} s: {}".format(state_1, state_2))
    plt.show()

def actions_per_state(data, actions):
    states = gen_states()
    data = (data-data.min())/(data.max()-data.min())
    for i in range(actions):
        plt.subplot(3,3, i)
        plt.bar(np.arange(len(states), data["{}_{}".format()])) # got to transform the data first

def state_visits(data, actions):
    counts = {}
    states = gen_states()
    for state in states:
        for i in range(actions-1):
            data["{}_{}".format(state, 0)] += data["{}_{}".format(state, i+1)]
        counts[state] = data["{}_{}".format(state, 0)].sum()
    plt.bar(counts.keys(), counts.values())
    plt.show()