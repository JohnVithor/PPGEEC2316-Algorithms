import pandas as pd
from matplotlib import pyplot as plt

df: pd.DataFrame = pd.read_csv("results.csv")
df["time"] = (df["time"] - df["time"].min()) / (df["time"].max() - df["time"].min())
df["ops"] = (df["ops"] - df["ops"].min()) / (df["ops"].max() - df["ops"].min())
df = df.groupby(["size"]).mean().reset_index()

fig, axes = plt.subplots(3, 2, figsize=(12, 9))


img = df.plot(
    x="size",
    y=["time", "ops"],
    ax=axes[0][0],
    title="Real vs RAM (linear scale)",
)

img = df.plot(
    x="size",
    y=["time", "ops"],
    ax=axes[0][1],
    logy=True,
    logx=True,
    title="Real vs RAM (log scale)",
)

cutpoint = 1000

img = df[df["size"] <= cutpoint].plot(
    x="size",
    y=["time", "ops"],
    ax=axes[1][0],
    title=f"Real vs RAM (linear scale) size <= {cutpoint}",
)

img = df[df["size"] <= cutpoint].plot(
    x="size",
    y=["time", "ops"],
    ax=axes[1][1],
    logy=True,
    logx=True,
    title=f"Real vs RAM (log scale) size <= {cutpoint}",
)

cutpoint = 80000

img = df[df["size"] >= cutpoint].plot(
    x="size",
    y=["time", "ops"],
    ax=axes[2][0],
    title=f"Real vs RAM (linear scale) size >= {cutpoint}",
)

img = df[df["size"] >= cutpoint].plot(
    x="size",
    y=["time", "ops"],
    ax=axes[2][1],
    logy=True,
    logx=True,
    title=f"Real vs RAM (log scale) size >= {cutpoint}",
)

plt.tight_layout()
fig.savefig("results.png")
