import random

WIDTH = 400
HEIGHT = 400

# Initialize with grass
world = [["," for _ in range(WIDTH)] for _ in range(HEIGHT)]


# Helper to place a rectangle
def fill_rect(x, y, w, h, char):
    for dy in range(h):
        for dx in range(w):
            if 0 <= y + dy < HEIGHT and 0 <= x + dx < WIDTH:
                world[y + dy][x + dx] = char


# Helper to place border
def border_rect(x, y, w, h, char):
    for dx in range(w):
        if 0 <= y < HEIGHT and 0 <= x + dx < WIDTH:
            world[y][x + dx] = char
        if 0 <= y + h - 1 < HEIGHT and 0 <= x + dx < WIDTH:
            world[y + h - 1][x + dx] = char
    for dy in range(h):
        if 0 <= y + dy < HEIGHT and 0 <= x < WIDTH:
            world[y + dy][x] = char
        if 0 <= y + dy < HEIGHT and 0 <= x + w - 1 < WIDTH:
            world[y + dy][x + w - 1] = char


# World border
border_rect(0, 0, WIDTH, HEIGHT, "#")

# Create several distinct zones

# Zone 1: Town/Spawn area (top-left quadrant)
fill_rect(1, 1, 150, 100, ".")

# Buildings in town
buildings = [
    (5, 5, 20, 15),
    (30, 5, 25, 12),
    (60, 5, 15, 10),
    (5, 25, 18, 14),
    (30, 22, 30, 20),
    (70, 18, 20, 15),
    (100, 5, 25, 20),
    (5, 50, 25, 20),
    (35, 50, 20, 18),
    (60, 45, 30, 25),
    (100, 30, 22, 18),
    (130, 10, 18, 25),
]

for bx, by, bw, bh in buildings:
    border_rect(bx, by, bw, bh, "#")
    # Add door
    door_x = bx + bw // 2
    world[by + bh - 1][door_x] = "."
    # Add some desks inside
    for _ in range(random.randint(1, 4)):
        dx = bx + random.randint(2, bw - 3)
        dy = by + random.randint(2, bh - 3)
        world[dy][dx] = "D"
    # Maybe a coffee machine
    if random.random() > 0.5:
        world[by + 2][bx + 2] = "C"

# Roads in town
for x in range(1, 150):
    world[42][x] = "."
    world[43][x] = "."
    world[75][x] = "."
    world[76][x] = "."

for y in range(1, 100):
    world[y][25] = "."
    world[y][26] = "."
    world[y][55] = "."
    world[y][56] = "."
    world[y][95] = "."
    world[y][96] = "."

# Zone 2: Forest (top-right area)
for y in range(10, 180):
    for x in range(160, 380):
        if random.random() < 0.3:
            world[y][x] = "T"

# Forest clearings
clearings = [
    (200, 50, 25, 25),
    (280, 80, 30, 20),
    (320, 30, 20, 30),
    (250, 120, 35, 25),
]
for cx, cy, cw, ch in clearings:
    fill_rect(cx, cy, cw, ch, ",")

# Zone 3: Lake region (middle area)
lakes = [
    (50, 150, 80, 50),
    (180, 200, 100, 60),
    (320, 180, 60, 80),
]

for lx, ly, lw, lh in lakes:
    for dy in range(lh):
        for dx in range(lw):
            # Ellipse shape
            cx, cy = lw / 2, lh / 2
            if ((dx - cx) ** 2 / (cx**2) + (dy - cy) ** 2 / (cy**2)) < 1:
                if 0 <= ly + dy < HEIGHT and 0 <= lx + dx < WIDTH:
                    world[ly + dy][lx + dx] = "~"

# Zone 4: Mountain/Rocky area (bottom-left)
for y in range(280, 390):
    for x in range(10, 180):
        if random.random() < 0.15:
            world[y][x] = "#"

# Mountain paths
for x in range(10, 180):
    world[320][x] = ","
    world[350][x] = ","

for y in range(280, 390):
    world[y][50] = ","
    world[y][100] = ","
    world[y][150] = ","

# Zone 5: Plains with scattered trees (bottom-right)
for y in range(250, 390):
    for x in range(200, 390):
        if random.random() < 0.08:
            world[y][x] = "T"

# Ancient ruins in plains
ruins = [(250, 300, 40, 30), (320, 330, 35, 35)]
for rx, ry, rw, rh in ruins:
    # Broken walls
    for dx in range(rw):
        if random.random() > 0.3:
            world[ry][rx + dx] = "#"
        if random.random() > 0.3:
            world[ry + rh - 1][rx + dx] = "#"
    for dy in range(rh):
        if random.random() > 0.3:
            world[ry + dy][rx] = "#"
        if random.random() > 0.3:
            world[ry + dy][rx + rw - 1] = "#"
    # Floor inside
    fill_rect(rx + 1, ry + 1, rw - 2, rh - 2, ".")

# Main roads connecting areas
# Horizontal road from town to forest
for x in range(1, 399):
    world[100][x] = "."
    world[101][x] = "."

# Vertical road from town to mountains
for y in range(1, 399):
    world[y][150] = "."
    world[y][151] = "."

# Diagonal-ish path to plains
for i in range(150):
    x = 151 + i
    y = 100 + i
    if 0 <= y < HEIGHT and 0 <= x < WIDTH:
        world[y][x] = "."
        world[y][x + 1] = "."

# River from lake to edge
for y in range(200, 400):
    river_x = 130 + int(20 * (1 + 0.5 * ((y - 200) / 200)))
    for dx in range(-2, 3):
        if 0 <= river_x + dx < WIDTH:
            world[y][river_x + dx] = "~"

# Small islands in lakes
for lx, ly, lw, lh in lakes:
    cx, cy = lx + lw // 2, ly + lh // 2
    for dy in range(-3, 4):
        for dx in range(-3, 4):
            if abs(dx) + abs(dy) <= 3:
                if 0 <= cy + dy < HEIGHT and 0 <= cx + dx < WIDTH:
                    world[cy + dy][cx + dx] = ","

# Spawn plaza
fill_rect(10, 85, 12, 12, ".")
border_rect(10, 85, 12, 12, "#")
world[96][16] = "."  # entrance

# Output
with open(
    "/Users/kennychiu/Untitled/kennysliding/terminal-meet/terminal-united/crates/client/assets/world_map.txt",
    "w",
) as f:
    for row in world:
        f.write("".join(row) + "\n")

print("Generated 400x400 map!")
