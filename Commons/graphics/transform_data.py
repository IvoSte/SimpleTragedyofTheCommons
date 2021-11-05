import pandas as pd

def average_results(path, filename, n_results):
    data = pd.read_csv("{}/{}/{}".format(path, 0, filename))
    for n in range(n_results-1):
        new_data = pd.read_csv("{}/{}/{}".format(path, n+1, filename))
        data += new_data
    data /= n_results
    return data
    