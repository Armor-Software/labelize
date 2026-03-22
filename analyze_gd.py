from PIL import Image

ref = Image.open('testdata/gd_thick.png')
rend = Image.open('testdata/rendered/gd_thick.png')
print('ref mode:', ref.mode, 'rend mode:', rend.mode)

def get_black_range(img, y, x_start, x_end):
    row = []
    for x in range(x_start, x_end):
        p = img.getpixel((x, y))
        v = p if isinstance(p, int) else p[0]
        if v < 128:
            row.append(x)
    return row

print('=== LEFT shape (R at 50,50 w=200 h=300) ===')
print('Reference:')
for y in [50, 100, 150, 200, 250, 300, 349]:
    row = get_black_range(ref, y, 40, 260)
    if row:
        print(f'  y={y}: x={row[0]}..{row[-1]} (width {row[-1]-row[0]+1})')
    else:
        print(f'  y={y}: (none)')

print('Rendered:')
for y in [50, 100, 150, 200, 250, 300, 349]:
    row = get_black_range(rend, y, 40, 260)
    if row:
        print(f'  y={y}: x={row[0]}..{row[-1]} (width {row[-1]-row[0]+1})')
    else:
        print(f'  y={y}: (none)')

print()
print('=== RIGHT shape (L at 300,50 w=200 h=300) ===')
print('Reference:')
for y in [50, 100, 150, 200, 250, 300, 349]:
    row = get_black_range(ref, y, 290, 510)
    if row:
        print(f'  y={y}: x={row[0]}..{row[-1]} (width {row[-1]-row[0]+1})')
    else:
        print(f'  y={y}: (none)')

print('Rendered:')
for y in [50, 100, 150, 200, 250, 300, 349]:
    row = get_black_range(rend, y, 290, 510)
    if row:
        print(f'  y={y}: x={row[0]}..{row[-1]} (width {row[-1]-row[0]+1})')
    else:
        print(f'  y={y}: (none)')
