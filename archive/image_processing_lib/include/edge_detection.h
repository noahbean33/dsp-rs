/**
 * @file edge_detection.h
 * @brief Edge detection algorithms for image processing
 * 
 * This module provides various edge detection algorithms including:
 * - Sobel operator (horizontal and vertical)
 * - Prewitt operator
 * - Roberts Cross operator
 * - Kirsch compass operator
 * - Robinson compass operator
 * - Laplacian operator
 * - Line detector (horizontal, vertical, diagonal)
 */

#ifndef EDGE_DETECTION_H
#define EDGE_DETECTION_H

#include "bmp_io.h"

/**
 * @brief Convolution mask structure
 */
typedef struct {
    int rows;           /* Number of rows in mask */
    int cols;           /* Number of columns in mask */
    signed char *data;  /* Mask coefficients */
} ConvMask;

/**
 * @brief Perform 2D discrete convolution
 * 
 * @param input Input image
 * @param output Output image (must be pre-allocated)
 * @param mask Convolution mask
 * @return 0 on success, -1 on failure
 */
int convolve2d(const BMPImage *input, BMPImage *output, const ConvMask *mask);

/**
 * @brief Apply Sobel edge detection (vertical)
 * 
 * Mask:
 *  -1  0  1
 *  -2  0  2
 *  -1  0  1
 * 
 * @param input Input image
 * @param output Output image
 * @return 0 on success, -1 on failure
 */
int sobel_vertical(const BMPImage *input, BMPImage *output);

/**
 * @brief Apply Sobel edge detection (horizontal)
 * 
 * Mask:
 *  -1 -2 -1
 *   0  0  0
 *   1  2  1
 * 
 * @param input Input image
 * @param output Output image
 * @return 0 on success, -1 on failure
 */
int sobel_horizontal(const BMPImage *input, BMPImage *output);

/**
 * @brief Apply Prewitt edge detection (vertical)
 * 
 * Mask:
 *  -1  0  1
 *  -1  0  1
 *  -1  0  1
 * 
 * @param input Input image
 * @param output Output image
 * @return 0 on success, -1 on failure
 */
int prewitt_vertical(const BMPImage *input, BMPImage *output);

/**
 * @brief Apply Prewitt edge detection (horizontal)
 * 
 * Mask:
 *  -1 -1 -1
 *   0  0  0
 *   1  1  1
 * 
 * @param input Input image
 * @param output Output image
 * @return 0 on success, -1 on failure
 */
int prewitt_horizontal(const BMPImage *input, BMPImage *output);

/**
 * @brief Apply Roberts Cross edge detection (vertical)
 * 
 * Mask:
 *   1  0
 *   0 -1
 * 
 * @param input Input image
 * @param output Output image
 * @return 0 on success, -1 on failure
 */
int roberts_vertical(const BMPImage *input, BMPImage *output);

/**
 * @brief Apply Roberts Cross edge detection (horizontal)
 * 
 * Mask:
 *   0  1
 *  -1  0
 * 
 * @param input Input image
 * @param output Output image
 * @return 0 on success, -1 on failure
 */
int roberts_horizontal(const BMPImage *input, BMPImage *output);

/**
 * @brief Apply Laplacian edge detection
 * 
 * Mask:
 *   0  1  0
 *   1 -4  1
 *   0  1  0
 * 
 * @param input Input image
 * @param output Output image
 * @return 0 on success, -1 on failure
 */
int laplacian(const BMPImage *input, BMPImage *output);

/**
 * @brief Apply Kirsch edge detection (North direction)
 * 
 * @param input Input image
 * @param output Output image
 * @return 0 on success, -1 on failure
 */
int kirsch_north(const BMPImage *input, BMPImage *output);

/**
 * @brief Apply Robinson edge detection (North direction)
 * 
 * @param input Input image
 * @param output Output image
 * @return 0 on success, -1 on failure
 */
int robinson_north(const BMPImage *input, BMPImage *output);

/**
 * @brief Detect horizontal lines
 * 
 * @param input Input image
 * @param output Output image
 * @return 0 on success, -1 on failure
 */
int line_horizontal(const BMPImage *input, BMPImage *output);

/**
 * @brief Detect vertical lines
 * 
 * @param input Input image
 * @param output Output image
 * @return 0 on success, -1 on failure
 */
int line_vertical(const BMPImage *input, BMPImage *output);

#endif /* EDGE_DETECTION_H */
