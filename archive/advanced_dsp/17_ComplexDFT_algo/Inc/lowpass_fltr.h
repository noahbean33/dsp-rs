#ifndef LOWPASS_FLTR_H_
#define LOWPASS_FLTR_H_

#include "arm_math.h"
void lowpass_filter_init(void);
q15_t lowpass_filter_exec(q15_t * input);

#endif
