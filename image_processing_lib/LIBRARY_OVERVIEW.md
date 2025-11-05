# Image Processing Library - Overview

## Purpose

This library is a cleaned-up, modularized version of the original `image_processing/` codebase. It consolidates redundant code, improves structure, and provides a professional API for BMP image processing operations.

## Key Improvements

### 1. Eliminated Code Redundancy
**Before:** Every program had its own copy of `imageReader()` and `imageWriter()` functions.
**After:** Centralized in `bmp_io.c` module, used by all functions.

### 2. Modular Organization
**Before:** 30+ separate folders, each with standalone programs
**After:** 6 functional modules with clear separation of concerns:
- `bmp_io` - File I/O operations
- `edge_detection` - Edge detection algorithms
- `image_filters` - Filtering operations
- `color_transform` - Color space transformations
- `image_adjust` - Brightness, contrast, histogram
- `image_noise` - Noise generation

### 3. Professional Structure
```
Before:                          After:
ImageBlur/                       include/    (headers)
  ImageBlur/                     src/        (implementations)
    main.c                       examples/   (sample programs)
    bin/                         test_images/(test data)
    obj/
    *.cbp, *.layout
```

### 4. Better Memory Management
**Before:** Fixed-size arrays (`unsigned char buffer[512*512]`)
**After:** Dynamic allocation with proper cleanup functions

### 5. Documentation
**Before:** Minimal or no comments
**After:** Comprehensive documentation:
- Function headers with @brief, @param, @return
- Module-level documentation
- README with usage examples
- This overview document

## Module Details

### bmp_io Module
**Purpose:** Core BMP file handling
**Key Functions:**
- `bmp_read()` - Load BMP from file
- `bmp_write()` - Save BMP to file
- `bmp_copy()` - Duplicate image
- `bmp_free()` - Release memory

**Improvements:**
- Handles both 8-bit and 24-bit images
- Automatic data size calculation
- Error checking and reporting

### edge_detection Module
**Purpose:** Edge detection and line detection algorithms
**Consolidates:** Sobel, Prewitt, Roberts, Kirsch, Robinson, Laplacian, Line Detector (7 original folders)

**Key Function:**
- `convolve2d()` - Generic 2D convolution (reused by all edge detectors)

**Improvements:**
- Single convolution function instead of 7 copies
- Mask-based approach for flexibility
- Proper bounds checking

### image_filters Module
**Purpose:** Smoothing and noise reduction
**Consolidates:** Median, Maximum, Minimum, Gaussian, Blur, HighPass (6 original folders)

**Key Improvements:**
- Configurable window sizes
- Efficient sorting for median filter
- Normalized Gaussian kernel

### color_transform Module
**Purpose:** Color space conversions
**Consolidates:** RGB to Grayscale, RGB to Sepia, Negative, Black & White (4 original folders)

**Key Improvements:**
- Standard luminance coefficients for RGB→Gray
- Proper sepia transformation matrix
- Threshold parameter for B&W conversion

### image_adjust Module
**Purpose:** Image enhancement operations
**Consolidates:** Brightness, Darkness, BrightnessCorrection, Histogram, HistogramEqualization (5 original folders)

**Key Improvements:**
- Unified brightness adjustment (positive/negative factors)
- Proper CDF calculation for histogram equalization
- Rotation functions added

### image_noise Module
**Purpose:** Noise generation for testing
**Consolidates:** GaussianNoise, SaltAndPepper (2 original folders)

**Key Improvements:**
- Box-Muller transform for proper Gaussian distribution
- Configurable noise parameters
- Added uniform noise function

## Files Not Included

The following were intentionally excluded from the library:

### Build Artifacts
- `*.exe`, `*.o` - Compiled binaries
- `bin/`, `obj/` - Build directories
- `*.cbp`, `*.layout` - IDE project files

### Non-Image Processing
- `Running Sum/` - Signal processing, not image processing
- `Sample-Signals/` - Signal data files

### Redundant Files
- Multiple output images in each folder
- Duplicate test images

### Documentation Files
- `image_hist.txt` - Temporary output files

## Usage Philosophy

### Simple Operations
For single operations, use individual functions:
```c
BMPImage input, output;
bmp_read("input.bmp", &input);
sobel_vertical(&input, &output);
bmp_write("edges.bmp", &output);
bmp_free(&input);
bmp_free(&output);
```

### Complex Pipelines
Chain operations for image processing pipelines:
```c
BMPImage img, temp;
bmp_read("input.bmp", &img);

// Add noise
add_salt_pepper_noise(&img, &temp, 0.05);
bmp_free(&img);

// Filter noise
median_filter(&temp, &img, 5);
bmp_free(&temp);

// Detect edges
sobel_vertical(&img, &temp);
bmp_write("result.bmp", &temp);

bmp_free(&img);
bmp_free(&temp);
```

## Testing Strategy

### Test Images Provided
- Standard test images (Lena, Cameraman, etc.)
- Various sizes (256x256, 512x512)
- Different content (faces, scenes, patterns)

### Example Programs
Three comprehensive examples demonstrate:
1. Edge detection algorithms
2. Filtering operations
3. Color transformations and adjustments

### Validation
- Visual inspection of outputs
- Compare with original implementations
- Check edge cases (borders, extreme values)

## Future Enhancements

Potential additions to consider:

1. **Format Support**
   - JPEG, PNG support (requires external libraries)
   - Raw image formats

2. **Advanced Algorithms**
   - Canny edge detection
   - Harris corner detection
   - SIFT/SURF feature detection

3. **Morphological Operations**
   - Opening, closing
   - Erosion, dilation (already have min/max filters)

4. **Geometric Transforms**
   - Scaling/resizing
   - Arbitrary rotation angles
   - Affine transformations

5. **Performance**
   - Multi-threading support
   - SIMD optimizations
   - GPU acceleration

6. **API Improvements**
   - Error codes as enums
   - Callback functions for progress
   - Batch processing utilities

## Migration Guide

### From Original Code
If you have code using the original structure:

**Old:**
```c
unsigned char imgBuffer[512*512];
imageReader("input.bmp", &height, &width, &bitDepth, 
            imgHeader, imgColorTable, imgBuffer);
// ... process ...
imageWriter("output.bmp", imgHeader, imgColorTable, 
            imgBuffer, bitDepth);
```

**New:**
```c
BMPImage img;
img.data = NULL;
bmp_read("input.bmp", &img);
// ... process img.data ...
bmp_write("output.bmp", &img);
bmp_free(&img);
```

### Benefits of Migration
- No hardcoded array sizes
- Automatic memory management
- Better error handling
- More maintainable code

## Conclusion

This library represents a significant improvement over the original codebase while preserving all functionality. It serves as:
- **A learning resource** for image processing algorithms
- **A practical tool** for BMP image manipulation
- **A foundation** for more advanced image processing projects

The original `image_processing/` folder remains available for reference and comparison.
