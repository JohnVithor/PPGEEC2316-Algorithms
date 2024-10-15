import pandas as pd
from matplotlib import pyplot as plt
import seaborn as sns

df_c: pd.DataFrame = pd.read_csv("results_c.csv")
df_c["language"] = "C"
df_r: pd.DataFrame = pd.read_csv("results_r.csv")
df_r["language"] = "Rust"

df = pd.concat([df_c, df_r])
df = df.groupby(["size", "language"]).mean().reset_index().drop("i", axis=1)
df = df.melt(id_vars=["size", "language"], var_name="kind", value_name="value")

fig, axes = plt.subplots(2, 2, figsize=(12, 9))

sns.lineplot(
    data=df,
    x="size",
    y="value",
    hue="language",
    style="kind",
    sizes=(0.25, 2.5),
    ax=axes[0][0],
)

r = sns.lineplot(
    data=df,
    x="size",
    y="value",
    hue="language",
    style="kind",
    sizes=(0.25, 2.5),
    ax=axes[0][1],
)
r.set(xscale="log")
r.set(yscale="log")

cutpoint1 = 1024
cutpoint2 = 2048

df_zoom = df[(df["size"] >= cutpoint1) & (df["size"] <= cutpoint2)]

sns.lineplot(
    data=df_zoom,
    x="size",
    y="value",
    hue="language",
    style="kind",
    sizes=(0.25, 2.5),
    ax=axes[1][0],
)

r = sns.lineplot(
    data=df_zoom,
    x="size",
    y="value",
    hue="language",
    style="kind",
    sizes=(0.25, 2.5),
    ax=axes[1][1],
)
r.set(xscale="log")
r.set(yscale="log")
plt.show()

# img = df[(df["size"] >= cutpoint1) & (df["size"] <= cutpoint2)].plot(
#     x="size",
#     y=["classic", "strassen", "transposed"],
#     style=[":s", "--o", "-."],
#     ax=axes[1][0],
#     title=f"Classic vs Strassen(linear scale) swap point",
# )

# img = df[(df["size"] >= cutpoint1) & (df["size"] <= cutpoint2)].plot(
#     x="size",
#     y=["classic", "strassen", "transposed"],
#     style=[":s", "--o", "-."],
#     ax=axes[1][1],
#     logy=True,
#     logx=True,
#     title=f"Classic vs Strassen(log scale) swap point",
# )

# plt.tight_layout()
# fig.savefig("results.png")
