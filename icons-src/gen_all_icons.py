"""从源图生成 Tauri 所需全套图标（不依赖 tauri icon 命令）。
输出到 icons/：32x32.png / 128x128.png / 128x128@2x.png / icon.ico / icon.png
"""
from PIL import Image

SRC = "/vol3/1000/AIfile/hermesfile/work/fnos-desktop/icons-src/icon.png"
OUT = "/vol3/1000/AIfile/hermesfile/work/fnos-desktop/icons"
import os
os.makedirs(OUT, exist_ok=True)

src = Image.open(SRC).convert("RGBA")

sizes_png = {
    "32x32.png": 32,
    "128x128.png": 128,
    "128x128@2x.png": 256,
    "icon.png": 512,
}
for name, size in sizes_png.items():
    src.resize((size, size), Image.LANCZOS).save(os.path.join(OUT, name))
    print("saved", name)

# ico：多尺寸
ico_sizes = [(16, 16), (32, 32), (48, 48), (64, 64), (128, 128), (256, 256)]
frames = [src.resize(s, Image.LANCZOS) for s in ico_sizes]
src.save(os.path.join(OUT, "icon.ico"), format="ICO", sizes=ico_sizes, append_images=frames[1:])
print("saved icon.ico")

for f in sorted(os.listdir(OUT)):
    p = os.path.join(OUT, f)
    print(f, os.path.getsize(p), "bytes")
