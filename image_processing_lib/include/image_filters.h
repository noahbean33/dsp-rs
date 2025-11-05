/**
 * @file image_filters.h
 * @brief Image filtering operations
 * 
 * This module provides various filtering operations including:
 * - Median filter (noise reduction)
 * - Maximum filter (dilation)
 * - Minimum filter (erosion)
 * - Gaussian blur
 * - Box blur
 * - High-pass spatial filter
 */

#ifndef IMAGE_FILTERS_H
#define IMAGE_FILTERS_H

#include "bmp_io.h"

/**
 * @brief Apply median filter for noise reduction
 * 
 * The median filter replaces each pixel with the median value
 * in a neighborhood window. Effective for salt-and-pepper noise.
 * 
 * @param input Input image
 * @param output Output image
 * @param windowSize Size of the filter window (e.g., 3, 5, 7)
 * @return 0 on success, -1 on failure
 */
int median_filter(const BMPImage *input, BMPImage *output, int windowSize);

/**
 * @brief Apply maximum filter (dilation)
 * 
 * Replaces each pixel with the maximum value in a neighborhood window.
 * 
 * @param input Input image
 * @param output Output image
 * @param windowSize Size of the filter window (e.g., 3, 5, 7)
 * @return 0 on success, -1 on failure
 */
int maximum_filter(const BMPImage *input, BMPImage *output, int windowSize);

/**
 * @brief Apply minimum filter (erosion)
 * 
 * Replaces each pixel with the minimum value in a neighborhood window.
 * 
 * @param input Input image
 * @param output Output image
 * @param windowSize Size of the filter window (e.g., 3, 5, 7)
 * @return 0 on success, -1 on failure
 */
int minimum_filter(const BMPImage *input, BMPImage *output, int windowSize);

/**
 * @brief Apply Gaussian blur
 * 
 * Applies a Gaussian filter for smooth blurring.
 * 
 * @param input Input image
 * @param output Output image
 * @return 0 on success, -1 on failure
 */
int gaussian_blur(const BMPImage *input, BMPImage *output);

/**
 * @brief Apply box blur (simple averaging filter)
 * 
 * Each pixel is replaced by the average of its neighbors.
 * 
 * @param input Input image
 * @param output Output image
 * @param windowSize Size of the filter window (e.g., 3, 5, 7)
 * @return 0 on success, -1 on failure
 */
int box_blur(const BMPImage *input, BMPImage *output, int windowSize);

/**
 * @brief Apply high-pass spatial filter
 * 
 * Enhances edges and high-frequency details.
 * 
 * @param input Input image
 * @param output Output image
 * @return 0 on success, -1 on failure
 */
int highpass_filter(const BMPImage *input, BMPImage *output);

#endif /* IMAGE_FILTERS_H */
