import matplotlib.pyplot as plt
import matplotlib.font_manager as fm
fonts = fm.findSystemFonts(fontpaths=None, fontext='ttf')

# start plot
(fig, ax) = plt.subplots(figsize=(5, 5))
plt.xticks([])
plt.yticks([])
for side in ["bottom", "top", "left", "right"]:
    ax.spines[side].set_color("#FFFFFF00")

# colors
col_dark = "#191924"
col_orange = "#e5652e"
col_yellow = "#eed858"
col_light = "#faf7e5"

outer_col = col_orange
inner_col = col_dark
text_col = col_light
line_col = col_yellow

# outer box
lw = 24
xs_outer = [0.5, 1, 1, 0, 0, 0.5]
ys_outer = [0, 0, 1, 1, 0, 0]
plt.plot(xs_outer, ys_outer, c=outer_col, lw=lw, zorder=0)
plt.fill(xs_outer, ys_outer, c=outer_col, lw=0)

# inner box
eps = 0.05
xs_inner = [0.5, 1-eps, 1-eps, eps, eps, 0.5]
ys_inner = [eps, eps, 1-eps, 1-eps, eps, eps]
plt.plot(xs_inner, ys_inner, c=inner_col, lw=0.6*lw, zorder=2)
plt.fill(xs_inner, ys_inner, c=inner_col, lw=0)

# line
eps = 0.125
plt.plot([0.5, eps, 1-eps, 0.5], [0.485] * 4,
         lw=5, c=col_yellow)

# text
fontfamily = "Bungee"
fonts = [f for f in fonts if fontfamily.split()[0] in f]
for font in fonts:
    fm.fontManager.addfont(font)

fontweight = "light"
fontstyle = "normal"
case = "lower"

fontsize = 100
plt.text(0.5, 0.72, "TEX", fontsize=fontsize, ha="center", va="center", fontweight=fontweight, c=text_col, fontfamily=fontfamily, fontstyle=fontstyle)

fontsize = 94
plt.text(0.5, 0.25, "FMT", fontsize=fontsize, ha="center", va="center", fontweight=fontweight, c=text_col, fontfamily=fontfamily, fontstyle=fontstyle)

# save
plt.savefig("logo.svg", dpi=1000, transparent=True)
plt.close("all")
