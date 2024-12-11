import pandas as pd
from matplotlib import pyplot as plt
import seaborn as sns

df = pd.read_csv("results.csv")
df.groupby(["size"]).mean().reset_index().drop("run", axis=1).to_csv(
    "results_grouped.csv", index=False, float_format="%.6f"
)
df_costs = df[["size", "run", "mtf_cost", "foresee_cost"]]
df_costs.loc[:, "Competitivity"] = df_costs["mtf_cost"] / df_costs["foresee_cost"]

df_times = df[["size", "run", "mtf_time", "foresee_time"]]
df_times.loc[:, "Competitivity"] = df_times["mtf_time"] / df_times["foresee_time"]


fig = sns.lineplot(data=df_costs, x="size", y="Competitivity", marker="o")
plt.tight_layout()
plt.savefig("results1.png")
plt.clf()

fig = sns.lineplot(data=df_costs, x="size", y="Competitivity", marker="o")
fig.set(xscale="log")
fig.set(yscale="log")

plt.tight_layout()
plt.savefig("results2.png")
plt.clf()

fig = sns.lineplot(data=df_times, x="size", y="Competitivity", marker="o")
plt.tight_layout()
plt.savefig("results3.png")
plt.clf()

fig = sns.lineplot(data=df_times, x="size", y="Competitivity", marker="o")
fig.set(xscale="log")
fig.set(yscale="log")

plt.tight_layout()
plt.savefig("results4.png")
