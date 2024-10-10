import pandas as pd
from matplotlib import pyplot as plt

df: pd.DataFrame = pd.read_csv("results.csv")

df_time = df[df["version"] == "insertion_sort"].drop("version", axis=1)
df_ops = df[df["version"] == "merge_sort"].drop("version", axis=1)

# df_time["random"] = (df_time["random"] - df_time["random"].min()) / (
#     df_time["random"].max() - df_time["random"].min()
# )
# df_time["best"] = (df_time["best"] - df_time["best"].min()) / (
#     df_time["best"].max() - df_time["best"].min()
# )
# df_time["worse"] = (df_time["worse"] - df_time["worse"].min()) / (
#     df_time["worse"].max() - df_time["worse"].min()
# )

# df_ops["random"] = (df_ops["random"] - df_ops["random"].min()) / (
#     df_ops["random"].max() - df_ops["random"].min()
# )
# df_ops["best"] = (df_ops["best"] - df_ops["best"].min()) / (
#     df_ops["best"].max() - df_ops["best"].min()
# )
# df_ops["worse"] = (df_ops["worse"] - df_ops["worse"].min()) / (
#     df_ops["worse"].max() - df_ops["worse"].min()
# )

df_time = df_time.groupby(["size"]).mean().reset_index().drop("run", axis=1)
df_ops = df_ops.groupby(["size"]).mean().reset_index().drop("run", axis=1)

df_time.columns = ["size", "insertion_random", "insertion_best", "insertion_worse"]
df_ops.columns = ["size", "merge_random", "merge_best", "merge_worse"]

df = pd.concat([df_time, df_ops.drop("size", axis=1)], axis=1)

fig, axes = plt.subplots(3, 3, figsize=(20, 9))

img = df.plot(
    x="size",
    y=[
        "insertion_worse",
        "merge_worse",
    ],
    ax=axes[0][0],
    title="Worse case (linear scale)",
)

img = df.plot(
    x="size",
    y=[
        "insertion_best",
        "merge_best",
    ],
    ax=axes[0][1],
    title="Best case (linear scale)",
)

img = df.plot(
    x="size",
    y=[
        "insertion_random",
        "merge_random",
    ],
    ax=axes[0][2],
    title="Average case (linear scale)",
)

img = df.plot(
    x="size",
    y=[
        "insertion_worse",
        "merge_worse",
    ],
    ax=axes[1][0],
    logy=True,
    logx=True,
    title="Worse case (log scale)",
)

img = df.plot(
    x="size",
    y=[
        "insertion_best",
        "merge_best",
    ],
    ax=axes[1][1],
    logy=True,
    logx=True,
    title="Best case (log scale)",
)

img = df.plot(
    x="size",
    y=[
        "insertion_random",
        "merge_random",
    ],
    ax=axes[1][2],
    logy=True,
    logx=True,
    title="Average case (log scale)",
)

img = df[(df["size"] >= 50) & (df["size"] <= 300)].plot(
    x="size",
    y=[
        "insertion_worse",
        "merge_worse",
    ],
    ax=axes[2][0],
    title="Worse case (zoom on interest area)",
)

img = df[df["size"] <= 500].plot(
    x="size",
    y=[
        "insertion_best",
        "merge_best",
    ],
    ax=axes[2][1],
    title="Best case (zoom on interest area)",
)

img = df[(df["size"] >= 100) & (df["size"] <= 500)].plot(
    x="size",
    y=[
        "insertion_random",
        "merge_random",
    ],
    ax=axes[2][2],
    title="Average case (zoom on interest area)",
)

plt.tight_layout()
fig.savefig("results.png")
