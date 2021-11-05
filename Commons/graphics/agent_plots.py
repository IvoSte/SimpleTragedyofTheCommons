import matplotlib.pyplot as plt
import numpy as np

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

