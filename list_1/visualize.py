import pandas as pd
from matplotlib import pyplot as plt
import seaborn as sns

df_c: pd.DataFrame = pd.read_csv("c_3/results.csv")
df_c["language"] = "C"
df_cpp: pd.DataFrame = pd.read_csv("cpp_3/results.csv")
df_cpp["language"] = "C++"
df_rust: pd.DataFrame = pd.read_csv("rust_3/results.csv")
df_rust["language"] = "Rust"

df = pd.concat(
    [
        df_c,
        df_cpp,
        df_rust,
    ]
)
df = df.groupby(["language", "size"]).mean().reset_index()

fig, axes = plt.subplots(2, 3, figsize=(20, 9))

sns.lineplot(
    data=df,
    hue="language",
    x="size",
    y="insertion_worse",
    ax=axes[0][0],
).set_title("Worse case (insertion sort)")

sns.lineplot(
    data=df,
    hue="language",
    x="size",
    y="insertion_best",
    ax=axes[0][1],
).set_title("Best case (insertion sort)")

sns.lineplot(
    data=df,
    hue="language",
    x="size",
    y="insertion_avg",
    ax=axes[0][2],
).set_title("Average case (insertion sort)")

sns.lineplot(
    data=df,
    hue="language",
    x="size",
    y="merge_worse",
    ax=axes[1][0],
).set_title("Worse case (merge sort)")

sns.lineplot(
    data=df,
    hue="language",
    x="size",
    y="merge_best",
    ax=axes[1][1],
).set_title("Best case (merge sort)")

sns.lineplot(
    data=df,
    hue="language",
    x="size",
    y="merge_avg",
    ax=axes[1][2],
).set_title("Average case (merge sort)")


plt.tight_layout()
fig.savefig("results.png")
