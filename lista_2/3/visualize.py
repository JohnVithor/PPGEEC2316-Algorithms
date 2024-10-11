import pandas as pd
from matplotlib import pyplot as plt
import seaborn as sns

df: pd.DataFrame = pd.read_csv("results.csv")

df = df.groupby(["size", "k"]).mean().reset_index().drop("run", axis=1)

df = df.melt(id_vars=["size", "k"], var_name="order", value_name="value")
# df["random"] = (df["random"] - df["random"].min()) / (
#     df["random"].max() - df["random"].min()
# )
# df["sorted"] = (df["sorted"] - df["sorted"].min()) / (
#     df["sorted"].max() - df["sorted"].min()
# )
# df["alternated"] = (df["alternated"] - df["alternated"].min()) / (
#     df["alternated"].max() - df["alternated"].min()
# )
sns.set_theme(font_scale=1.5)

sns.relplot(
    data=df,
    x="size",
    y="value",
    col="k",
    hue="order",
    kind="line",
    col_wrap=4,
)
plt.savefig("results_linear.png")

sns.relplot(
    data=df,
    x="size",
    y="value",
    col="k",
    hue="order",
    kind="line",
    col_wrap=4,
)
plt.xscale("log")
plt.yscale("log")
plt.savefig("results_log.png")


# fig, axes = plt.subplots(2, 5, figsize=(20, 9))

# img = k1.plot(
#     x="size",
#     y=[
#         "random",
#         "sorted",
#         "alternated",
#     ],
#     ax=axes[0][0],
#     title="K = 1/10",
# )

# img = k2.plot(
#     x="size",
#     y=[
#         "random",
#         "sorted",
#         "alternated",
#     ],
#     ax=axes[0][1],
#     title="K = 2/10",
# )

# img = k3.plot(
#     x="size",
#     y=[
#         "random",
#         "sorted",
#         "alternated",
#     ],
#     ax=axes[0][2],
#     title="K = 3/10",
# )

# img = k4.plot(
#     x="size",
#     y=[
#         "random",
#         "sorted",
#         "alternated",
#     ],
#     ax=axes[0][3],
#     title="K = 4/10",
# )

# img = k5.plot(
#     x="size",
#     y=[
#         "random",
#         "sorted",
#         "alternated",
#     ],
#     ax=axes[0][4],
#     title="K = 5/10",
# )

# img = k6.plot(
#     x="size",
#     y=[
#         "random",
#         "sorted",
#         "alternated",
#     ],
#     ax=axes[1][0],
#     title="K = 6/10",
# )

# img = k7.plot(
#     x="size",
#     y=[
#         "random",
#         "sorted",
#         "alternated",
#     ],
#     ax=axes[1][1],
#     title="K = 7/10",
# )

# img = k8.plot(
#     x="size",
#     y=[
#         "random",
#         "sorted",
#         "alternated",
#     ],
#     ax=axes[1][2],
#     title="K = 8/10",
# )

# img = k9.plot(
#     x="size",
#     y=[
#         "random",
#         "sorted",
#         "alternated",
#     ],
#     ax=axes[1][3],
#     title="K = 9/10",
# )

# img = k10.plot(
#     x="size",
#     y=[
#         "random",
#         "sorted",
#         "alternated",
#     ],
#     ax=axes[1][4],
#     title="K = 10/10",
# )

# plt.tight_layout()
# fig.savefig("results.png")

# fig, axes = plt.subplots(2, 5, figsize=(20, 9))

# img = k1.plot(
#     x="size",
#     y=[
#         "random",
#         "sorted",
#         "alternated",
#     ],
#     logy=True,
#     logx=True,
#     ax=axes[0][0],
#     title="K = 1/10",
# )

# img = k2.plot(
#     x="size",
#     y=[
#         "random",
#         "sorted",
#         "alternated",
#     ],
#     logy=True,
#     logx=True,
#     ax=axes[0][1],
#     title="K = 2/10",
# )

# img = k3.plot(
#     x="size",
#     y=[
#         "random",
#         "sorted",
#         "alternated",
#     ],
#     logy=True,
#     logx=True,
#     ax=axes[0][2],
#     title="K = 3/10",
# )

# img = k4.plot(
#     x="size",
#     y=[
#         "random",
#         "sorted",
#         "alternated",
#     ],
#     logy=True,
#     logx=True,
#     ax=axes[0][3],
#     title="K = 4/10",
# )

# img = k5.plot(
#     x="size",
#     y=[
#         "random",
#         "sorted",
#         "alternated",
#     ],
#     logy=True,
#     logx=True,
#     ax=axes[0][4],
#     title="K = 5/10",
# )

# img = k6.plot(
#     x="size",
#     y=[
#         "random",
#         "sorted",
#         "alternated",
#     ],
#     logy=True,
#     logx=True,
#     ax=axes[1][0],
#     title="K = 6/10",
# )

# img = k7.plot(
#     x="size",
#     y=[
#         "random",
#         "sorted",
#         "alternated",
#     ],
#     logy=True,
#     logx=True,
#     ax=axes[1][1],
#     title="K = 7/10",
# )

# img = k8.plot(
#     x="size",
#     y=[
#         "random",
#         "sorted",
#         "alternated",
#     ],
#     logy=True,
#     logx=True,
#     ax=axes[1][2],
#     title="K = 8/10",
# )

# img = k9.plot(
#     x="size",
#     y=[
#         "random",
#         "sorted",
#         "alternated",
#     ],
#     logy=True,
#     logx=True,
#     ax=axes[1][3],
#     title="K = 9/10",
# )

# img = k10.plot(
#     x="size",
#     y=[
#         "random",
#         "sorted",
#         "alternated",
#     ],
#     logy=True,
#     logx=True,
#     ax=axes[1][4],
#     title="K = 10/10",
# )


# plt.tight_layout()
# fig.savefig("results_two.png")
