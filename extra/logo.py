import matplotlib.pyplot as plt

# start plot
(fig, ax) = plt.subplots(figsize=(5, 5))
plt.xticks([])
plt.yticks([])
for side in ["bottom", "top", "left", "right"]:
    ax.spines[side].set_color("#FFFFFF00")

# colors
col_dark = "#040404"
col_light = "#f9f9ee"
col_orange = "#BC3908"
col_yellow = "#F6AA1C"
col_brown = "#321911"
outer_col = col_dark
inner_col = col_yellow
text_col = col_dark

# outer box
lw = 24
xs_outer = [0.5, 1, 1, 0, 0, 0.5]
ys_outer = [0, 0, 1, 1, 0, 0]
plt.plot(xs_outer, ys_outer, c=outer_col, lw=lw, zorder=0)
plt.fill(xs_outer, ys_outer, c=outer_col, lw=0)

# inner box
eps = 0.04
xs_inner = [0.5, 1-eps, 1-eps, eps, eps, 0.5]
ys_inner = [eps, eps, 1-eps, 1-eps, eps, eps]
plt.plot(xs_inner, ys_inner, c=inner_col, lw=0.6*lw, zorder=1)
plt.fill(xs_inner, ys_inner, c=inner_col, lw=0)

# text
fontsize=120
fontfamily = "Source Code Pro"
fontweight = "bold"
plt.text(0.5, 0.68, "tex", fontsize=fontsize, fontfamily=fontfamily, ha="center", va="center", fontweight=fontweight, c=text_col)
plt.text(0.5, 0.25, "fmt", fontsize=fontsize, fontfamily=fontfamily, ha="center", va="center", fontweight=fontweight, c=text_col)

# save
plt.savefig("logo.svg", dpi=1000, transparent=True)
#plt.savefig("logo.pdf", dpi=1000, transparent=True)
plt.close("all")
