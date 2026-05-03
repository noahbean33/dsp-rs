/**
 * @file image_filters.c
 * @brief Implementation of image filtering operations
 */

#include "../include/image_filters.h"
#include "../include/edge_detection.h"
#include <stdlib.h>
#include <string.h>

/* Helper function for sorting (used in median filter) */
static void insertion_sort(unsigned char *arr, int n) {
    int i, j;
    unsigned char key;
    
    for (i = 1; i < n; i++) {
        key = arr[i];
        j = i - 1;
        
        while (j >= 0 && arr[j] > key) {
            arr[j + 1] = arr[j];
            j = j - 1;
        }
        arr[j + 1] = key;
    }
}

int median_filter(const BMPImage *input, BMPImage *output, int windowSize) {
    int x, y, i, j, z;
    int halfWindow = windowSize / 2;
    int imgRows = input->height;
    int imgCols = input->width;
    unsigned char *window;
    
    /* Allocate memory for window */
    window = (unsigned char *)malloc(windowSize * windowSize);
    if (window == NULL) {
        printf("Error: Unable to allocate memory for filter window\n");
        return -1;
    }
    
    /* Copy input to output */
    if (bmp_copy(input, output) != 0) {
        free(window);
        return -1;
    }
    
    /* Apply median filter */
    for (y = halfWindow; y < imgRows - halfWindow; y++) {
        for (x = halfWindow; x < imgCols - halfWindow; x++) {
            z = 0;
            
            /* Extract window */
            for (j = -halfWindow; j <= halfWindow; j++) {
                for (i = -halfWindow; i <= halfWindow; i++) {
                    window[z] = input->data[(x + i) + (long)(y + j) * imgCols];
                    z++;
                }
            }
            
            /* Sort and find median */
            insertion_sort(window, windowSize * windowSize);
            output->data[x + (long)y * imgCols] = window[windowSize * windowSize / 2];
        }
    }
    
    free(window);
    return 0;
}

int maximum_filter(const BMPImage *input, BMPImage *output, int windowSize) {
    int x, y, i, j;
    int halfWindow = windowSize / 2;
    int imgRows = input->height;
    int imgCols = input->width;
    unsigned char maxVal;
    
    /* Copy input to output */
    if (bmp_copy(input, output) != 0) {
        return -1;
    }
    
    /* Apply maximum filter */
    for (y = halfWindow; y < imgRows - halfWindow; y++) {
        for (x = halfWindow; x < imgCols - halfWindow; x++) {
            maxVal = 0;
            
            /* Find maximum in window */
            for (j = -halfWindow; j <= halfWindow; j++) {
                for (i = -halfWindow; i <= halfWindow; i++) {
                    unsigned char val = input->data[(x + i) + (long)(y + j) * imgCols];
                    if (val > maxVal) {
                        maxVal = val;
                    }
                }
            }
            
            output->data[x + (long)y * imgCols] = maxVal;
        }
    }
    
    return 0;
}

int minimum_filter(const BMPImage *input, BMPImage *output, int windowSize) {
    int x, y, i, j;
    int halfWindow = windowSize / 2;
    int imgRows = input->height;
    int imgCols = input->width;
    unsigned char minVal;
    
    /* Copy input to output */
    if (bmp_copy(input, output) != 0) {
        return -1;
    }
    
    /* Apply minimum filter */
    for (y = halfWindow; y < imgRows - halfWindow; y++) {
        for (x = halfWindow; x < imgCols - halfWindow; x++) {
            minVal = 255;
            
            /* Find minimum in window */
            for (j = -halfWindow; j <= halfWindow; j++) {
                for (i = -halfWindow; i <= halfWindow; i++) {
                    unsigned char val = input->data[(x + i) + (long)(y + j) * imgCols];
                    if (val < minVal) {
                        minVal = val;
                    }
                }
            }
            
            output->data[x + (long)y * imgCols] = minVal;
        }
    }
    
    return 0;
}

int gaussian_blur(const BMPImage *input, BMPImage *output) {
    /* 3x3 Gaussian kernel approximation */
    ConvMask mask;
    signed char gaussian[] = {1, 2, 1, 2, 4, 2, 1, 2, 1};
    int x, y;
    int imgRows = input->height;
    int imgCols = input->width;
    
    mask.rows = 3;
    mask.cols = 3;
    mask.data = gaussian;
    
    /* Apply convolution */
    if (convolve2d(input, output, &mask) != 0) {
        return -1;
    }
    
    /* Normalize by dividing by sum of weights (16) */
    for (y = 0; y < imgRows; y++) {
        for (x = 0; x < imgCols; x++) {
            output->data[y * imgCols + x] = output->data[y * imgCols + x] / 16;
        }
    }
    
    return 0;
}

int box_blur(const BMPImage *input, BMPImage *output, int windowSize) {
    int x, y, i, j;
    int halfWindow = windowSize / 2;
    int imgRows = input->height;
    int imgCols = input->width;
    int sum, count;
    
    /* Copy input to output */
    if (bmp_copy(input, output) != 0) {
        return -1;
    }
    
    /* Apply box blur (averaging) */
    for (y = halfWindow; y < imgRows - halfWindow; y++) {
        for (x = halfWindow; x < imgCols - halfWindow; x++) {
            sum = 0;
            count = 0;
            
            /* Calculate average in window */
            for (j = -halfWindow; j <= halfWindow; j++) {
                for (i = -halfWindow; i <= halfWindow; i++) {
                    sum += input->data[(x + i) + (long)(y + j) * imgCols];
                    count++;
                }
            }
            
            output->data[x + (long)y * imgCols] = (unsigned char)(sum / count);
        }
    }
    
    return 0;
}

int highpass_filter(const BMPImage *input, BMPImage *output) {
    /* High-pass filter mask */
    ConvMask mask;
    signed char highpass[] = {-1, -1, -1, -1, 9, -1, -1, -1, -1};
    
    mask.rows = 3;
    mask.cols = 3;
    mask.data = highpass;
    
    return convolve2d(input, output, &mask);
}
