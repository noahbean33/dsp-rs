# Quick Start Guide

## Get Started in 5 Minutes

### Step 1: Build the Library

**Windows (with MinGW):**
```bash
build.bat
```

**Linux/macOS:**
```bash
make
```

### Step 2: Run Examples

**Windows:**
```bash
cd bin
example_edge_detection.exe
example_filters.exe
example_color_adjust.exe
```

**Linux/macOS:**
```bash
./bin/example_edge_detection
./bin/example_filters
./bin/example_color_adjust
```

### Step 3: View Results

Output images will be created in the `bin/` directory:
- `output_sobel_vertical.bmp` - Sobel edge detection
- `output_median_filtered.bmp` - Noise reduction
- `output_bright.bmp` - Brightness adjustment
- And many more!

## Write Your First Program

Create `my_program.c`:

```c
#include "imgproc.h"

int main() {
    BMPImage input, output;
    input.data = NULL;
    output.data = NULL;
    
    // Read image
    if (bmp_read("test_images/lena512.bmp", &input) != 0) {
        return 1;
    }
    
    // Detect edges
    sobel_vertical(&input, &output);
    
    // Save result
    bmp_write("my_edges.bmp", &output);
    
    // Cleanup
    bmp_free(&input);
    bmp_free(&output);
    
    printf("Done! Check my_edges.bmp\n");
    return 0;
}
```

**Compile and run:**
```bash
# Windows
gcc -o my_program.exe my_program.c src/*.c -Iinclude

# Linux/macOS
gcc -o my_program my_program.c src/*.c -Iinclude -lm
```

## Common Tasks

### Task 1: Remove Noise
```c
BMPImage input, noisy, clean;

bmp_read("image.bmp", &input);
add_salt_pepper_noise(&input, &noisy, 0.05);  // Add noise
median_filter(&noisy, &clean, 7);              // Remove noise
bmp_write("clean.bmp", &clean);

bmp_free(&input);
bmp_free(&noisy);
bmp_free(&clean);
```

### Task 2: Enhance Contrast
```c
BMPImage input, output;

bmp_read("image.bmp", &input);
histogram_equalization(&input, &output);  // Enhance contrast
bmp_write("enhanced.bmp", &output);

bmp_free(&input);
bmp_free(&output);
```

### Task 3: Convert to Grayscale
```c
BMPImage rgb, gray;

bmp_read("color.bmp", &rgb);
rgb_to_grayscale(&rgb, &gray);
bmp_write("gray.bmp", &gray);

bmp_free(&rgb);
bmp_free(&gray);
```

### Task 4: Apply Multiple Filters
```c
BMPImage img, temp1, temp2;

bmp_read("input.bmp", &img);

// Blur
gaussian_blur(&img, &temp1);
bmp_free(&img);

// Detect edges
sobel_vertical(&temp1, &temp2);
bmp_free(&temp1);

// Enhance
adjust_brightness(&temp2, &temp1, 30);
bmp_write("result.bmp", &temp1);

bmp_free(&temp1);
bmp_free(&temp2);
```

## Available Functions

### Edge Detection
- `sobel_vertical()` / `sobel_horizontal()`
- `prewitt_vertical()` / `prewitt_horizontal()`
- `roberts_vertical()` / `roberts_horizontal()`
- `laplacian()`
- `kirsch_north()`, `robinson_north()`
- `line_horizontal()`, `line_vertical()`

### Filters
- `median_filter()` - Remove noise
- `gaussian_blur()` - Smooth blur
- `box_blur()` - Simple averaging
- `highpass_filter()` - Sharpen
- `maximum_filter()` - Dilate
- `minimum_filter()` - Erode

### Color
- `rgb_to_grayscale()`
- `rgb_to_sepia()`
- `negative_image()`
- `black_and_white()`

### Adjustments
- `adjust_brightness()`
- `histogram_equalization()`
- `rotate_90_cw()`, `rotate_180()`, `rotate_90_ccw()`

### Noise
- `add_gaussian_noise()`
- `add_salt_pepper_noise()`
- `add_uniform_noise()`

## Troubleshooting

### Build Errors
- **"gcc not found"**: Install MinGW (Windows) or GCC (Linux/macOS)
- **Link errors**: Make sure to compile all `.c` files in `src/`

### Runtime Errors
- **"Unable to open file"**: Check file path and that image exists
- **"Unable to allocate memory"**: Image too large or insufficient RAM

### Image Issues
- **All black output**: Check input image bit depth (8-bit or 24-bit BMP)
- **Corrupted output**: Ensure you call `bmp_free()` to clean up

## Next Steps

1. **Read the full README.md** for detailed API documentation
2. **Study the examples/** to see complete programs
3. **Read LIBRARY_OVERVIEW.md** for design details
4. **Experiment** with different parameters and combinations

## Need Help?

- Check `README.md` for full documentation
- Look at `examples/` for working code
- Review `LIBRARY_OVERVIEW.md` for technical details
- Examine the original code in `../image_processing/` folder

Happy image processing! 🖼️
