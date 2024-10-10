import pandas as pd
from matplotlib import pyplot as plt

df: pd.DataFrame = pd.read_csv("results.csv")

df = df.groupby(["size", "k"]).mean().reset_index().drop("run", axis=1)

k1 = df[df["k"] == 1]
k2 = df[df["k"] == 2]
k3 = df[df["k"] == 3]
k4 = df[df["k"] == 4]
k5 = df[df["k"] == 5]
k6 = df[df["k"] == 6]

fig, axes = plt.subplots(2, 3, figsize=(20, 9))

img = k1.plot(
    x="size",
    y=[
        "random",
        "sorted",
        "alternated",
    ],
    ax=axes[0][0],
    title="Worse case (linear scale)",
)

img = k2.plot(
    x="size",
    y=[
        "random",
        "sorted",
        "alternated",
    ],
    ax=axes[0][1],
    title="Best case (linear scale)",
)

img = k3.plot(
    x="size",
    y=[
        "random",
        "sorted",
        "alternated",
    ],
    ax=axes[0][2],
    title="Average case (linear scale)",
)

img = k4.plot(
    x="size",
    y=[
        "random",
        "sorted",
        "alternated",
    ],
    ax=axes[1][0],
    title="Worse case (linear scale)",
)

img = k5.plot(
    x="size",
    y=[
        "random",
        "sorted",
        "alternated",
    ],
    ax=axes[1][1],
    title="Best case (linear scale)",
)

img = k6.plot(
    x="size",
    y=[
        "random",
        "sorted",
        "alternated",
    ],
    ax=axes[1][2],
    title="Average case (linear scale)",
)

plt.tight_layout()
fig.savefig("results.png")

fig, axes = plt.subplots(2, 3, figsize=(20, 9))

img = k1.plot(
    x="size",
    y=[
        "random",
        "sorted",
        "alternated",
    ],
    ax=axes[0][0],
    logy=True,
    logx=True,
    title="Worse case (linear scale)",
)

img = k2.plot(
    x="size",
    y=[
        "random",
        "sorted",
        "alternated",
    ],
    ax=axes[0][1],
    logy=True,
    logx=True,
    title="Best case (linear scale)",
)

img = k3.plot(
    x="size",
    y=[
        "random",
        "sorted",
        "alternated",
    ],
    ax=axes[0][2],
    logy=True,
    logx=True,
    title="Average case (linear scale)",
)

img = k4.plot(
    x="size",
    y=[
        "random",
        "sorted",
        "alternated",
    ],
    ax=axes[1][0],
    logy=True,
    logx=True,
    title="Worse case (linear scale)",
)

img = k5.plot(
    x="size",
    y=[
        "random",
        "sorted",
        "alternated",
    ],
    ax=axes[1][1],
    logy=True,
    logx=True,
    title="Best case (linear scale)",
)

img = k6.plot(
    x="size",
    y=[
        "random",
        "sorted",
        "alternated",
    ],
    ax=axes[1][2],
    logy=True,
    logx=True,
    title="Average case (linear scale)",
)

plt.tight_layout()
fig.savefig("results_two.png")
