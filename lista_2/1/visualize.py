import pandas as pd
from matplotlib import pyplot as plt
import numpy as np

df: pd.DataFrame = pd.read_csv("results.csv")

x_values = df["size"].values
y_values_1 = df["classic"].values
y_values_2 = df["strassen"].values
x_extrapolate = 8192 * 2

# Let's try a polynomial fit instead
poly_1 = np.polyfit(np.log2(x_values), np.log2(y_values_1), 3)
poly_2 = np.polyfit(np.log2(x_values), np.log2(y_values_2), 3)

# Polynomial extrapolation for x = 8192
log2_x_extrapolate = np.log2(x_extrapolate)
log2_y_extrapolate_1 = np.polyval(poly_1, log2_x_extrapolate)
log2_y_extrapolate_2 = np.polyval(poly_2, log2_x_extrapolate)

# Converting back from log scale
y_extrapolate_1_poly = 2**log2_y_extrapolate_1
y_extrapolate_2_poly = 2**log2_y_extrapolate_2

print(y_extrapolate_1_poly, y_extrapolate_2_poly)


fig, axes = plt.subplots(2, 2, figsize=(12, 9))


img = df.plot(
    x="size",
    y=["classic", "strassen"],
    ax=axes[0][0],
    title="Classic vs Strassen (linear scale)",
)

img = df.plot(
    x="size",
    y=["classic", "strassen"],
    ax=axes[0][1],
    logy=True,
    logx=True,
    title="Classic vs Strassen(log scale)",
)

cutpoint1 = 1024
cutpoint2 = 2048

img = df[(df["size"] >= cutpoint1) & (df["size"] <= cutpoint2)].plot(
    x="size",
    y=["classic", "strassen"],
    ax=axes[1][0],
    title=f"Classic vs Strassen(linear scale) size <= {cutpoint1}",
)

img = df[(df["size"] >= cutpoint1) & (df["size"] <= cutpoint2)].plot(
    x="size",
    y=["classic", "strassen"],
    ax=axes[1][1],
    logy=True,
    logx=True,
    title=f"Classic vs Strassen(log scale) size <= {cutpoint1}",
)

plt.tight_layout()
fig.savefig("results.png")
