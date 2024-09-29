import pandas as pd
from matplotlib import pyplot as plt

df: pd.DataFrame = pd.read_csv("results.csv")
df = df.groupby(["size"]).mean().reset_index()

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
        "insertion_avg",
        "merge_avg",
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
        "insertion_avg",
        "merge_avg",
    ],
    ax=axes[1][2],
    logy=True,
    logx=True,
    title="Average case (log scale)",
)

img = df[(df["size"] >= 100) & (df["size"] <= 300)].plot(
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

img = df[(df["size"] >= 400) & (df["size"] <= 800)].plot(
    x="size",
    y=[
        "insertion_avg",
        "merge_avg",
    ],
    ax=axes[2][2],
    title="Average case (zoom on interest area)",
)

plt.tight_layout()
fig.savefig("results.png")
