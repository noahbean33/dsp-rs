# Image Processing Library

A comprehensive C library for image processing operations on BMP images. This library provides a clean, modular interface for common image processing tasks including edge detection, filtering, color transformations, and noise generation.

## Features

### 🖼️ BMP File I/O
- Read and write BMP image files (8-bit grayscale and 24-bit RGB)
- Automatic memory management
- Image copying and manipulation utilities

### 🔍 Edge Detection
- **Sobel operator** (horizontal and vertical)
- **Prewitt operator** (horizontal and vertical)
- **Roberts Cross operator**
- **Kirsch compass operator**
- **Robinson compass operator**
- **Laplacian operator**
- **Line detectors** (horizontal and vertical)

### 🎨 Image Filters
- **Median filter** - Noise reduction (effective for salt-and-pepper noise)
- **Gaussian blur** - Smooth blurring
- **Box blur** - Simple averaging filter
- **High-pass filter** - Edge enhancement
- **Maximum filter** - Morphological dilation
- **Minimum filter** - Morphological erosion

### 🌈 Color Transformations
- **RGB to Grayscale** - Standard luminance conversion
- **RGB to Sepia** - Vintage sepia tone effect
- **Negative image** - Color inversion
- **Black and White** - Binary threshold conversion

### ⚙️ Image Adjustments
- **Brightness adjustment** - Increase or decrease brightness
- **Histogram calculation** - Compute intensity distribution
- **Histogram equalization** - Contrast enhancement
- **Image rotation** - 90°, 180°, 270° rotation

### 🎲 Noise Generation
- **Gaussian noise** - Additive white Gaussian noise (AWGN)
- **Salt-and-pepper noise** - Impulse noise
- **Uniform noise** - Uniformly distributed noise

## Directory Structure

```
image_processing_lib/
├── include/              # Header files
│   ├── bmp_io.h         # BMP file I/O
│   ├── edge_detection.h # Edge detection algorithms
│   ├── image_filters.h  # Filtering operations
│   ├── color_transform.h# Color transformations
│   ├── image_adjust.h   # Image adjustments
│   └── image_noise.h    # Noise generation
├── src/                 # Implementation files
│   ├── bmp_io.c
│   ├── edge_detection.c
│   ├── image_filters.c
│   ├── color_transform.c
│   ├── image_adjust.c
│   └── image_noise.c
├── examples/            # Example programs
│   ├── example_edge_detection.c
│   ├── example_filters.c
│   └── example_color_adjust.c
├── test_images/         # Sample test images
└── README.md            # This file
```

## Getting Started

### Prerequisites
- C compiler (GCC, Clang, or MSVC)
- Standard C library
- Math library (libm)

### Compilation

#### Compile a single example:
```bash
gcc -o edge_detect examples/example_edge_detection.c src/*.c -Iinclude -lm
```

#### Compile all examples:
```bash
# Linux/macOS
gcc -o edge_detect examples/example_edge_detection.c src/*.c -Iinclude -lm
gcc -o filters examples/example_filters.c src/*.c -Iinclude -lm
gcc -o color_adjust examples/example_color_adjust.c src/*.c -Iinclude -lm

# Windows (MinGW)
gcc -o edge_detect.exe examples/example_edge_detection.c src/*.c -Iinclude
gcc -o filters.exe examples/example_filters.c src/*.c -Iinclude
gcc -o color_adjust.exe examples/example_color_adjust.c src/*.c -Iinclude
```

### Running Examples

```bash
# Run edge detection example
./edge_detect

# Run filtering example
./filters

# Run color/adjustment example
./color_adjust
```

## Usage Examples

### Basic Image I/O

```c
#include "bmp_io.h"

int main() {
    BMPImage img;
    img.data = NULL;
    
    // Read image
    if (bmp_read("input.bmp", &img) == 0) {
        printf("Image: %dx%d, %d-bit\n", 
               img.width, img.height, img.bitDepth);
        
        // Process image here...
        
        // Write result
        bmp_write("output.bmp", &img);
    }
    
    // Clean up
    bmp_free(&img);
    return 0;
}
```

### Edge Detection

```c
#include "bmp_io.h"
#include "edge_detection.h"

int main() {
    BMPImage input, output;
    input.data = NULL;
    output.data = NULL;
    
    bmp_read("input.bmp", &input);
    
    // Apply Sobel edge detection
    sobel_vertical(&input, &output);
    bmp_write("edges.bmp", &output);
    
    bmp_free(&input);
    bmp_free(&output);
    return 0;
}
```

### Noise Reduction

