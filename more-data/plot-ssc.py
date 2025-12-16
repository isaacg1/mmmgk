import sys
import matplotlib.pyplot as plt
filename = sys.argv[1]
outname = sys.argv[2]
assert filename.endswith(".csv")
assert outname.endswith(".eps")

datass = [[], []]
state = None
with open(filename) as file:
    for line in file:
        line = line.strip()
        if not line:
            continue
        if line[0] == 'm':
            assert line[7] in ['0', '8']
            state = ['0', '8'].index(line[7])
            datass[state].append([])
        if line[0] in ['0', '1']:
            datass[state][-1].append([float(i) for i in line.split(';') if i])

alphas = [row[0] for row in datass[0][0]]
inv_alphas = [1/alpha for alpha in alphas]
for state in [0, 1]:
    if state == 0:
        k_star_num = 7/12
    if state == 1:
        k_star_num = 169/12
    for col_group in [0, 1]:
        largest_stddev = 0
        plt.figure(figsize=(5, 3))
        plt.xscale('log')
        if state == 1 or col_group == 0:
            plt.yscale('log')
        for server in [0, 1, 2]:
            col = server + 3 * col_group + 1
            means = []
            p95s = []
            for alpha_i in range(len(alphas)):
                alpha = alphas[alpha_i]
                results = []
                for seed in range(len(datass[state])):
                    try:
                        results.append(datass[state][seed][alpha_i][col])
                    except:
                        pass
                mean = sum(results)/len(results)
                stddev = (sum((result - mean) ** 2 for result in results)/(len(results) - 1)) ** 0.5
                p95 = 1.96*stddev
                means.append(mean)
                p95s.append(p95)
                if stddev > largest_stddev:
                    largest_stddev = stddev
                    print(largest_stddev)
            plt.errorbar(inv_alphas, means, yerr=p95s, label="Server "+str(server+1), zorder=server)
        if col_group == 0:
            predictions = [1/(1-0.95) * (1 + k_star_num * inv_alpha/6)/3 for inv_alpha in inv_alphas]
            plt.plot(inv_alphas, predictions, linestyle=":", color="k", label="Predicted heavy traffic mean", zorder=3)
        plt.xlabel("Mean modulation duration $1/\\alpha$")
        plt.legend()
        if col_group == 0:
            plt.ylabel("Mean queue length $E[q_i]$")
            extra_name = "mean"
        else:
            plt.ylabel("Mean absolute gap $E[|q_j - q_\\Sigma/n|]$")
            extra_name = "gap"
        name_chunks = outname.split(".")
        new_name = ("-" + extra_name + "-" + str(state) + ".").join(name_chunks)
        plt.savefig(new_name, bbox_inches="tight")
        plt.close()


