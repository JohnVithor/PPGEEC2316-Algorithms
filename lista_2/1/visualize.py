import pandas as pd
from matplotlib import pyplot as plt
import numpy as np

df: pd.DataFrame = pd.read_csv("results.csv")

fig, axes = plt.subplots(2, 2, figsize=(12, 9))


img = df.plot(
    x="size",
    y=["classic", "strassen"],
    ax=axes[0][0],
    style=[":s", "--o"],
    title="Classic vs Strassen (linear scale)",
)

img = df.plot(
    x="size",
    y=["classic", "strassen"],
    ax=axes[0][1],
    style=[":s", "--o"],
    logy=True,
    logx=True,
    title="Classic vs Strassen(log scale)",
)

cutpoint1 = 1024
cutpoint2 = 2048

img = df[(df["size"] >= cutpoint1) & (df["size"] <= cutpoint2)].plot(
    x="size",
    y=["classic", "strassen"],
    style=[":s", "--o"],
    ax=axes[1][0],
    title=f"Classic vs Strassen(linear scale) size <= {cutpoint1}",
)

img = df[(df["size"] >= cutpoint1) & (df["size"] <= cutpoint2)].plot(
    x="size",
    y=["classic", "strassen"],
    style=[":s", "--o"],
    ax=axes[1][1],
    logy=True,
    logx=True,
    title=f"Classic vs Strassen(log scale) size <= {cutpoint1}",
)

plt.tight_layout()
fig.savefig("results.png")
