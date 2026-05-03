/**
 * @file image_noise.h
 * @brief Noise generation for image processing
 * 
 * This module provides functions to add various types of noise to images:
 * - Gaussian noise
 * - Salt-and-pepper noise
 * - Uniform noise
 */

#ifndef IMAGE_NOISE_H
#define IMAGE_NOISE_H

#include "bmp_io.h"

/**
 * @brief Add Gaussian (normal) noise to image
 * 
 * Adds random noise with Gaussian distribution to each pixel.
 * 
 * @param input Input image
 * @param output Output noisy image
 * @param mean Mean of Gaussian distribution (typically 0)
 * @param stddev Standard deviation of Gaussian distribution
 * @return 0 on success, -1 on failure
 */
int add_gaussian_noise(const BMPImage *input, BMPImage *output, double mean, double stddev);

/**
 * @brief Add salt-and-pepper noise to image
 * 
 * Randomly sets pixels to either 0 (pepper) or 255 (salt).
 * 
 * @param input Input image
 * @param output Output noisy image
 * @param probability Probability of noise (0.0 to 1.0)
 * @return 0 on success, -1 on failure
 */
int add_salt_pepper_noise(const BMPImage *input, BMPImage *output, double probability);

/**
 * @brief Add uniform noise to image
 * 
 * Adds random noise uniformly distributed in [-amplitude, amplitude].
 * 
 * @param input Input image
 * @param output Output noisy image
 * @param amplitude Maximum noise amplitude
 * @return 0 on success, -1 on failure
 */
int add_uniform_noise(const BMPImage *input, BMPImage *output, int amplitude);

#endif /* IMAGE_NOISE_H */
