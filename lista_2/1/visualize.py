import pandas as pd
from matplotlib import pyplot as plt
import seaborn as sns

df_c: pd.DataFrame = pd.read_csv("results_c.csv")
df_c["language"] = "C"
df_r: pd.DataFrame = pd.read_csv("results_r.csv")
df_r["language"] = "Rust"

df = pd.concat([df_c, df_r])
# df = df.groupby(["size", "language"]).mean().reset_index().drop("i", axis=1)
df.groupby(["size", "language"]).mean().reset_index().drop("i", axis=1).to_csv(
    "results_grouped.csv", index=False, float_format="%.6f"
)
df = df.melt(id_vars=["size", "language", "i"], var_name="kind", value_name="value")


fig, axes = plt.subplots(2, 2, figsize=(12, 9))

sns.lineplot(
    data=df,
    x="size",
    y="value",
    hue="language",
    style="kind",
    errorbar="ci",
    marker="o",
    sizes=(0.25, 2.5),
    ax=axes[0][0],
)

r = sns.lineplot(
    data=df,
    x="size",
    y="value",
    hue="language",
    style="kind",
    errorbar="ci",
    marker="o",
    sizes=(0.25, 2.5),
    ax=axes[0][1],
)
r.set(xscale="log")
r.set(yscale="log")

cutpoint1 = 512
cutpoint2 = 2048

df_zoom = df[(df["size"] >= cutpoint1) & (df["size"] <= cutpoint2)]

sns.lineplot(
    data=df_zoom,
    x="size",
    y="value",
    hue="language",
    style="kind",
    errorbar="ci",
    marker="o",
    sizes=(0.25, 2.5),
    ax=axes[1][0],
)

r = sns.lineplot(
    data=df_zoom,
    x="size",
    y="value",
    hue="language",
    style="kind",
    errorbar="ci",
    marker="o",
    sizes=(0.25, 2.5),
    ax=axes[1][1],
)
r.set(xscale="log")
r.set(yscale="log")

plt.tight_layout()
fig.savefig("results.png")