```c
#include "bmp_io.h"
#include "image_filters.h"
#include "image_noise.h"

int main() {
    BMPImage input, noisy, filtered;
    input.data = NULL;
    noisy.data = NULL;
    filtered.data = NULL;
    
    bmp_read("input.bmp", &input);
    
    // Add salt-and-pepper noise
    add_salt_pepper_noise(&input, &noisy, 0.05);
    bmp_write("noisy.bmp", &noisy);
    
    // Remove noise with median filter
    median_filter(&noisy, &filtered, 5);
    bmp_write("filtered.bmp", &filtered);
    
    bmp_free(&input);
    bmp_free(&noisy);
    bmp_free(&filtered);
    return 0;
}
```

### Color Transformations

```c
#include "bmp_io.h"
#include "color_transform.h"

int main() {
    BMPImage rgb, gray, sepia;
    rgb.data = NULL;
    gray.data = NULL;
    sepia.data = NULL;
    
    // Read RGB image
    bmp_read("color_image.bmp", &rgb);
    
    // Convert to grayscale
    rgb_to_grayscale(&rgb, &gray);
    bmp_write("grayscale.bmp", &gray);
    
    // Convert to sepia
    rgb_to_sepia(&rgb, &sepia);
    bmp_write("sepia.bmp", &sepia);
    
    bmp_free(&rgb);
    bmp_free(&gray);
    bmp_free(&sepia);
    return 0;
}
```

### Brightness and Contrast

```c
#include "bmp_io.h"
#include "image_adjust.h"

int main() {
    BMPImage input, output;
    input.data = NULL;
    output.data = NULL;
    
    bmp_read("input.bmp", &input);
    
    // Increase brightness
    adjust_brightness(&input, &output, 50);
    bmp_write("brighter.bmp", &output);
    bmp_free(&output);
    
    // Histogram equalization for contrast enhancement
    histogram_equalization(&input, &output);
    bmp_write("enhanced.bmp", &output);
    
    bmp_free(&input);
    bmp_free(&output);
    return 0;
}
```

## API Reference

### Core Data Structures

#### BMPImage
```c
typedef struct {
    int width;                    // Image width in pixels
    int height;                   // Image height in pixels
    int bitDepth;                 // Bits per pixel (8 or 24)
    unsigned char header[54];     // BMP file header
    unsigned char colorTable[1024]; // Color table (8-bit images)
    unsigned char *data;          // Image pixel data
    size_t dataSize;              // Size of image data
} BMPImage;
```

#### ConvMask
```c
typedef struct {
    int rows;           // Number of rows in mask
    int cols;           // Number of columns in mask
    signed char *data;  // Mask coefficients
} ConvMask;
```

### Function Return Values
- **0** - Success
- **-1** - Failure (error message printed to stdout)

### Memory Management
- Always initialize image pointers to NULL: `img.data = NULL;`
- Call `bmp_free()` to release image memory
- Functions that allocate output images handle memory internally

## Test Images

The `test_images/` directory contains several standard test images:
- `lena512.bmp` - 512x512 grayscale Lena image
- `lena_color.bmp` - Color version of Lena
- `cameraman.bmp` - 256x256 cameraman image
- `girlface.bmp` - Portrait for edge detection testing
- And more...

## Performance Considerations

- **Image size limits**: Default maximum is 1024x1024 pixels
- **Memory usage**: Images are stored in memory during processing
- **Filter window size**: Larger windows increase processing time
- **Convolution operations**: O(n²×m²) complexity where n=image size, m=mask size

## Improvements from Original Code

This library provides significant improvements over the original codebase:

1. **✅ Eliminated redundancy** - Common functions (imageReader/imageWriter) consolidated into `bmp_io` module
2. **✅ Modular design** - Organized by functionality (edge detection, filters, etc.)
3. **✅ Comprehensive comments** - Detailed documentation for all functions
4. **✅ Better structure** - Header/source separation, clear API
5. **✅ Memory safety** - Proper allocation/deallocation functions
6. **✅ Error handling** - Return codes and error messages
7. **✅ Examples** - Working example programs for each module
8. **✅ No hardcoded paths** - Flexible file I/O

## Known Limitations

- Only supports 8-bit grayscale and 24-bit RGB BMP images
- No support for compressed BMP formats
- Rotation functions modify image dimensions
- Some filters may not preserve image borders

## Contributing

This library was created for educational purposes. Feel free to:
- Add new algorithms
- Optimize existing implementations
- Extend to support additional image formats
- Improve error handling

## License

This is educational code. Use freely for learning and non-commercial purposes.

## References

- Gonzalez & Woods, "Digital Image Processing"
- BMP file format specification
- Standard image processing algorithms

## Contact

For questions or issues, refer to the original codebase in the `image_processing/` directory.

---

**Note**: This library consolidates and cleans up the code from the `image_processing/` folder, which has been kept intact for reference.
