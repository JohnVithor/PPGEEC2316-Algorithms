import pandas as pd
from matplotlib import pyplot as plt
import seaborn as sns

df_c: pd.DataFrame = pd.read_csv("results_c.csv")
df_c["language"] = "C"
df_r: pd.DataFrame = pd.read_csv("results_r.csv")
df_r["language"] = "Rust"

df = pd.concat([df_c, df_r])
df.groupby(["size", "language", "version"]).mean().reset_index().drop(
    "run", axis=1
).to_csv("results_grouped.csv", index=False, float_format="%.6f")
df = df.melt(
    id_vars=["size", "language", "version", "run"], var_name="kind", value_name="value"
)


# fig, axes = plt.subplots(2, 2, figsize=(12, 9))

sns.relplot(
    data=df,
    x="size",
    y="value",
    col="kind",
    hue="language",
    style="version",
    kind="line",
    errorbar="ci",
    marker="o",
    sizes=(0.25, 2.5),
)

# sns.lineplot(
#     data=df,
#     x="size",
#     y="value",
#     hue="language",
#     style="kind",
#     errorbar="ci",
#     marker="o",
#     sizes=(0.25, 2.5),
#     ax=axes[0][0],
# )

# r = sns.lineplot(
#     data=df,
#     x="size",
#     y="value",
#     hue="language",
#     style="kind",
#     errorbar="ci",
#     marker="o",
#     sizes=(0.25, 2.5),
#     ax=axes[0][1],
# )
# r.set(xscale="log")
# r.set(yscale="log")

# df_random = df[(df["kind"] == "random")]

# sns.lineplot(
#     data=df_random,
#     x="size",
#     y="value",
#     hue="language",
#     style="kind",
#     errorbar="ci",
#     marker="o",
#     sizes=(0.25, 2.5),
#     ax=axes[1][0],
# )

# r = sns.lineplot(
#     data=df_random,
#     x="size",
#     y="value",
#     hue="language",
#     style="kind",
#     errorbar="ci",
#     marker="o",
#     sizes=(0.25, 2.5),
#     ax=axes[1][1],
# )
# r.set(xscale="log")
# r.set(yscale="log")

plt.tight_layout()
plt.savefig("results.png")
