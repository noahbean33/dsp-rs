/**
 * @file bmp_io.c
 * @brief Implementation of BMP image file I/O operations
 */

#include "../include/bmp_io.h"
#include <string.h>

int bmp_read(const char *filename, BMPImage *img) {
    FILE *file;
    int i;
    
    /* Open file for reading */
    file = fopen(filename, "rb");
    if (file == NULL) {
        printf("Error: Unable to open file '%s' for reading\n", filename);
        return -1;
    }
    
    /* Read BMP header (54 bytes) */
    for (i = 0; i < BMP_HEADER_SIZE; i++) {
        img->header[i] = getc(file);
    }
    
    /* Extract image properties from header */
    img->width = *(int *)&img->header[18];
    img->height = *(int *)&img->header[22];
    img->bitDepth = *(int *)&img->header[28];
    
    /* Read color table for 8-bit images */
    if (img->bitDepth <= 8) {
        fread(img->colorTable, sizeof(unsigned char), BMP_COLOR_TABLE_SIZE, file);
    }
    
    /* Calculate image data size */
    if (img->bitDepth == 24) {
        /* 24-bit RGB images have 3 bytes per pixel */
        img->dataSize = img->height * img->width * 3;
    } else {
        /* 8-bit grayscale images have 1 byte per pixel */
        img->dataSize = img->height * img->width;
    }
    
    /* Allocate memory for image data */
    if (bmp_alloc_data(img, img->dataSize) != 0) {
        fclose(file);
        return -1;
    }
    
    /* Read image data */
    fread(img->data, sizeof(unsigned char), img->dataSize, file);
    
    fclose(file);
    
    printf("Image loaded: %dx%d, %d-bit\n", img->width, img->height, img->bitDepth);
    
    return 0;
}

int bmp_write(const char *filename, const BMPImage *img) {
    FILE *file;
    
    /* Open file for writing */
    file = fopen(filename, "wb");
    if (file == NULL) {
        printf("Error: Unable to open file '%s' for writing\n", filename);
        return -1;
    }
    
    /* Write BMP header */
    fwrite(img->header, sizeof(unsigned char), BMP_HEADER_SIZE, file);
    
    /* Write color table for 8-bit images */
    if (img->bitDepth <= 8) {
        fwrite(img->colorTable, sizeof(unsigned char), BMP_COLOR_TABLE_SIZE, file);
    }
    
    /* Write image data */
    fwrite(img->data, sizeof(unsigned char), img->dataSize, file);
    
    fclose(file);
    
    printf("Image saved: %s\n", filename);
    
    return 0;
}

int bmp_alloc_data(BMPImage *img, size_t size) {
    img->data = (unsigned char *)malloc(size);
    if (img->data == NULL) {
        printf("Error: Unable to allocate memory for image data\n");
        return -1;
    }
    img->dataSize = size;
    return 0;
}

void bmp_free(BMPImage *img) {
    if (img->data != NULL) {
        free(img->data);
        img->data = NULL;
    }
}

int bmp_copy(const BMPImage *src, BMPImage *dst) {
    /* Copy header and properties */
    memcpy(dst->header, src->header, BMP_HEADER_SIZE);
    memcpy(dst->colorTable, src->colorTable, BMP_COLOR_TABLE_SIZE);
    dst->width = src->width;
    dst->height = src->height;
    dst->bitDepth = src->bitDepth;
    dst->dataSize = src->dataSize;
    
    /* Allocate and copy image data */
    if (bmp_alloc_data(dst, src->dataSize) != 0) {
        return -1;
    }
    memcpy(dst->data, src->data, src->dataSize);
    
    return 0;
}
