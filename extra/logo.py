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
col_dark = "#040404"
col_light = "#d3d0cb"
col_orange = "#a73102"
col_yellow = "#F6AA1C"
col_brown = "#321911"
outer_col = col_orange
inner_col = col_light
text_col = col_dark

# outer box
lw = 24
xs_outer = [0.5, 1, 1, 0, 0, 0.5]
ys_outer = [0, 0, 1, 1, 0, 0]
plt.plot(xs_outer, ys_outer, c=outer_col, lw=lw, zorder=0)
plt.fill(xs_outer, ys_outer, c=outer_col, lw=0)

# inner box
eps = 0.045
xs_inner = [0.5, 1-eps, 1-eps, eps, eps, 0.5]
ys_inner = [eps, eps, 1-eps, 1-eps, eps, eps]
plt.plot(xs_inner, ys_inner, c=inner_col, lw=0.6*lw, zorder=1)
plt.fill(xs_inner, ys_inner, c=inner_col, lw=0)

# text

# no
#fontfamily = "Paytone One"
#fontfamily = "Archivo Black"
#fontfamily = "Russo One"
#fontfamily = "Ultra"
#fontfamily = "Lobster"
#fontfamily = "Rowdies"
#fontfamily = "Acme"
#fontfamily = "Rammetto One"
#fontfamily = "Libre Baskerville"
#fontfamily = "Racing Sans One"
#fontfamily = "Changa One"

# maybe
fontfamily = "Averia Serif Libre"
#fontfamily = "Corben"
#fontfamily = "Calistoga"
#fontfamily = "Suez One"
#fontfamily = "Passion One"
#fontfamily = "Lilita One"
#fontfamily = "Gelasio"
#fontfamily = "Alfa Slab One"
#fontfamily = "Patua One"
#fontfamily = "Fira Sans"

fonts = [f for f in fonts if fontfamily.split()[0] in f]
for font in fonts:
    fm.fontManager.addfont(font)

fontsize=120
fontweight = "regular"
fontstyle = "normal"
plt.text(0.5, 0.68, "tex", fontsize=fontsize, ha="center", va="center", fontweight=fontweight, c=text_col, fontfamily=fontfamily, fontstyle=fontstyle)
plt.text(0.5, 0.25, "fmt", fontsize=fontsize, ha="center", va="center", fontweight=fontweight, c=text_col, fontfamily=fontfamily, fontstyle=fontstyle)

# save
plt.savefig("logo.svg", dpi=1000, transparent=True)
plt.close("all")
