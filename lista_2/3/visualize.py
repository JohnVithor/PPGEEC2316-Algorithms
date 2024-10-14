import pandas as pd
from matplotlib import pyplot as plt
import seaborn as sns

df: pd.DataFrame = pd.read_csv("results.csv")

df = df.groupby(["size", "k"]).mean().reset_index().drop("run", axis=1)

df = df.melt(id_vars=["size", "k"], var_name="order", value_name="value")

ratios = df[df["k"].str.contains("/")]
not_ratios = df[~df["k"].str.contains("/")]

sns.set_theme(font_scale=1.5)

sns.relplot(
    data=ratios,
    x="size",
    y="value",
    col="k",
    hue="order",
    kind="line",
    col_wrap=4,
)
plt.savefig("ratios_results_linear.png")

sns.relplot(
    data=not_ratios,
    x="size",
    y="value",
    col="k",
    hue="order",
    kind="line",
    col_wrap=4,
)
plt.savefig("not_ratios_results_linear.png")

sns.relplot(
    data=ratios,
    x="size",
    y="value",
    col="k",
    hue="order",
    kind="line",
    col_wrap=4,
)
plt.xscale("log")
plt.yscale("log")
plt.savefig("ratios_results_log.png")

sns.relplot(
    data=not_ratios,
    x="size",
    y="value",
    col="k",
    hue="order",
    kind="line",
    col_wrap=4,
)
plt.xscale("log")
plt.yscale("log")
plt.savefig("not_ratios_results_log.png")
