# Changelog

## Version 1.0.0 - Initial Release

### Created
- **Core Modules**
  - `bmp_io` - BMP file I/O with dynamic memory management
  - `edge_detection` - 11 edge detection algorithms with unified convolution
  - `image_filters` - 7 filtering operations (median, blur, etc.)
  - `color_transform` - 4 color transformation functions
  - `image_adjust` - 6 adjustment operations (brightness, histogram, rotation)
  - `image_noise` - 3 noise generation functions

### Features
- Support for 8-bit grayscale and 24-bit RGB BMP images
- Comprehensive API documentation with doxygen-style comments
- Memory safety with proper allocation/deallocation
- Error handling with return codes and messages
- 3 complete example programs demonstrating library usage

### Documentation
- README.md - Comprehensive user guide
- LIBRARY_OVERVIEW.md - Technical design documentation
- CHANGELOG.md - This file
- Inline code documentation

### Build System
- Makefile for Linux/macOS/MinGW
- build.bat for Windows
- Example compilation instructions in README

### Test Data
- 14 standard test images (Lena, Cameraman, etc.)
- Images in test_images/ directory

### Improvements Over Original Code
1. ✅ Eliminated duplicate imageReader/imageWriter functions
2. ✅ Consolidated 30+ folders into 6 modules
3. ✅ Replaced fixed-size arrays with dynamic allocation
4. ✅ Added comprehensive documentation
5. ✅ Removed IDE-specific files (*.cbp, *.layout, bin/, obj/)
6. ✅ Created professional library structure
7. ✅ Unified convolution function for edge detection
8. ✅ Added error checking and validation
9. ✅ Provided working example programs
10. ✅ Created build automation scripts

### Files Consolidated

**Edge Detection** (7 folders → 1 module):
- Sobel-Edge+Detection
- Prewitt+Edge-Detection
- Roberts+Cross+Edge-Detector
- Kirsch+Edge-Detector
- Robinson+Edge-Detection
- LaplacianEdge-Detector
- LineDetector

**Filters** (6 folders → 1 module):
- MedianFilter
- MaximumFilter
- MinimumFilter
- ImageBlur
- GaussianNoise (partial)
- HighPassSpatialFilter

**Color Transform** (4 folders → 1 module):
- RGB_To_Grayscale
- RGB_to_Sepia
- NegativeImage
- BlackNWhite

**Image Adjust** (5 folders → 1 module):
- ImageBrightness
- ImageDarkness
- ImageBrightnessCorrection
- ImageHistogram
- ImageHistogramEqualization
- ImageRotation

**Noise** (2 folders → 1 module):
- GaussianNoise
- SaltAndPepper

**Core I/O** (2 folders → 1 module):
- ImageCopy
- ImageCopy_Modular
- DiscreteConvolution (convolution function)

### Known Issues
None currently. This is a stable initial release.

### Future Enhancements
See LIBRARY_OVERVIEW.md for potential future additions.

---

**Note**: The original `image_processing/` folder has been preserved for reference.
