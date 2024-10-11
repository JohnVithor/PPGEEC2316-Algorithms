import pandas as pd
from matplotlib import pyplot as plt
import numpy as np


def strassen_theoretical(n: int) -> int:
    if n == 1:
        return 6
    return 7 * strassen_theoretical(n / 2) + 238 * (n / 2) * (n / 2) + 37


df: pd.DataFrame = pd.read_csv("results.csv")

df["strassen_theoretical"] = df["size"].apply(strassen_theoretical)
df["strassen_O(n^2.81)"] = df["size"].apply(lambda x: x**2.81)

df["strassen_theoretical"] = (
    df["strassen_theoretical"] - df["strassen_theoretical"].min()
) / (df["strassen_theoretical"].max() - df["strassen_theoretical"].min())

df["strassen_O(n^2.81)"] = (
    df["strassen_O(n^2.81)"] - df["strassen_O(n^2.81)"].min()
) / (df["strassen_O(n^2.81)"].max() - df["strassen_O(n^2.81)"].min())

df["strassen"] = (df["strassen"] - df["strassen"].min()) / (
    df["strassen"].max() - df["strassen"].min()
)

fig, axes = plt.subplots(3, 2, figsize=(12, 9))


img = df.plot(
    x="size",
    y=["strassen_theoretical", "strassen", "strassen_O(n^2.81)"],
    style=[":s", "--o", "-x"],
    ax=axes[0][0],
    title="linear scale",
)

img = df.plot(
    x="size",
    y=["strassen_theoretical", "strassen", "strassen_O(n^2.81)"],
    style=[":s", "--o", "-x"],
    ax=axes[0][1],
    logy=True,
    logx=True,
    title="log scale",
)

cutpoint = 128

img = df[df["size"] <= cutpoint].plot(
    x="size",
    y=["strassen_theoretical", "strassen", "strassen_O(n^2.81)"],
    style=[":s", "--o", "-x"],
    ax=axes[1][0],
    title="linear scale",
)

img = df[df["size"] <= cutpoint].plot(
    x="size",
    y=["strassen_theoretical", "strassen", "strassen_O(n^2.81)"],
    style=[":s", "--o", "-x"],
    ax=axes[1][1],
    logy=True,
    logx=True,
    title="log scale",
)

cutpoint = 4096

img = df[df["size"] >= cutpoint].plot(
    x="size",
    y=["strassen_theoretical", "strassen", "strassen_O(n^2.81)"],
    style=[":s", "--o", "-x"],
    ax=axes[2][0],
    title="linear scale",
)

img = df[df["size"] >= cutpoint].plot(
    x="size",
    y=["strassen_theoretical", "strassen", "strassen_O(n^2.81)"],
    style=[":s", "--o", "-x"],
    ax=axes[2][1],
    logy=True,
    logx=True,
    title="log scale",
)


plt.tight_layout()
fig.savefig("results.png")
