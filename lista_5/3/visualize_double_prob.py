import pandas as pd
from matplotlib import pyplot as plt
import seaborn as sns


df = pd.read_csv("results.csv")
df.groupby(["size", "mod", "hash"]).mean().reset_index().drop("run", axis=1).to_csv(
    "results_grouped.csv", index=False, float_format="%.6f"
)

df = df[
    (df["hash"] == "linear_double_hashing_table")
    | (df["hash"] == "linear_probing_table")
]

df = df.melt(
    id_vars=["size", "mod", "run", "hash"], var_name="kind", value_name="value"
)

fig = sns.relplot(
    data=df,
    x="size",
    y="value",
    col="kind",
    col_wrap=2,
    hue="hash",
    kind="line",
    errorbar="ci",
    marker="o",
    legend="full",
    size="mod",
    sizes=(0.25, 2.5),
)

sns.move_legend(fig, "upper right")

plt.tight_layout()
plt.savefig("results1.png")

fig = sns.relplot(
    data=df,
    x="size",
    y="value",
    col="kind",
    col_wrap=2,
    hue="hash",
    kind="line",
    errorbar="ci",
    marker="o",
    legend="full",
    size="mod",
    sizes=(0.25, 2.5),
)

sns.move_legend(fig, "upper right")

fig.set(xscale="log")
fig.set(yscale="log")

plt.tight_layout()
plt.savefig("results2.png")
