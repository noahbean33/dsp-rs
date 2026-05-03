#include "sine_generator.h"


void sine_gen_init_q15(sine_generator_q15_t *sine_desc, q15_t sine_freq, q15_t sampl_freq)
{
	/*Variables to hold intermediate sine values and coeffs*/
	float32_t y[4];
	float32_t coeff4;
	float32_t coeff5;


	y[0] = 0;

	/*Calculate sine values normalised by samplying freq for first 3 harmonics*/
	y[1] = arm_sin_f32(2 *PI *sine_freq/sampl_freq);
	y[2] = arm_sin_f32(4 *PI *sine_freq/sampl_freq);
	y[3] = arm_sin_f32(6 *PI *sine_freq/sampl_freq);

	/*Calculate coeffs for sine wave generator*/
	coeff4 = (y[2]/y[1])/2;
	coeff5 = ((y[1] * y[3] - y[2] * y[2])/(y[1] *y[1]))/2;

	/*Convert float to q*/
	arm_float_to_q15(&coeff4,&(sine_desc->coeff[4]),1);
	arm_float_to_q15(&coeff5,&(sine_desc->coeff[5]),1);

	/*Init biquad filter instance used for the sine wave generation*/
	arm_biquad_cascade_df1_init_q15(&(sine_desc->iir_sine_generator_instance),1,sine_desc->coeff,sine_desc->state,1);

	/*Initialize state variable*/
	arm_float_to_q15(&y[1],&(sine_desc->state[3]),1);


}

q15_t sine_calc_sample_q15(sine_generator_q15_t *sine_desc)
{
	q15_t input, output;
	input =0;

	/*Generate one sample of the sine wave using the biquad filter*/
	arm_biquad_cascade_df1_q15(&(sine_desc->iir_sine_generator_instance),&input,&output,1);

    return (output);

}
