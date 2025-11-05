/**
 * @file example_color_adjust.c
 * @brief Example demonstrating color transformations and adjustments
 * 
 * This example shows how to perform color transformations, brightness
 * adjustments, and histogram equalization.
 * 
 * Compile:
 *   gcc -o color_adjust example_color_adjust.c ../src/*.c -I../include -lm
 * 
 * Run:
 *   ./color_adjust
 */

#include <stdio.h>
#include "../include/bmp_io.h"
#include "../include/color_transform.h"
#include "../include/image_adjust.h"

int main() {
    BMPImage input, output, rgb_input;
    int result;
    
    /* Initialize image structures */
    input.data = NULL;
    output.data = NULL;
    rgb_input.data = NULL;
    
    /* Read grayscale input image */
    printf("Reading grayscale input image...\n");
    if (bmp_read("../test_images/lena512.bmp", &input) != 0) {
        printf("Failed to read input image\n");
        return 1;
    }
    
    /* Adjust brightness (increase) */
    printf("Increasing brightness...\n");
    result = adjust_brightness(&input, &output, 50);
    if (result == 0) {
        bmp_write("output_bright.bmp", &output);
        printf("Brightened image: output_bright.bmp\n");
        bmp_free(&output);
    }
    
    /* Adjust brightness (decrease) */
    printf("Decreasing brightness...\n");
    result = adjust_brightness(&input, &output, -50);
    if (result == 0) {
        bmp_write("output_dark.bmp", &output);
        printf("Darkened image: output_dark.bmp\n");
        bmp_free(&output);
    }
    
    /* Create negative image */
    printf("Creating negative image...\n");
    result = negative_image(&input, &output);
    if (result == 0) {
        bmp_write("output_negative.bmp", &output);
        printf("Negative image: output_negative.bmp\n");
        bmp_free(&output);
    }
    
    /* Convert to black and white */
    printf("Converting to black and white (threshold 128)...\n");
    result = black_and_white(&input, &output, 128);
    if (result == 0) {
        bmp_write("output_bw.bmp", &output);
        printf("Black and white: output_bw.bmp\n");
        bmp_free(&output);
    }
    
    /* Apply histogram equalization */
    printf("Applying histogram equalization...\n");
    result = histogram_equalization(&input, &output);
    if (result == 0) {
        bmp_write("output_hist_eq.bmp", &output);
        printf("Histogram equalized: output_hist_eq.bmp\n");
        bmp_free(&output);
    }
    
    /* Read RGB color image */
    printf("\nReading RGB color input image...\n");
    if (bmp_read("../test_images/lena_color.bmp", &rgb_input) != 0) {
        printf("Failed to read RGB image\n");
        bmp_free(&input);
        return 1;
    }
    
    /* Convert RGB to grayscale */
    printf("Converting RGB to grayscale...\n");
    result = rgb_to_grayscale(&rgb_input, &output);
    if (result == 0) {
        bmp_write("output_grayscale.bmp", &output);
        printf("Grayscale: output_grayscale.bmp\n");
        bmp_free(&output);
    }
    
    /* Convert RGB to sepia */
    printf("Converting RGB to sepia tone...\n");
    result = rgb_to_sepia(&rgb_input, &output);
    if (result == 0) {
        bmp_write("output_sepia.bmp", &output);
        printf("Sepia tone: output_sepia.bmp\n");
        bmp_free(&output);
    }
    
    /* Clean up */
    bmp_free(&input);
    bmp_free(&rgb_input);
    
    printf("\nColor transformation and adjustment complete!\n");
    return 0;
}
