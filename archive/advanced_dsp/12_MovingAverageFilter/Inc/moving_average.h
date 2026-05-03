#ifndef MOVING_AVERAGE_H
#define MOVING_AVERAGE_H

#include "arm_math.h"


void moving_average(float32_t *sig_src_arr,
					float32_t *sig_out_arr,
					uint32_t signal_length,
					uint32_t filter_pts);
#endif
