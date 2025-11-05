/**
 * @file image_adjust.c
 * @brief Implementation of image adjustment operations
 */

#include "../include/image_adjust.h"
#include <string.h>

int adjust_brightness(const BMPImage *input, BMPImage *output, int factor) {
    int i;
    int temp;
    
    /* Copy header and properties */
    if (bmp_copy(input, output) != 0) {
        return -1;
    }
    
    /* Adjust brightness for each pixel */
    for (i = 0; i < input->dataSize; i++) {
        temp = input->data[i] + factor;
        
        /* Clamp to valid range [0, 255] */
        if (temp > 255) temp = 255;
        if (temp < 0) temp = 0;
        
        output->data[i] = (unsigned char)temp;
    }
    
    return 0;
}

int calculate_histogram(const BMPImage *input, float histogram[256]) {
    int x, y, i, j;
    long int ihist[256];
    long int sum;
    int imgRows = input->height;
    int imgCols = input->width;
    
    /* Check if image is 8-bit grayscale */
    if (input->bitDepth > 8) {
        printf("Error: Histogram requires 8-bit grayscale image\n");
        return -1;
    }
    
    /* Initialize histogram */
    for (i = 0; i < 256; i++) {
        ihist[i] = 0;
    }
    
    /* Count pixel intensities */
    sum = 0;
    for (y = 0; y < imgRows; y++) {
        for (x = 0; x < imgCols; x++) {
            j = input->data[x + y * imgCols];
            ihist[j]++;
            sum++;
        }
    }
    
    /* Normalize histogram */
    for (i = 0; i < 256; i++) {
        histogram[i] = (float)ihist[i] / (float)sum;
    }
    
    return 0;
}

int histogram_equalization(const BMPImage *input, BMPImage *output) {
    int x, y, i, j;
    int histeq[256];
    float hist[256];
    float sum;
    int imgRows = input->height;
    int imgCols = input->width;
    
    /* Check if image is 8-bit grayscale */
    if (input->bitDepth > 8) {
        printf("Error: Histogram equalization requires 8-bit grayscale image\n");
        return -1;
    }
    
    /* Calculate histogram */
    if (calculate_histogram(input, hist) != 0) {
        return -1;
    }
    
    /* Copy header and properties */
    if (bmp_copy(input, output) != 0) {
        return -1;
    }
    
    /* Calculate cumulative distribution function (CDF) */
    for (i = 0; i < 256; i++) {
        sum = 0.0;
        for (j = 0; j <= i; j++) {
            sum += hist[j];
        }
        histeq[i] = (int)(255 * sum + 0.5);
    }
    
    /* Apply histogram equalization */
    for (y = 0; y < imgRows; y++) {
        for (x = 0; x < imgCols; x++) {
            output->data[x + y * imgCols] = histeq[input->data[x + y * imgCols]];
        }
    }
    
    return 0;
}

int rotate_90_cw(const BMPImage *input, BMPImage *output) {
    int x, y;
    int imgRows = input->height;
    int imgCols = input->width;
    
    /* Copy header and update dimensions */
    memcpy(output->header, input->header, BMP_HEADER_SIZE);
    memcpy(output->colorTable, input->colorTable, BMP_COLOR_TABLE_SIZE);
    
    /* Swap width and height */
    output->width = imgRows;
    output->height = imgCols;
    output->bitDepth = input->bitDepth;
    output->dataSize = input->dataSize;
    
    /* Allocate memory */
    if (bmp_alloc_data(output, input->dataSize) != 0) {
        return -1;
    }
    
    /* Rotate pixels 90 degrees clockwise */
    for (y = 0; y < imgRows; y++) {
        for (x = 0; x < imgCols; x++) {
            output->data[(imgCols - 1 - x) * imgRows + y] = input->data[y * imgCols + x];
        }
    }
    
    return 0;
}

int rotate_180(const BMPImage *input, BMPImage *output) {
    int i;
    
    /* Copy header and properties */
    if (bmp_copy(input, output) != 0) {
        return -1;
    }
    
    /* Reverse pixel data */
    for (i = 0; i < input->dataSize; i++) {
        output->data[i] = input->data[input->dataSize - 1 - i];
    }
    
    return 0;
}

int rotate_90_ccw(const BMPImage *input, BMPImage *output) {
    int x, y;
    int imgRows = input->height;
    int imgCols = input->width;
    
    /* Copy header and update dimensions */
    memcpy(output->header, input->header, BMP_HEADER_SIZE);
    memcpy(output->colorTable, input->colorTable, BMP_COLOR_TABLE_SIZE);
    
    /* Swap width and height */
    output->width = imgRows;
    output->height = imgCols;
    output->bitDepth = input->bitDepth;
    output->dataSize = input->dataSize;
    
    /* Allocate memory */
    if (bmp_alloc_data(output, input->dataSize) != 0) {
        return -1;
    }
    
    /* Rotate pixels 90 degrees counter-clockwise */
    for (y = 0; y < imgRows; y++) {
        for (x = 0; x < imgCols; x++) {
            output->data[x * imgRows + (imgRows - 1 - y)] = input->data[y * imgCols + x];
        }
    }
    
    return 0;
}
