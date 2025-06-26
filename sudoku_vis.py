# Librerias necesarias para correr el codigo
import pygame
import heapq
import sys

# Game constants
WIDTH = 1000
HEIGHT = 618
FPS = 60

TITLE_MARGIN = 8

FONT_FAMILY = "optima"
BIG_FONT_SIZE = 48
SMALL_FONT_SIZE = 36

BACKGROUND_COLOR = "brown4"
BUTTON_COLOR = pygame.Color(200, 200, 200)
FONT_COLOR = pygame.Color("black")

# Constants used for scaling that look pretty
GOLDEN = 1.618
SILVER = 1.5

pygame.init()
pygame.display.set_caption('Polyomino')

window = pygame.display.set_mode((WIDTH, HEIGHT))
clock = pygame.time.Clock()

pygame.font.init()
big_font = pygame.font.SysFont(FONT_FAMILY, BIG_FONT_SIZE)
small_font = pygame.font.SysFont(FONT_FAMILY, SMALL_FONT_SIZE)


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

def render_maze(window, font, font_color, n):
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

    cell_margin = 4
    extra_margin = 5
    cell_padding = 25

    cell_size = cell_padding + max(
        max([s.get_width() for s in digits]),
        max([s.get_height() for s in digits])
    )

    cell_shift = cell_size + cell_margin

    maze_width = n * cell_size + (n - 1) * cell_margin + 2 * extra_margin
    maze_height = n * cell_size + (n - 1) * cell_margin + 2 * extra_margin


    for i in range(n):
        for j in range(n):
            rect = pygame.Rect(
                (width - maze_width) // 2 + j * cell_shift + extra_margin*(j//3),
                (height - maze_height) // 2 + i * cell_shift + extra_margin*(i//3),
                cell_size,
                cell_size
            )
            #print(f"({i}, {j}) = ({(width - maze_width) // 2 + j * cell_shift + extra_margin * (j/3)} , {(height - maze_height) // 2 + i * cell_shift + extra_margin * (i/3)}) -> ({extra_margin*(j/3)}, {extra_margin*(i/3)})")
            render_rect(window, CELL_COLOR, font.render("", True, font_color), rect)



# Game loop
while True:
    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            pygame.quit()
            sys.exit()

    # Fill screen to not get clipping
    window.fill(BACKGROUND_COLOR)

    render_maze(window, small_font, FONT_COLOR, 9)

    pygame.display.flip()
    clock.tick(FPS)