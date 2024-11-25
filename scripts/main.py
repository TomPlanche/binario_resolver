import cv2
from cv2.typing import MatLike
from enum import Enum
from math import sqrt
import numpy as np
import sys
from typing import List, Tuple

class Color(Enum):
    GRAY = -1
    BLUE = 0
    RED = 1

def crop_image(image: MatLike, top_left: tuple[int, int], bottom_right: tuple[int, int]) -> MatLike:
    """
    Crop an image from the top left corner to the bottom right corner

    Parameters:
        - image: MatLike - The image to crop
        - top_left: tuple[int, int] - The top left corner of the crop
        - bottom_right: tuple[int, int] - The bottom right corner of the crop

    Returns:
        - MatLike - The cropped image
    """
    return image[top_left[1]:bottom_right[1], top_left[0]:bottom_right[0]]

def get_color(bgr_color: np.ndarray) -> Color:
    """
    Determine the color based on BGR values

    Parameters:
        - bgr_color: np.ndarray - BGR color values

    Returns:
        - Color - Enum value for the detected color
    """
    blue, green, red = bgr_color

    # #689BD1 blue
    # #E89E5D red
    if blue > 200 and green > 200 and red > 200:
        return Color.GRAY

    if blue > red and blue > green:
        return Color.BLUE

    if red > blue and red > green:
        return Color.RED

    raise ValueError('Color not recognized')

def detect_rectangles(image: MatLike, show_result: bool = False) -> List[List[Tuple[int, int, Color]]]:
    """
    Detect rectangles in an image and return their positions and colors

    Parameters:
        - image: MatLike - The image to detect rectangles in
        - show_result: bool - Whether to show the result or not

    Returns:
        - List[List[Tuple[int, int, Color]]] - Grid of (row, col, color) tuples
    """
    gray = cv2.cvtColor(image, cv2.COLOR_BGR2GRAY)

    ret, thrash = cv2.threshold(gray, 240, 255, cv2.CHAIN_APPROX_NONE)
    contours, _ = cv2.findContours(thrash, cv2.RETR_TREE, cv2.CHAIN_APPROX_NONE)

    contours = contours[::-1]

    # Store rectangles with their centers for sorting
    rectangles = []

    if contours:
        for contour in contours:
            approx = cv2.approxPolyDP(contour, 0.01 * cv2.arcLength(contour, True), True)

            # find the center of the rectangle
            center_x = (approx[0][0][0] + approx[2][0][0]) // 2
            center_y = (approx[0][0][1] + approx[2][0][1]) // 2

            color = get_color(image[center_y, center_x])

            rectangles.append({
                'center': (center_y, center_x),
                'color': color,
                'approx': approx
            })

            if show_result:
                cv2.drawContours(image, [approx], 0, (0, 0, 0), 2)

    grid_size = int(sqrt(len(rectangles)))

    # Sort rectangles by y coordinate (row)
    rectangles.sort(key=lambda r: r['center'][0])

    # Group rectangles into rows and sort each row by x coordinate
    grid = []
    for i in range(0, len(rectangles), grid_size):
        row = rectangles[i:i + grid_size]
        row.sort(key=lambda r: r['center'][1])
        grid.append(row)

    # Create the final grid with positions and colors
    result_grid = []
    for row_idx, row in enumerate(grid):
        result_row = []
        for col_idx, rect in enumerate(row):
            result_row.append((row_idx, col_idx, rect['color'].value))

            if show_result:
                center_y, center_x = rect['center']
                cv2.putText(image, f"({row_idx},{col_idx})",
                          (center_x, center_y),
                          cv2.FONT_HERSHEY_SIMPLEX,
                          0.5, (0, 0, 0), 2)

        result_grid.append(result_row)

    if show_result:
        cv2.imshow('Result', image)
        cv2.waitKey(0)
        cv2.destroyAllWindows()

    return result_grid

if __name__ == '__main__':
    args = sys.argv[1:]

    if not args:
        print('Usage: python main.py <image_path>')
        sys.exit(1)

    image_path, show_result = args if len(args) == 2 else (args[0], 'False')

    cropped_image = crop_image(cv2.imread(image_path), (860, 454), (1866, 1463))
    result = detect_rectangles(cropped_image, show_result == 'True')

    # write the result to a file
    with open('result.txt', 'w') as file:
        for row in result:
            file.write(','.join([str(cell[2]) for cell in row]) + '\n')
