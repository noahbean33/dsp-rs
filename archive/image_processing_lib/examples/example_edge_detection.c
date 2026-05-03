/**
 * @file example_edge_detection.c
 * @brief Example demonstrating edge detection algorithms
 * 
 * This example shows how to use various edge detection algorithms
 * from the image processing library.
 * 
 * Compile:
 *   gcc -o edge_detect example_edge_detection.c ../src/*.c -I../include -lm
 * 
 * Run:
 *   ./edge_detect
 */

#include <stdio.h>
#include "../include/bmp_io.h"
#include "../include/edge_detection.h"

int main() {
    BMPImage input, output;
    int result;
    
    /* Initialize image structures */
    input.data = NULL;
    output.data = NULL;
    
    /* Read input image */
    printf("Reading input image...\n");
    if (bmp_read("../test_images/lena512.bmp", &input) != 0) {
        printf("Failed to read input image\n");
        return 1;
    }
    
    /* Apply Sobel vertical edge detection */
    printf("Applying Sobel vertical edge detection...\n");
    result = sobel_vertical(&input, &output);
    if (result == 0) {
        bmp_write("output_sobel_vertical.bmp", &output);
        printf("Sobel vertical: output_sobel_vertical.bmp\n");
        bmp_free(&output);
    }
    
    /* Apply Sobel horizontal edge detection */
    printf("Applying Sobel horizontal edge detection...\n");
    result = sobel_horizontal(&input, &output);
    if (result == 0) {
        bmp_write("output_sobel_horizontal.bmp", &output);
        printf("Sobel horizontal: output_sobel_horizontal.bmp\n");
        bmp_free(&output);
    }
    
    /* Apply Prewitt edge detection */
    printf("Applying Prewitt edge detection...\n");
    result = prewitt_vertical(&input, &output);
    if (result == 0) {
        bmp_write("output_prewitt.bmp", &output);
        printf("Prewitt: output_prewitt.bmp\n");
        bmp_free(&output);
    }
    
    /* Apply Laplacian edge detection */
    printf("Applying Laplacian edge detection...\n");
    result = laplacian(&input, &output);
    if (result == 0) {
        bmp_write("output_laplacian.bmp", &output);
        printf("Laplacian: output_laplacian.bmp\n");
        bmp_free(&output);
    }
    
    /* Apply Kirsch edge detection */
    printf("Applying Kirsch edge detection...\n");
    result = kirsch_north(&input, &output);
    if (result == 0) {
        bmp_write("output_kirsch.bmp", &output);
        printf("Kirsch: output_kirsch.bmp\n");
        bmp_free(&output);
    }
    
    /* Clean up */
    bmp_free(&input);
    
    printf("\nEdge detection complete!\n");
    return 0;
}
