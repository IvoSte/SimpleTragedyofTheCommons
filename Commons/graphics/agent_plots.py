import matplotlib.pyplot as plt
import numpy as np

def actions_ev_chart(data):
    counter = 0
    for state_1 in ["LOW", "MEDIUM", "HIGH"]:
        for state_2 in ["LOW", "MEDIUM", "HIGH"]:
            counter += 1
            plt.subplot(3,3, counter)
            plt.bar(np.arange(len(data["{} {}".format(state_1, state_2)])), data["{} {}".format(state_1, state_2)])
            plt.title("{} {}".format(state_1, state_2))
    plt.show()

