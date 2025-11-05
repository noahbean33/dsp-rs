/**
 * @file color_transform.c
 * @brief Implementation of color transformation operations
 */

#include "../include/color_transform.h"

int rgb_to_grayscale(const BMPImage *input, BMPImage *output) {
    int i;
    int imgSize = input->height * input->width;
    unsigned char r, g, b, gray;
    
    /* Check if input is 24-bit RGB */
    if (input->bitDepth != 24) {
        printf("Error: Input must be 24-bit RGB image\n");
        return -1;
    }
    
    /* Copy header and properties */
    if (bmp_copy(input, output) != 0) {
        return -1;
    }
    
    /* Convert each pixel to grayscale */
    for (i = 0; i < imgSize; i++) {
        b = input->data[i * 3];      /* Blue component */
        g = input->data[i * 3 + 1];  /* Green component */
        r = input->data[i * 3 + 2];  /* Red component */
        
        /* Calculate grayscale using standard luminance formula */
        gray = (unsigned char)(r * 0.30 + g * 0.59 + b * 0.11);
        
        /* Write grayscale value to all three channels */
        output->data[i * 3] = gray;
        output->data[i * 3 + 1] = gray;
        output->data[i * 3 + 2] = gray;
    }
    
    return 0;
}

int rgb_to_sepia(const BMPImage *input, BMPImage *output) {
    int i;
    int imgSize = input->height * input->width;
    unsigned char r, g, b;
    int tr, tg, tb;
    
    /* Check if input is 24-bit RGB */
    if (input->bitDepth != 24) {
        printf("Error: Input must be 24-bit RGB image\n");
        return -1;
    }
    
    /* Copy header and properties */
    if (bmp_copy(input, output) != 0) {
        return -1;
    }
    
    /* Apply sepia tone transformation */
    for (i = 0; i < imgSize; i++) {
        b = input->data[i * 3];
        g = input->data[i * 3 + 1];
        r = input->data[i * 3 + 2];
        
        /* Sepia transformation matrix */
        tr = (int)(r * 0.393 + g * 0.769 + b * 0.189);
        tg = (int)(r * 0.349 + g * 0.686 + b * 0.168);
        tb = (int)(r * 0.272 + g * 0.534 + b * 0.131);
        
        /* Clamp values to valid range */
        output->data[i * 3 + 2] = (tr > 255) ? 255 : (unsigned char)tr;
        output->data[i * 3 + 1] = (tg > 255) ? 255 : (unsigned char)tg;
        output->data[i * 3] = (tb > 255) ? 255 : (unsigned char)tb;
    }
    
    return 0;
}

int negative_image(const BMPImage *input, BMPImage *output) {
    int i;
    
    /* Copy header and properties */
    if (bmp_copy(input, output) != 0) {
        return -1;
    }
    
    /* Invert all pixel values */
    for (i = 0; i < input->dataSize; i++) {
        output->data[i] = 255 - input->data[i];
    }
    
    return 0;
}

int black_and_white(const BMPImage *input, BMPImage *output, unsigned char threshold) {
    int i;
    
    /* Copy header and properties */
    if (bmp_copy(input, output) != 0) {
        return -1;
    }
    
    /* Apply binary threshold */
    for (i = 0; i < input->dataSize; i++) {
        output->data[i] = (input->data[i] >= threshold) ? 255 : 0;
    }
    
    return 0;
}
