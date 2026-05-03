/**
 * @file edge_detection.c
 * @brief Implementation of edge detection algorithms
 */

#include "../include/edge_detection.h"
#include <stdlib.h>
#include <string.h>

/* Helper function to create a convolution mask */
static int create_mask(ConvMask *mask, int size, const signed char *data) {
    mask->rows = size;
    mask->cols = size;
    mask->data = (signed char *)malloc(size * size);
    if (mask->data == NULL) {
        printf("Error: Unable to allocate memory for mask\n");
        return -1;
    }
    memcpy(mask->data, data, size * size);
    return 0;
}

/* Helper function to free a convolution mask */
static void free_mask(ConvMask *mask) {
    if (mask->data != NULL) {
        free(mask->data);
        mask->data = NULL;
    }
}

int convolve2d(const BMPImage *input, BMPImage *output, const ConvMask *mask) {
    long i, j, m, n, idx, jdx;
    int ms, im, val;
    int imgRows = input->height;
    int imgCols = input->width;
    
    /* Initialize output image */
    if (bmp_copy(input, output) != 0) {
        return -1;
    }
    
    /* Perform 2D discrete convolution */
    for (i = 0; i < imgRows; i++) {
        for (j = 0; j < imgCols; j++) {
            val = 0;
            
            /* Apply mask */
            for (m = 0; m < mask->rows; m++) {
                for (n = 0; n < mask->cols; n++) {
                    ms = mask->data[m * mask->rows + n];
                    idx = i - m;
                    jdx = j - n;
                    
                    /* Check bounds */
                    if (idx >= 0 && jdx >= 0 && idx < imgRows && jdx < imgCols) {
                        im = input->data[idx * imgCols + jdx];
                        val += ms * im;
                    }
                }
            }
            
            /* Clamp value to valid range */
            if (val > 255) val = 255;
            if (val < 0) val = 0;
            
            output->data[i * imgCols + j] = (unsigned char)val;
        }
    }
    
    return 0;
}

int sobel_vertical(const BMPImage *input, BMPImage *output) {
    ConvMask mask;
    signed char sobel_v[] = {-1, 0, 1, -2, 0, 2, -1, 0, 1};
    int result;
    
    if (create_mask(&mask, 3, sobel_v) != 0) {
        return -1;
    }
    
    result = convolve2d(input, output, &mask);
    free_mask(&mask);
    
    return result;
}

int sobel_horizontal(const BMPImage *input, BMPImage *output) {
    ConvMask mask;
    signed char sobel_h[] = {-1, -2, -1, 0, 0, 0, 1, 2, 1};
    int result;
    
    if (create_mask(&mask, 3, sobel_h) != 0) {
        return -1;
    }
    
    result = convolve2d(input, output, &mask);
    free_mask(&mask);
    
    return result;
}

int prewitt_vertical(const BMPImage *input, BMPImage *output) {
    ConvMask mask;
    signed char prewitt_v[] = {-1, 0, 1, -1, 0, 1, -1, 0, 1};
    int result;
    
    if (create_mask(&mask, 3, prewitt_v) != 0) {
        return -1;
    }
    
    result = convolve2d(input, output, &mask);
    free_mask(&mask);
    
    return result;
}

int prewitt_horizontal(const BMPImage *input, BMPImage *output) {
    ConvMask mask;
    signed char prewitt_h[] = {-1, -1, -1, 0, 0, 0, 1, 1, 1};
    int result;
    
    if (create_mask(&mask, 3, prewitt_h) != 0) {
        return -1;
    }
    
    result = convolve2d(input, output, &mask);
    free_mask(&mask);
    
    return result;
}

int roberts_vertical(const BMPImage *input, BMPImage *output) {
    ConvMask mask;
    signed char roberts_v[] = {1, 0, 0, -1};
    int result;
    
    if (create_mask(&mask, 2, roberts_v) != 0) {
        return -1;
    }
    
    result = convolve2d(input, output, &mask);
    free_mask(&mask);
    
    return result;
}

int roberts_horizontal(const BMPImage *input, BMPImage *output) {
    ConvMask mask;
    signed char roberts_h[] = {0, 1, -1, 0};
    int result;
    
    if (create_mask(&mask, 2, roberts_h) != 0) {
        return -1;
    }
    
    result = convolve2d(input, output, &mask);
    free_mask(&mask);
    
    return result;
}

int laplacian(const BMPImage *input, BMPImage *output) {
    ConvMask mask;
    signed char laplacian_mask[] = {0, 1, 0, 1, -4, 1, 0, 1, 0};
    int result;
    
    if (create_mask(&mask, 3, laplacian_mask) != 0) {
        return -1;
    }
    
    result = convolve2d(input, output, &mask);
    free_mask(&mask);
    
    return result;
}

int kirsch_north(const BMPImage *input, BMPImage *output) {
    ConvMask mask;
    signed char kirsch_n[] = {5, 5, 5, -3, 0, -3, -3, -3, -3};
    int result;
    
    if (create_mask(&mask, 3, kirsch_n) != 0) {
        return -1;
    }
    
    result = convolve2d(input, output, &mask);
    free_mask(&mask);
    
    return result;
}

int robinson_north(const BMPImage *input, BMPImage *output) {
    ConvMask mask;
    signed char robinson_n[] = {1, 1, 1, 1, -2, 1, -1, -1, -1};
    int result;
    
    if (create_mask(&mask, 3, robinson_n) != 0) {
        return -1;
    }
    
    result = convolve2d(input, output, &mask);
    free_mask(&mask);
    
    return result;
}

int line_horizontal(const BMPImage *input, BMPImage *output) {
    ConvMask mask;
    signed char line_h[] = {-1, -1, -1, 2, 2, 2, -1, -1, -1};
    int result;
    
    if (create_mask(&mask, 3, line_h) != 0) {
        return -1;
    }
    
    result = convolve2d(input, output, &mask);
    free_mask(&mask);
    
    return result;
}

int line_vertical(const BMPImage *input, BMPImage *output) {
    ConvMask mask;
    signed char line_v[] = {-1, 2, -1, -1, 2, -1, -1, 2, -1};
    int result;
    
    if (create_mask(&mask, 3, line_v) != 0) {
        return -1;
    }
    
    result = convolve2d(input, output, &mask);
    free_mask(&mask);
    
    return result;
}
