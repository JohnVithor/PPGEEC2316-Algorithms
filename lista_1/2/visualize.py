import pandas as pd
from matplotlib import pyplot as plt

df: pd.DataFrame = pd.read_csv("results.csv")

df_time = df[df["version"] == "insertion_sort"].drop("version", axis=1)
df_ops = df[df["version"] == "insertion_sort_ram"].drop("version", axis=1)

df_time["random"] = (df_time["random"] - df_time["random"].min()) / (
    df_time["random"].max() - df_time["random"].min()
)
df_time["best"] = (df_time["best"] - df_time["best"].min()) / (
    df_time["best"].max() - df_time["best"].min()
)
df_time["worse"] = (df_time["worse"] - df_time["worse"].min()) / (
    df_time["worse"].max() - df_time["worse"].min()
)

df_ops["random"] = (df_ops["random"] - df_ops["random"].min()) / (
    df_ops["random"].max() - df_ops["random"].min()
)
df_ops["best"] = (df_ops["best"] - df_ops["best"].min()) / (
    df_ops["best"].max() - df_ops["best"].min()
)
df_ops["worse"] = (df_ops["worse"] - df_ops["worse"].min()) / (
    df_ops["worse"].max() - df_ops["worse"].min()
)

df_time = df_time.groupby(["size"]).mean().reset_index().drop("run", axis=1)
df_ops = df_ops.groupby(["size"]).mean().reset_index().drop("run", axis=1)

df_time.columns = ["size", "time_random", "time_best", "time_worse"]
df_ops.columns = ["size", "ops_random", "ops_best", "ops_worse"]

df = pd.concat([df_time, df_ops.drop("size", axis=1)], axis=1)


fig, axes = plt.subplots(2, 2, figsize=(12, 9))


img = df.plot(
    x="size",
    y=["time_random", "ops_random"],
    ax=axes[0][0],
    title="Real vs RAM (linear scale)",
)

img = df.plot(
    x="size",
    y=["time_random", "ops_random"],
    ax=axes[0][1],
    logy=True,
    logx=True,
    title="Real vs RAM (log scale)",
)

cutpoint = 1000

img = df[df["size"] <= cutpoint].plot(
    x="size",
    y=["time_random", "ops_random"],
    ax=axes[1][0],
    title=f"Real vs RAM (linear scale) size <= {cutpoint}",
)

img = df[df["size"] <= cutpoint].plot(
    x="size",
    y=["time_random", "ops_random"],
    ax=axes[1][1],
    logy=True,
    logx=True,
    title=f"Real vs RAM (log scale) size <= {cutpoint}",
)

plt.tight_layout()
fig.savefig("results.png")
