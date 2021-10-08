

def equilibrium_calculator(agents, max_r, r):
    agents_allowance = (max_r - (max_r / r)) / agents
    suggested_init = (max_r - (max_r / r))
    print(f"{agents_allowance = } {suggested_init = }")


def main():
    equilibrium_calculator(1, 9, 1.5)

if __name__ == "__main__":
    main()