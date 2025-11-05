@echo off
REM Build script for Image Processing Library (Windows)
REM Requires MinGW GCC or Visual Studio compiler

echo ========================================
echo Image Processing Library - Build Script
echo ========================================
echo.

REM Check if bin directory exists, create if not
if not exist bin mkdir bin

REM Compiler settings
set CC=gcc
set CFLAGS=-Wall -O2 -Iinclude
set SRC=src\bmp_io.c src\edge_detection.c src\image_filters.c src\color_transform.c src\image_adjust.c src\image_noise.c

echo Building examples...
echo.

REM Build edge detection example
echo [1/3] Building example_edge_detection...
%CC% %CFLAGS% -o bin\example_edge_detection.exe examples\example_edge_detection.c %SRC%
if %errorlevel% neq 0 (
    echo ERROR: Failed to build example_edge_detection
    goto error
)
echo       SUCCESS: bin\example_edge_detection.exe

REM Build filters example
echo [2/3] Building example_filters...
%CC% %CFLAGS% -o bin\example_filters.exe examples\example_filters.c %SRC%
if %errorlevel% neq 0 (
    echo ERROR: Failed to build example_filters
    goto error
)
echo       SUCCESS: bin\example_filters.exe

REM Build color adjust example
echo [3/3] Building example_color_adjust...
%CC% %CFLAGS% -o bin\example_color_adjust.exe examples\example_color_adjust.c %SRC%
if %errorlevel% neq 0 (
    echo ERROR: Failed to build example_color_adjust
    goto error
)
echo       SUCCESS: bin\example_color_adjust.exe

echo.
echo ========================================
echo Build completed successfully!
echo ========================================
echo.
echo Executables are in the bin\ directory:
echo   - bin\example_edge_detection.exe
echo   - bin\example_filters.exe
echo   - bin\example_color_adjust.exe
echo.
echo To run examples:
echo   cd bin
echo   example_edge_detection.exe
echo   example_filters.exe
echo   example_color_adjust.exe
echo.
goto end

:error
echo.
echo ========================================
echo Build failed!
echo ========================================
echo.
echo Make sure GCC (MinGW) is installed and in your PATH.
echo To install MinGW: https://www.mingw-w64.org/
echo.
exit /b 1

:end
pause
