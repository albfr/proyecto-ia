import subprocess
import sys

import copy
import sys

import pygame
from pygame.locals import *

# Game constants
WIDTH = 1000
HEIGHT = 618
FPS = 60

TITLE_MARGIN = 8

FONT_FAMILY = "optima"
BIG_FONT_SIZE = 48
SMALL_FONT_SIZE = 36

BACKGROUND_COLOR = "purple"
BUTTON_COLOR = pygame.Color(200, 200, 200)
FONT_COLOR = pygame.Color("black")

# Constants used for scaling that look pretty
GOLDEN = 1.618
SILVER = 1.5

def render_rect(window, color, text, rect):
    """Renders text over a rect.

    Inputs:
    window: game window,
    color: background color of buttons,
    text: a pygame.Surface of text to be rendered over button,
    rect: a pygame.Rect.
    """

    pygame.draw.rect(window, color, rect)
    window.blit(
        text,
        (
            rect.left + (rect.width - text.get_width()) // 2,
            rect.top + (rect.height - text.get_height()) // 2,
        )
    )

def render_maze(window, font, font_color, grid):
    n = len(grid)
    CELL_COLOR = pygame.Color(234, 234, 234)
    START_COLOR = pygame.Color(12, 234, 12)
    TARGET_COLOR = pygame.Color(234, 12, 12)
    AT_GOAL_COLOR = pygame.Color(234, 234, 12)

    GENERATED_COLOR = pygame.Color(12, 12, 123)
    VISITED_COLOR = pygame.Color(12, 12, 234)

    MARKED_COLOR = pygame.Color(0, 0, 0)

    width = window.get_width()
    height = window.get_height()

    digits = [font.render(str(i), True, font_color) for i in range(10)]

    cell_margin = 2
    cell_padding = 3

    cell_size = cell_padding + max(
        max([s.get_width() for s in digits]),
        max([s.get_height() for s in digits])
    )

    cell_shift = cell_size + cell_margin

    maze_width = n * cell_size + (n - 1) * cell_margin
    maze_height = n * cell_size + (n - 1) * cell_margin

    for i in range(n):
        for j in range(n):
            rect = pygame.Rect(
                (width - maze_width) // 2 + j * cell_shift,
                (height - maze_height) / 2 + i * cell_shift,
                cell_size,
                cell_size
            )

            if grid[i][j] == '#':
                render_rect(window, MARKED_COLOR, font.render("", True, font_color), rect)
            else:
                render_rect(window, CELL_COLOR, font.render("", True, font_color), rect)

if __name__ == "__main__":
    n = int(sys.argv[1])

    m = n // 2

    i = 1
    j = 0

    xc_input = ""

    while True:
        if m + i <= n:
            xc_input += f"r{m + i} "
            xc_input += f"c{m + i} "
        if m + j > 0:
            xc_input += f"r{m + j} "
            xc_input += f"c{m + j} "

        i += 1
        j -= 1

        if m + i > n and m + j <= 0:
            break
    xc_input += "| "
    for i in range(2, 2 * n + 1):
        xc_input += f"a{i} "
    for i in range(1 - n, n):
        xc_input += f"b{i} "
    xc_input += '\n'

    for i in range(1, n + 1):
        for j in range(1, n + 1):
            xc_input += f"r{i} c{j} a{i + j} b{i - j}\n"

    p = subprocess.run(["cargo", "run",  "--release",  "--",  "-f", "-t", "1"], input=xc_input, capture_output=True, text=True)

    grid = [['.' for _ in range(n)] for _ in range(n)]

    for line in p.stdout.split('\n')[1:n+1]:
        items, _ = line.split('(')
        a, b, c, r = sorted(items.split())

        i = int(r[1:])
        j = int(c[1:])

        grid[i - 1][j - 1] = '#'

    # Initialize pygame, screen, clock and font
    pygame.init()

    window = pygame.display.set_mode((WIDTH, HEIGHT))
    clock = pygame.time.Clock()

    big_font = pygame.font.SysFont(FONT_FAMILY, BIG_FONT_SIZE)
    small_font = pygame.font.SysFont(FONT_FAMILY, SMALL_FONT_SIZE)

    # Game loop
    while True:
        for event in pygame.event.get():
            if event.type == QUIT:
                pygame.quit()
                sys.exit()

        # Fill screen to not get clipping
        window.fill(BACKGROUND_COLOR)

        render_maze(window, small_font, FONT_COLOR, grid)

        pygame.display.flip()
        clock.tick(FPS)
