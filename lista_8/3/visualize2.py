import pandas as pd
from matplotlib import pyplot as plt
import seaborn as sns

df = pd.read_csv("results.csv")
df.groupby(["cache", "size"]).mean().reset_index().drop("run", axis=1).to_csv(
    "results_grouped.csv", index=False, float_format="%.6f"
)

df_costs = df[["cache", "size", "run", "mark_cost", "lru_cost"]].copy()
df_costs = df_costs.melt(
    id_vars=["size", "cache", "run"], var_name="kind", value_name="value"
)
df_times = df[["cache", "size", "run", "mark_elapsed", "lru_elapsed"]].copy()
df_times = df_times.melt(
    id_vars=["size", "cache", "run"], var_name="kind", value_name="value"
)
fig = sns.relplot(
    data=df_costs,
    x="cache",
    y="value",
    col="size",
    hue="kind",
    kind="line",
    errorbar="ci",
    marker="o",
    legend="full",
)
plt.tight_layout()
plt.savefig("results1b.png")
plt.clf()

fig = sns.relplot(
    data=df_costs,
    x="cache",
    y="value",
    col="size",
    hue="kind",
    kind="line",
    errorbar="ci",
    marker="o",
    legend="full",
)
fig.set(xscale="log")
fig.set(yscale="log")

plt.tight_layout()
plt.savefig("results2b.png")
plt.clf()

fig = sns.relplot(
    data=df_times,
    x="cache",
    y="value",
    col="size",
    hue="kind",
    kind="line",
    errorbar="ci",
    marker="o",
    legend="full",
)
plt.tight_layout()
plt.savefig("results3b.png")
plt.clf()

fig = sns.relplot(
    data=df_times,
    x="cache",
    y="value",
    col="size",
    hue="kind",
    kind="line",
    errorbar="ci",
    marker="o",
    legend="full",
)
fig.set(xscale="log")
fig.set(yscale="log")

plt.tight_layout()
plt.savefig("results4b.png")
