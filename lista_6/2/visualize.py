import pandas as pd
from matplotlib import pyplot as plt
import seaborn as sns


df = pd.read_csv("results_1000.csv")
df.groupby(["nodes", "edges", "modifier"]).mean().reset_index().drop(
    "run", axis=1
).to_csv("results_grouped.csv", index=False, float_format="%.6f")

df.drop(["kruskal_cost", "prim_cost"], axis=1, inplace=True)

df = df.melt(
    id_vars=["nodes", "edges", "modifier", "run"], var_name="kind", value_name="value"
)

df_adj = df[df["modifier"].str.contains("adjacency")]
df_edge = df[df["modifier"].str.contains("edge")]

fig = sns.relplot(
    data=df_adj,
    x="nodes",
    y="value",
    hue="kind",
    kind="line",
    errorbar="ci",
    marker="o",
    legend="full",
    sizes=(0.25, 2.5),
    size="modifier",
)

sns.move_legend(fig, "upper right")

plt.tight_layout()
plt.savefig("results1.png")

fig = sns.relplot(
    data=df_adj,
    x="nodes",
    y="value",
    hue="kind",
    kind="line",
    errorbar="ci",
    marker="o",
    legend="full",
    sizes=(0.25, 2.5),
    size="modifier",
)

sns.move_legend(fig, "upper right")

fig.set(xscale="log")
fig.set(yscale="log")

plt.tight_layout()
plt.savefig("results2.png")

##

fig = sns.relplot(
    data=df_edge,
    x="nodes",
    y="value",
    hue="kind",
    kind="line",
    errorbar="ci",
    marker="o",
    legend="full",
    sizes=(0.25, 2.5),
    size="modifier",
)

sns.move_legend(fig, "upper right")

plt.tight_layout()
plt.savefig("results3.png")

fig = sns.relplot(
    data=df_edge,
    x="nodes",
    y="value",
    hue="kind",
    kind="line",
    errorbar="ci",
    marker="o",
    legend="full",
    sizes=(0.25, 2.5),
    size="modifier",
)

sns.move_legend(fig, "upper right")

fig.set(xscale="log")
fig.set(yscale="log")

plt.tight_layout()
plt.savefig("results4.png")
