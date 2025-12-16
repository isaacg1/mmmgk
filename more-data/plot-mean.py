import sys
import matplotlib.pyplot as plt
filename = sys.argv[1]
outname = sys.argv[2]
# assert filename.endswith(".csv")
assert outname.endswith(".eps")

datas = []
with open(filename) as file:
    for line in file:
        line = line.strip()
        if not line:
            continue
        if line[0] == 'l':
            datas.append([])
        if line[1] == '.':
            datas[-1].append([float(i) for i in line.split(';') if i])

plt.figure(figsize=(5, 3))
rhos = [row[0] for row in datas[0]]
largest_stddev = 0
for server in range(1, len(datas[0][0])):
    means = []
    p95s = []
    for load in range(len(rhos)):
        rho = rhos[load]
        results = []
        for seed in range(len(datas)):
            try:
                results.append(datas[seed][load][server])
            except: pass
        mean = sum(results)/len(results)*(1-rho)
        stddev = sum((result*(1-rho) - mean) ** 2 for result in results)/(len(results) - 1)
        p95 = 1.96*stddev
        means.append(mean)
        p95s.append(p95)
        if stddev > largest_stddev:
            largest_stddev = stddev
            print(largest_stddev)
    plt.errorbar(rhos, means, yerr=p95s, label="Server " + str(server))

plt.plot([1], [71/108], 'o', color='k', label='Predicted heavy traffic limit')
plt.xlabel("Load $\\rho$")
plt.ylabel("Scaled queue length $E[q_j](1-\\rho)$")
plt.legend()
plt.savefig(outname, bbox_inches='tight')

