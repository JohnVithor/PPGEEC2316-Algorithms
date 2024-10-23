import pandas as pd
from matplotlib import pyplot as plt
import seaborn as sns

df_c: pd.DataFrame = pd.read_csv("results_c_2.csv")
df_c["language"] = "C"
df_r: pd.DataFrame = pd.read_csv("results_r_2.csv")
df_r["language"] = "Rust"

df_r["version"] = df_r["version"].str.replace("count sort", "count_sort")
df_r["version"] = df_r["version"].str.replace("radix sort", "radix_sort")

df = pd.concat([df_c, df_r])
df.groupby(["size", "language", "version"]).mean().reset_index().drop(
    "run", axis=1
).to_csv("results_grouped_1.csv", index=False, float_format="%.6f")
df = df.melt(
    id_vars=["size", "language", "version", "run"], var_name="kind", value_name="value"
)

fig = sns.relplot(
    data=df,
    x="size",
    y="value",
    col="kind",
    hue="language",
    style="version",
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
    data=df,
    x="size",
    y="value",
    col="kind",
    hue="language",
    style="version",
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
