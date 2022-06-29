import matplotlib.pyplot as plt



threads_info = None
x_data = None

with open("threads_data") as f:
    threads_info = f.readline()
threads_info = [int(x) for x in threads_info.split(" ")]

with open("seq_tests_data") as f:
    x_data = f.readline()
x_data = [int(x)**2 for x in x_data.split(" ")]

plt.figure()

with open("bench_results") as f:
    lines = f.readlines()
    y_data = [float(line.split()[1]) for line in lines]
    plt.plot(x_data, y_data, label="sequential")

for t in threads_info:
    with open(f"parallel_results_{t}") as f:
        lines = f.readlines()
        y_data = [float(line.split()[1]) for line in lines]
        plt.plot(x_data, y_data, label=f"{t} threads")

with open("seq_optimized") as f:
    lines = f.readlines()
    y_data = [float(line.split()[1]) for line in lines]
    plt.plot(x_data, y_data, label="seq opt")

with open("parallel_8_optimized") as f:
    lines = f.readlines()
    y_data = [float(line.split()[1]) for line in lines]
    plt.plot(x_data, y_data, label="8 threads opt")

plt.legend(loc="upper left")
plt.xlabel("m * n")
plt.ylabel("time ns")
plt.savefig("fill_graph.png")

plt.figure()
with open("bench_results") as f:
    lines = f.readlines()
    y_data = [float(line.split()[2]) for line in lines]
    plt.plot(x_data, y_data, label="sequential")

for t in threads_info:
    with open(f"parallel_results_{t}") as f:
        lines = f.readlines()
        y_data = [float(line.split()[2]) for line in lines]
        plt.plot(x_data, y_data, label=f"{t} threads")

with open("seq_optimized") as f:
    lines = f.readlines()
    y_data = [float(line.split()[2]) for line in lines]
    plt.plot(x_data, y_data, label="seq opt")

with open("parallel_8_optimized") as f:
    lines = f.readlines()
    y_data = [float(line.split()[2]) for line in lines]
    plt.plot(x_data, y_data, label="8 threads opt")

plt.legend(loc="upper left")
plt.xlabel("m * n")
plt.ylabel("time ns")
plt.savefig("calc_graph.png")