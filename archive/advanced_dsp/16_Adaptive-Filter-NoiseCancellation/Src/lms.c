#include "lms.h"
#include <stdio.h>
#include "arm_math.h"

float32_t desired_output[ITERATIONS];
float32_t fltr_output[ITERATIONS];
float32_t err_values[ITERATIONS];
float32_t fltr_weights[FILTER_COEFFS + 1] = {0.0};
float32_t fltr_delay_line[FILTER_COEFFS + 1] = {0.0};


void lms_algo_test(void)
{
	int current_iteration, coeff_idx;

	float32_t desired_sample, output_sample, err_sample;

	for(current_iteration = 0; current_iteration < ITERATIONS; current_iteration++)
	{
		/*Obtain new input sample*/
		fltr_delay_line[0] =  sin(2 * M_PI* current_iteration/8);
		/*Obtain new desired output*/
		desired_sample =  cos(2 * M_PI* current_iteration/8);

		output_sample  = 0.0f;

		for( coeff_idx = 0;  coeff_idx <= FILTER_COEFFS; coeff_idx++ )
		{
			output_sample +=(fltr_weights[coeff_idx] * fltr_delay_line[coeff_idx]);
		}

		/*Get err*/
		err_sample =  desired_sample - output_sample;

		/*Update filter weights and delay line*/
		for(coeff_idx = FILTER_COEFFS; coeff_idx >= 0; coeff_idx--)
		{
			fltr_weights[coeff_idx] +=(LEARNING_RATE * err_sample *fltr_delay_line[coeff_idx]);
			if(coeff_idx != 0)
			{
				fltr_delay_line[coeff_idx] = fltr_delay_line[coeff_idx - 1];
			}
		}

		/*Store results for analysis*/
		desired_output[current_iteration] = desired_sample;
		fltr_output[current_iteration] = output_sample;
		err_values[current_iteration] = err_sample;


	}

    //TEST: Plot signals 1 by1 to analyze each
	//Only 1 of the 3 print statements should be enabled
//	for(int i =0; i<ITERATIONS; i++)
//	{
//		printf("%f\r\n", err_values[i]);
////		printf("%f\r\n", desired_output[i]);
////		printf("%f\r\n", fltr_output[i]);
//
//	}

	for(int i =0; i<ITERATIONS; i++)
	{
		printf("%f,", err_values[i]);
		printf("%f,", desired_output[i]);
		printf("%f\r\n", fltr_output[i]);

	}
}
