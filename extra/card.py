from PIL import Image
import matplotlib.pyplot as plt
import matplotlib.font_manager as fm

# start plot
(fig, ax) = plt.subplots(figsize=(10, 5))
plt.xticks([])
plt.yticks([])
for side in ["bottom", "top", "left", "right"]:
    ax.spines[side].set_color("#FFFFFF00")

# colors
col_dark = "#191924"
col_yellow = "#eed858"
col_light = "#faf7e5"

outer_col = col_yellow
inner_col = col_light
text_col = col_dark

# outer box
w = 200
h = 100
xs_outer = [w/2, w, w, 0, 0, w/2]
ys_outer = [0, 0, h, h, 0, 0]
plt.fill(xs_outer, ys_outer, c=outer_col, lw=1, zorder=1)

# inner box
dw = 23
dh = 20 
xs_inner = [w/2, w-dw, w-dw, dw, dw, w/2]
ys_inner = [dh, dh, h-dh, h-dh, dh, dh]
plt.plot(xs_inner, ys_inner, c=inner_col, lw=30, zorder=2)
plt.fill(xs_inner, ys_inner, c=inner_col, lw=0)

# logo
img = Image.open("logo.png").resize((900, 900))
fig.figimage(img, 2210, 540)

# text
fontfamily = "Roboto Slab"
fonts = fm.findSystemFonts(fontpaths=None, fontext='ttf')
[fm.fontManager.addfont(f) for f in fonts if fontfamily.split()[0] in f]

fontsize = 16
plt.text(31, 50, "An extremely fast La\nformatter written in Rust.", fontsize=fontsize, ha="left", va="center", fontweight="light", c=text_col, fontfamily=fontfamily, fontstyle="normal")

plt.text(92.6, 53.53, "T", fontsize=fontsize, ha="left", va="center", fontweight="light", c=text_col, fontfamily=fontfamily, fontstyle="normal")

plt.text(96.55, 53.53, "eX", fontsize=fontsize, ha="left", va="center", fontweight="light", c=text_col, fontfamily=fontfamily, fontstyle="normal")

# save
plt.savefig("card.svg", dpi=400, transparent=True)
plt.close("all")
