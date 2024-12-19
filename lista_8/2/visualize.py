import pandas as pd
from matplotlib import pyplot as plt
import seaborn as sns

df = pd.read_csv("results.csv")
df.groupby(["cache", "sequence"]).mean().reset_index().drop("run", axis=1).to_csv(
    "results_grouped.csv", index=False, float_format="%.6f"
)
df_costs = df[["cache", "sequence", "run", "oracle_cost", "lru_cost"]].copy()
df_costs.loc[:, "Competitivity"] = df_costs["lru_cost"] / df_costs["oracle_cost"]

df_times = df[["cache", "sequence", "run", "oracle_time", "lru_time"]].copy()
df_times.loc[:, "Competitivity"] = df_times["lru_time"] / df_times["oracle_time"]


fig = sns.relplot(
    data=df_costs,
    x="sequence",
    y="Competitivity",
    hue="cache",
    kind="line",
    errorbar="ci",
    marker="o",
    legend="full",
)
plt.tight_layout()
plt.savefig("results1.png")
plt.clf()

fig = sns.relplot(
    data=df_costs,
    x="sequence",
    y="Competitivity",
    hue="cache",
    kind="line",
    errorbar="ci",
    marker="o",
    legend="full",
)
fig.set(xscale="log")
fig.set(yscale="log")

plt.tight_layout()
plt.savefig("results2.png")
plt.clf()

fig = sns.relplot(
    data=df_times,
    x="sequence",
    y="Competitivity",
    hue="cache",
    kind="line",
    errorbar="ci",
    marker="o",
    legend="full",
)
plt.tight_layout()
plt.savefig("results3.png")
plt.clf()

fig = sns.relplot(
    data=df_times,
    x="sequence",
    y="Competitivity",
    hue="cache",
    kind="line",
    errorbar="ci",
    marker="o",
    legend="full",
)
fig.set(xscale="log")
fig.set(yscale="log")

plt.tight_layout()
plt.savefig("results4.png")
