"""生成 fnos-desktop 应用图标源图（1024x1024）。

设计：深蓝渐变圆角方块 + 白色飞牛风格 "F" 标记（简笔牛角），
契合 fnOS 深色玻璃质感。
"""
from PIL import Image, ImageDraw, ImageFont

SIZE = 1024
img = Image.new("RGBA", (SIZE, SIZE), (0, 0, 0, 0))
d = ImageDraw.Draw(img)

# 圆角方块（半径 200）
radius = 200
d.rounded_rectangle([0, 0, SIZE - 1, SIZE - 1], radius=radius, fill=(13, 22, 34, 255))

# 顶部渐变：深蓝 → 更深蓝（逐行叠加）
top = (59, 130, 246)      # #3b82f6
bottom = (13, 22, 34)     # #0d1622
for y in range(0, SIZE // 2):
    t = y / (SIZE // 2)
    r = int(top[0] + (bottom[0] - top[0]) * t)
    g = int(top[1] + (bottom[1] - top[1]) * t)
    b = int(top[2] + (bottom[2] - top[2]) * t)
    d.line([(0, y), (SIZE, y)], fill=(r, g, b, 255))

# 底部到中段保持深色，加一点底部光晕
for y in range(SIZE // 2, SIZE):
    t = (y - SIZE // 2) / (SIZE // 2)
    r = int(bottom[0] + (30 - bottom[0]) * t * 0.4)
    g = int(bottom[1] + (45 - bottom[1]) * t * 0.4)
    b = int(bottom[2] + (70 - bottom[2]) * t * 0.4)
    d.line([(0, y), (SIZE, y)], fill=(r, g, b, 255))

# 白色 "F" 标记：粗圆角矩形横竖条 + 牛角两点（简笔）
white = (240, 246, 255, 255)
bar_w = 84
# 横条（F 上横）
d.rounded_rectangle([300, 300, 300 + 420, 300 + bar_w], radius=40, fill=white)
# 竖条（F 中竖）
d.rounded_rectangle([300, 300, 300 + bar_w, 700], radius=40, fill=white)
# 中横（F 中横）
d.rounded_rectangle([300, 470, 300 + 330, 470 + bar_w], radius=40, fill=white)
# 牛角两点（右下，示意 NAS/存储）
d.ellipse([640, 640, 760, 760], fill=(147, 197, 253, 255))
d.ellipse([680, 700, 800, 820], fill=(96, 165, 250, 200))

img.save("/vol3/1000/AIfile/hermesfile/work/fnos-desktop/icons-src/icon.png")
print("icon source saved")
