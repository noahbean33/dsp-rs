#include "moving_average.h"



void moving_average(float32_t *sig_src_arr,
					float32_t *sig_out_arr,
					uint32_t signal_length,
					uint32_t filter_pts)
{
	uint32_t i,j;
	for( i  =  floor(filter_pts/2);  i < (signal_length -  (filter_pts / 2)) - 1; i++)
	{
		sig_out_arr[i] = 0;

		/*Sum up the 'filter_pts' number of points around sig_src_arr[i] in a sig_out_arr[i]*/
		for(j =  -(floor(filter_pts/2));  j < floor(filter_pts/2); j++){

			/*Add each surrounding point to the current output element*/
			sig_out_arr[i] = sig_out_arr[i]  +  sig_src_arr[i+j];
		}
		/*Divide by number of points to the the average*/
		sig_out_arr[i] = sig_out_arr[i] / filter_pts;
	}

}
