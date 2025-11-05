/**
 * @file example_filters.c
 * @brief Example demonstrating image filtering operations
 * 
 * This example shows how to use various filters including median,
 * blur, and high-pass filters.
 * 
 * Compile:
 *   gcc -o filters example_filters.c ../src/*.c -I../include -lm
 * 
 * Run:
 *   ./filters
 */

#include <stdio.h>
#include "../include/bmp_io.h"
#include "../include/image_filters.h"
#include "../include/image_noise.h"

int main() {
    BMPImage input, noisy, output;
    int result;
    
    /* Initialize image structures */
    input.data = NULL;
    noisy.data = NULL;
    output.data = NULL;
    
    /* Read input image */
    printf("Reading input image...\n");
    if (bmp_read("../test_images/lena512.bmp", &input) != 0) {
        printf("Failed to read input image\n");
        return 1;
    }
    
    /* Add salt-and-pepper noise */
    printf("Adding salt-and-pepper noise...\n");
    result = add_salt_pepper_noise(&input, &noisy, 0.05);
    if (result == 0) {
        bmp_write("output_noisy.bmp", &noisy);
        printf("Noisy image: output_noisy.bmp\n");
    }
    
    /* Apply median filter to remove noise */
    printf("Applying median filter (window size 7)...\n");
    result = median_filter(&noisy, &output, 7);
    if (result == 0) {
        bmp_write("output_median_filtered.bmp", &output);
        printf("Median filtered: output_median_filtered.bmp\n");
        bmp_free(&output);
    }
    
    /* Apply Gaussian blur */
    printf("Applying Gaussian blur...\n");
    result = gaussian_blur(&input, &output);
    if (result == 0) {
        bmp_write("output_gaussian_blur.bmp", &output);
        printf("Gaussian blur: output_gaussian_blur.bmp\n");
        bmp_free(&output);
    }
    
    /* Apply box blur */
    printf("Applying box blur (window size 5)...\n");
    result = box_blur(&input, &output, 5);
    if (result == 0) {
        bmp_write("output_box_blur.bmp", &output);
        printf("Box blur: output_box_blur.bmp\n");
        bmp_free(&output);
    }
    
    /* Apply high-pass filter */
    printf("Applying high-pass filter...\n");
    result = highpass_filter(&input, &output);
    if (result == 0) {
        bmp_write("output_highpass.bmp", &output);
        printf("High-pass: output_highpass.bmp\n");
        bmp_free(&output);
    }
    
    /* Apply maximum filter */
    printf("Applying maximum filter (window size 3)...\n");
    result = maximum_filter(&input, &output, 3);
    if (result == 0) {
        bmp_write("output_maximum.bmp", &output);
        printf("Maximum filter: output_maximum.bmp\n");
        bmp_free(&output);
    }
    
    /* Apply minimum filter */
    printf("Applying minimum filter (window size 3)...\n");
    result = minimum_filter(&input, &output, 3);
    if (result == 0) {
        bmp_write("output_minimum.bmp", &output);
        printf("Minimum filter: output_minimum.bmp\n");
        bmp_free(&output);
    }
    
    /* Clean up */
    bmp_free(&input);
    bmp_free(&noisy);
    
    printf("\nFiltering operations complete!\n");
    return 0;
}
