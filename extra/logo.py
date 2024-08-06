import matplotlib.pyplot as plt

# start plot
(fig, ax) = plt.subplots(figsize=(5, 5))
plt.xticks([])
plt.yticks([])
for side in ["bottom", "top", "left", "right"]:
    ax.spines[side].set_color("#FFFFFF00")

# colors
col_dark = "#111111"
col_light = "#f9f9ee"
col_purple = "#d5b8f5"
col_1 = "#a8f896"
col_2 = "#2e95ef"
col_3 = "#ffec02"
col_4 = "#f56666"

# outer box
lw = 24
xs_outer = [0.5, 1, 1, 0, 0, 0.5]
ys_outer = [0, 0, 1, 1, 0, 0]
outer_col = col_purple
plt.plot(xs_outer, ys_outer, c=outer_col, lw=lw, zorder=0)
plt.fill(xs_outer, ys_outer, c=outer_col, lw=0)

# inner box
eps = 0.06
xs_inner = [0.5, 1-eps, 1-eps, eps, eps, 0.5]
ys_inner = [eps, eps, 1-eps, 1-eps, eps, eps]
inner_col = col_light
plt.plot(xs_inner, ys_inner, c=inner_col, lw=(1-eps)*lw, zorder=1)
plt.fill(xs_inner, ys_inner, c=inner_col, lw=0)

# text
fontsize=120
fontfamily = "Source Code Pro"
fontweight = "bold"
plt.text(0.5, 0.68, "tex", fontsize=fontsize, fontfamily=fontfamily, ha="center", va="center", fontweight=fontweight)
plt.text(0.5, 0.25, "fmt", fontsize=fontsize, fontfamily=fontfamily, ha="center", va="center", fontweight=fontweight)

# save
plt.savefig("logo.svg", dpi=1000, transparent=True)
#plt.savefig("logo.pdf", dpi=1000, transparent=True)
plt.close("all")
