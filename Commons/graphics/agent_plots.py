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
            plt.title("commons: {} score: {}".format(state_1, state_2))
    plt.text(-5.0, -0.3, "# resources taken", ha = 'center', va = 'center', fontsize = 14)
    plt.text(-14.0, 1.8, "normalized expected value", ha='center', va='center', rotation='vertical', fontsize = 14)
    plt.suptitle("Normalized expected values of actions per state in the Q-Table", fontsize = 18)
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
        data["{}_total".format(state)] = 0
        for i in range(actions):
            data["{}_total".format(state)] += data["{}_{}".format(state, i)]
        counts[state] = data["{}_total".format(state)].sum()
    plt.bar(counts.keys(), counts.values())
    plt.xlabel("State")
    plt.ylabel("Times visited")
    plt.show()

def action_counts(data, actions):
    counts = {}
    states = gen_states()
    for i in range(actions):
        data["{}_total".format(i)] = 0
        for state in states:
            data["{}_total".format(i)] += data["{}_{}".format(state, i)]
        counts[i] = data["{}_total".format(i)].sum()
    plt.bar(counts.keys(), counts.values())
    plt.xlabel("Number of resources taken")
    plt.ylabel("Times chosen")
    plt.show()
