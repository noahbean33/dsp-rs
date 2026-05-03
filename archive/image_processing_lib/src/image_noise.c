/**
 * @file image_noise.c
 * @brief Implementation of noise generation functions
 */

#include "../include/image_noise.h"
#include <stdlib.h>
#include <math.h>
#include <time.h>

/* Initialize random number generator (call once) */
static int rng_initialized = 0;

static void init_rng() {
    if (!rng_initialized) {
        srand((unsigned int)time(NULL));
        rng_initialized = 1;
    }
}

/* Generate random number in [0, 1] */
static double rand_uniform() {
    return (double)rand() / (double)RAND_MAX;
}

/* Generate Gaussian random number using Box-Muller transform */
static double rand_gaussian(double mean, double stddev) {
    static int has_spare = 0;
    static double spare;
    
    if (has_spare) {
        has_spare = 0;
        return mean + stddev * spare;
    }
    
    has_spare = 1;
    double u, v, s;
    do {
        u = rand_uniform() * 2.0 - 1.0;
        v = rand_uniform() * 2.0 - 1.0;
        s = u * u + v * v;
    } while (s >= 1.0 || s == 0.0);
    
    s = sqrt(-2.0 * log(s) / s);
    spare = v * s;
    
    return mean + stddev * u * s;
}

int add_gaussian_noise(const BMPImage *input, BMPImage *output, double mean, double stddev) {
    int i;
    int temp;
    
    init_rng();
    
    /* Copy header and properties */
    if (bmp_copy(input, output) != 0) {
        return -1;
    }
    
    /* Add Gaussian noise to each pixel */
    for (i = 0; i < input->dataSize; i++) {
        temp = (int)(input->data[i] + rand_gaussian(mean, stddev));
        
        /* Clamp to valid range */
        if (temp > 255) temp = 255;
        if (temp < 0) temp = 0;
        
        output->data[i] = (unsigned char)temp;
    }
    
    return 0;
}

int add_salt_pepper_noise(const BMPImage *input, BMPImage *output, double probability) {
    int i;
    double r;
    
    init_rng();
    
    /* Copy header and properties */
    if (bmp_copy(input, output) != 0) {
        return -1;
    }
    
    /* Add salt-and-pepper noise */
    for (i = 0; i < input->dataSize; i++) {
        r = rand_uniform();
        
        if (r < probability / 2.0) {
            /* Pepper (black) */
            output->data[i] = 0;
        } else if (r < probability) {
            /* Salt (white) */
            output->data[i] = 255;
        } else {
            /* No noise */
            output->data[i] = input->data[i];
        }
    }
    
    return 0;
}

int add_uniform_noise(const BMPImage *input, BMPImage *output, int amplitude) {
    int i;
    int temp, noise;
    
    init_rng();
    
    /* Copy header and properties */
    if (bmp_copy(input, output) != 0) {
        return -1;
    }
    
    /* Add uniform noise to each pixel */
    for (i = 0; i < input->dataSize; i++) {
        noise = (int)((rand_uniform() * 2.0 - 1.0) * amplitude);
        temp = input->data[i] + noise;
        
        /* Clamp to valid range */
        if (temp > 255) temp = 255;
        if (temp < 0) temp = 0;
        
        output->data[i] = (unsigned char)temp;
    }
    
    return 0;
}
