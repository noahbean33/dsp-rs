#include <stdlib.h>
#include "fir_filter.h"

void fir_filter_init(fir_filter_type * fir,float32_t * fltr_kernel,uint32_t fltr_kernel_len)
{

	/*Reset buffer index*/
	fir->buff_idx = 0;

	/*Clear filter output*/
	fir->out =  0.00f;

	/*Set filter kernel*/
	fir->kernel =  fltr_kernel;

	/*Set filter kernel length*/
	fir->kernel_len = fltr_kernel_len;

	/*Dynamically allocate buffer and initialize to zero*/
	fir->buff  = (float32_t *)calloc(fir->kernel_len,sizeof(float32_t));



}



float32_t fir_filter_update(fir_filter_type * fir, float32_t curr_sample)
{
	uint32_t sum_idx;
	/*Store latest sample in the buffer*/
	fir->buff[fir->buff_idx] = curr_sample;

	/*Increment buff index and wrap around if at the end*/
	fir->buff_idx++;

	if(fir->buff_idx == (fir->kernel_len))
	{
		fir->buff_idx = 0;
	}

	/*Perform convolution*/
	fir->out  = 0.00f;

	sum_idx = fir->buff_idx;

	for( int i = 0; i < (fir->kernel_len); i++)
	{
		if( sum_idx > 0)
		{
			sum_idx--;
		}
		else
		{
			sum_idx =  (fir->kernel_len) -1;
		}

		fir->out += fir->kernel[i]*fir->buff[sum_idx];
	}

	return fir->out;
}
