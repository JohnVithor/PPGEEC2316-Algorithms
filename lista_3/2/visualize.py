import pandas as pd
from matplotlib import pyplot as plt

df: pd.DataFrame = pd.read_csv("results.csv")

df_time = df[df["version"] == "quick_sort"].drop("version", axis=1)
df_ops = df[df["version"] == "randomized_quick_sort"].drop("version", axis=1)

df_time = df_time.groupby(["size"]).mean().reset_index().drop("run", axis=1)
df_ops = df_ops.groupby(["size"]).mean().reset_index().drop("run", axis=1)

df_time.columns = ["size", "quick_random", "quick_best", "quick_worse"]
df_ops.columns = [
    "size",
    "randomized_quick_random",
    "randomized_quick_best",
    "randomized_quick_worse",
]

df = pd.concat([df_time, df_ops.drop("size", axis=1)], axis=1)

fig, axes = plt.subplots(3, 3, figsize=(20, 9))

img = df.plot(
    x="size",
    y=[
        "quick_worse",
        "randomized_quick_worse",
    ],
    ax=axes[0][0],
    title="Worse case (linear scale)",
)

img = df.plot(
    x="size",
    y=[
        "quick_best",
        "randomized_quick_best",
    ],
    ax=axes[0][1],
    title="Best case (linear scale)",
)

img = df.plot(
    x="size",
    y=[
        "quick_random",
        "randomized_quick_random",
    ],
    ax=axes[0][2],
    title="Average case (linear scale)",
)

img = df.plot(
    x="size",
    y=[
        "quick_worse",
        "randomized_quick_worse",
    ],
    ax=axes[1][0],
    logy=True,
    logx=True,
    title="Worse case (log scale)",
)

img = df.plot(
    x="size",
    y=[
        "quick_best",
        "randomized_quick_best",
    ],
    ax=axes[1][1],
    logy=True,
    logx=True,
    title="Best case (log scale)",
)

img = df.plot(
    x="size",
    y=[
        "quick_random",
        "randomized_quick_random",
    ],
    ax=axes[1][2],
    logy=True,
    logx=True,
    title="Average case (log scale)",
)

img = df[(df["size"] >= 50) & (df["size"] <= 300)].plot(
    x="size",
    y=[
        "quick_worse",
        "randomized_quick_worse",
    ],
    ax=axes[2][0],
    title="Worse case (zoom on interest area)",
)

img = df[df["size"] <= 500].plot(
    x="size",
    y=[
        "quick_best",
        "randomized_quick_best",
    ],
    ax=axes[2][1],
    title="Best case (zoom on interest area)",
)

img = df[(df["size"] >= 100) & (df["size"] <= 500)].plot(
    x="size",
    y=[
        "quick_random",
        "randomized_quick_random",
    ],
    ax=axes[2][2],
    title="Average case (zoom on interest area)",
)

plt.tight_layout()
fig.savefig("results.png")
