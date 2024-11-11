import pandas as pd
from matplotlib import pyplot as plt
import seaborn as sns


df = pd.read_csv("results.csv")
df.groupby(["size", "struct"]).mean().reset_index().drop("run", axis=1).to_csv(
    "results_grouped.csv", index=False, float_format="%.6f"
)


df = df.melt(id_vars=["size", "struct", "run"], var_name="kind", value_name="value")

df_total = df[df["kind"] == "total"]
df_ops = df[df["kind"] != "total"]

df_insert = df_ops[df_ops["kind"] == "insert"]
df_ops = df_ops[df_ops["kind"] != "insert"]

fig = sns.relplot(
    data=df_total,
    x="size",
    y="value",
    hue="struct",
    kind="line",
    errorbar="ci",
    marker="o",
    legend="full",
    sizes=(0.25, 2.5),
)

sns.move_legend(fig, "upper right")

plt.tight_layout()
plt.savefig("results1.png")

fig = sns.relplot(
    data=df_total,
    x="size",
    y="value",
    hue="struct",
    kind="line",
    errorbar="ci",
    marker="o",
    legend="full",
    sizes=(0.25, 2.5),
)

sns.move_legend(fig, "upper right")

fig.set(xscale="log")
fig.set(yscale="log")

plt.tight_layout()
plt.savefig("results2.png")

#
fig = sns.relplot(
    data=df_insert,
    x="size",
    y="value",
    hue="struct",
    kind="line",
    errorbar="ci",
    marker="o",
    legend="full",
    sizes=(0.25, 2.5),
)

sns.move_legend(fig, "upper right")

plt.tight_layout()
plt.savefig("results3.png")

fig = sns.relplot(
    data=df_insert,
    x="size",
    y="value",
    hue="struct",
    kind="line",
    errorbar="ci",
    marker="o",
    legend="full",
    sizes=(0.25, 2.5),
)

sns.move_legend(fig, "upper right")

fig.set(xscale="log")
fig.set(yscale="log")

plt.tight_layout()
plt.savefig("results4.png")


#

fig = sns.relplot(
    data=df_ops,
    x="size",
    y="value",
    col="kind",
    col_wrap=3,
    hue="struct",
    kind="line",
    errorbar="ci",
    marker="o",
    legend="full",
    sizes=(0.25, 2.5),
)

sns.move_legend(fig, "upper right")

plt.tight_layout()
plt.savefig("results5.png")

fig = sns.relplot(
    data=df_ops,
    x="size",
    y="value",
    col="kind",
    col_wrap=3,
    hue="struct",
    kind="line",
    errorbar="ci",
    marker="o",
    legend="full",
    sizes=(0.25, 2.5),
)

sns.move_legend(fig, "upper right")

fig.set(xscale="log")
fig.set(yscale="log")

plt.tight_layout()
plt.savefig("results6.png")
