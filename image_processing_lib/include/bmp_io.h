/**
 * @file bmp_io.h
 * @brief BMP image file I/O operations
 * 
 * This module provides functions for reading and writing BMP image files.
 * Supports grayscale (8-bit) and color (24-bit) BMP images.
 */

#ifndef BMP_IO_H
#define BMP_IO_H

#include <stdio.h>
#include <stdlib.h>

/* BMP file constants */
#define BMP_HEADER_SIZE         54
#define BMP_COLOR_TABLE_SIZE    1024
#define MAX_IMAGE_SIZE          1024*1024

/**
 * @brief Image structure to hold BMP image data
 */
typedef struct {
    int width;                              /* Image width in pixels */
    int height;                             /* Image height in pixels */
    int bitDepth;                           /* Bits per pixel (8 or 24) */
    unsigned char header[BMP_HEADER_SIZE];  /* BMP file header */
    unsigned char colorTable[BMP_COLOR_TABLE_SIZE]; /* Color table for 8-bit images */
    unsigned char *data;                    /* Image pixel data */
    size_t dataSize;                        /* Size of image data in bytes */
} BMPImage;

/**
 * @brief Read a BMP image from file
 * 
 * @param filename Path to the BMP file
 * @param img Pointer to BMPImage structure to store the image
 * @return 0 on success, -1 on failure
 */
int bmp_read(const char *filename, BMPImage *img);

/**
 * @brief Write a BMP image to file
 * 
 * @param filename Path to the output BMP file
 * @param img Pointer to BMPImage structure containing the image
 * @return 0 on success, -1 on failure
 */
int bmp_write(const char *filename, const BMPImage *img);

/**
 * @brief Allocate memory for image data
 * 
 * @param img Pointer to BMPImage structure
 * @param size Size of data to allocate
 * @return 0 on success, -1 on failure
 */
int bmp_alloc_data(BMPImage *img, size_t size);

/**
 * @brief Free image data memory
 * 
 * @param img Pointer to BMPImage structure
 */
void bmp_free(BMPImage *img);

/**
 * @brief Create a copy of an image
 * 
 * @param src Source image
 * @param dst Destination image
 * @return 0 on success, -1 on failure
 */
int bmp_copy(const BMPImage *src, BMPImage *dst);

#endif /* BMP_IO_H */
