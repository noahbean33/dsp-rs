/**
 * @file image_adjust.h
 * @brief Image adjustment operations
 * 
 * This module provides operations for adjusting image properties:
 * - Brightness adjustment
 * - Contrast adjustment
 * - Histogram operations
 * - Image rotation
 */

#ifndef IMAGE_ADJUST_H
#define IMAGE_ADJUST_H

#include "bmp_io.h"

/**
 * @brief Adjust image brightness
 * 
 * Adds or subtracts a constant value from all pixels.
 * Positive factor increases brightness, negative decreases.
 * 
 * @param input Input image
 * @param output Output image
 * @param factor Brightness adjustment factor (-255 to 255)
 * @return 0 on success, -1 on failure
 */
int adjust_brightness(const BMPImage *input, BMPImage *output, int factor);

/**
 * @brief Calculate image histogram
 * 
 * Computes the normalized histogram (probability distribution) of pixel values.
 * 
 * @param input Input image (8-bit grayscale)
 * @param histogram Output histogram array (must be size 256)
 * @return 0 on success, -1 on failure
 */
int calculate_histogram(const BMPImage *input, float histogram[256]);

/**
 * @brief Apply histogram equalization
 * 
 * Enhances image contrast by spreading out intensity values.
 * 
 * @param input Input image (8-bit grayscale)
 * @param output Output image
 * @return 0 on success, -1 on failure
 */
int histogram_equalization(const BMPImage *input, BMPImage *output);

/**
 * @brief Rotate image 90 degrees clockwise
 * 
 * @param input Input image
 * @param output Output rotated image
 * @return 0 on success, -1 on failure
 */
int rotate_90_cw(const BMPImage *input, BMPImage *output);

/**
 * @brief Rotate image 180 degrees
 * 
 * @param input Input image
 * @param output Output rotated image
 * @return 0 on success, -1 on failure
 */
int rotate_180(const BMPImage *input, BMPImage *output);

/**
 * @brief Rotate image 90 degrees counter-clockwise
 * 
 * @param input Input image
 * @param output Output rotated image
 * @return 0 on success, -1 on failure
 */
int rotate_90_ccw(const BMPImage *input, BMPImage *output);

#endif /* IMAGE_ADJUST_H */
