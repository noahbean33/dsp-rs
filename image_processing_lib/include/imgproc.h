/**
 * @file imgproc.h
 * @brief Main header file for the Image Processing Library
 * 
 * Include this single header to access all library functions.
 * 
 * Usage:
 *   #include "imgproc.h"
 */

#ifndef IMGPROC_H
#define IMGPROC_H

/* Include all module headers */
#include "bmp_io.h"
#include "edge_detection.h"
#include "image_filters.h"
#include "color_transform.h"
#include "image_adjust.h"
#include "image_noise.h"

/* Library version */
#define IMGPROC_VERSION_MAJOR 1
#define IMGPROC_VERSION_MINOR 0
#define IMGPROC_VERSION_PATCH 0

/* Utility macros */
#define MIN(a, b) ((a) < (b) ? (a) : (b))
#define MAX(a, b) ((a) > (b) ? (a) : (b))
#define CLAMP(val, min, max) (MAX((min), MIN((max), (val))))

#endif /* IMGPROC_H */
