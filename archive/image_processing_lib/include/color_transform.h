/**
 * @file color_transform.h
 * @brief Color transformation operations
 * 
 * This module provides color space transformations including:
 * - RGB to grayscale conversion
 * - RGB to sepia tone
 * - Negative image transformation
 * - Black and white conversion
 */

#ifndef COLOR_TRANSFORM_H
#define COLOR_TRANSFORM_H

#include "bmp_io.h"

/**
 * @brief Convert RGB image to grayscale
 * 
 * Uses the standard luminance formula:
 * Gray = 0.30*R + 0.59*G + 0.11*B
 * 
 * @param input Input RGB image (24-bit)
 * @param output Output grayscale image
 * @return 0 on success, -1 on failure
 */
int rgb_to_grayscale(const BMPImage *input, BMPImage *output);

/**
 * @brief Convert RGB image to sepia tone
 * 
 * Applies a warm, vintage sepia tone effect.
 * 
 * @param input Input RGB image (24-bit)
 * @param output Output sepia image
 * @return 0 on success, -1 on failure
 */
int rgb_to_sepia(const BMPImage *input, BMPImage *output);

/**
 * @brief Create negative image
 * 
 * Inverts all pixel values: output = 255 - input
 * 
 * @param input Input image
 * @param output Output negative image
 * @return 0 on success, -1 on failure
 */
int negative_image(const BMPImage *input, BMPImage *output);

/**
 * @brief Convert to black and white (binary threshold)
 * 
 * Pixels below threshold become black (0), above become white (255).
 * 
 * @param input Input image
 * @param output Output binary image
 * @param threshold Threshold value (0-255)
 * @return 0 on success, -1 on failure
 */
int black_and_white(const BMPImage *input, BMPImage *output, unsigned char threshold);

#endif /* COLOR_TRANSFORM_H */
