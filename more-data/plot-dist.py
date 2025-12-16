import sys
import matplotlib.pyplot as plt
import math
filename = sys.argv[1]
outname = sys.argv[2]
assert filename.endswith(".csv")
assert outname.endswith(".eps")


datas = []
with open(filename) as file:
    for line in file:
        line = line.strip()
        if not line:
            continue
        if line[0] == 's':
            datas.append([])
        if line[1] == ';':
            datas[-1].append([int(i) for i in line.split(';') if i])

plt.figure(figsize=(5, 3))
longest = max(max(len(data_row) for data_row in data) for data in datas)
plt.yscale('log')

num_servers = len(datas[0])

for server in range(num_servers):
    means = []
    stddevs = []
    for entry in range(1, longest):
        counts = []
        for seed in range(len(datas)):
            if entry < len(datas[seed][server]):
                counts.append(datas[seed][server][entry])
            else:
                counts.append(0)
        mean = sum(counts)/len(counts)
        stddev = (sum((count - mean) ** 2 for count in counts)/(len(counts) - 1))**0.5
        means.append(mean)
        stddevs.append(stddev)
    means_frac = [mean/sum(means) for mean in means]
    p95_frac = [stddev * 1.96/sum(means)/20**0.5 for stddev in stddevs]
    occassional_error = [frac if i % 20 == 0 else 0 for (i, frac) in enumerate(p95_frac)]
    plt.errorbar(range(230), means_frac[:230], yerr=occassional_error[:230], label="Server " + str(server+1), zorder=server)

mean = 71/108 /(1 - 0.98)
rate = 1/mean
predictions = [rate * math.exp(-i * rate) for i in range(0, 230)]
plt.plot(range(230), predictions, linestyle=":", color="k", label="Limiting exponential", zorder=num_servers)

plt.xlabel("Number $x$")
plt.xlim((-10,230))
plt.ylabel("Queue length PMF: $P(q_j = x)$")
plt.legend()
plt.savefig(outname, bbox_inches='tight')

