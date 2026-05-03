#ifndef ADC_H_
#define ADC_H_

#include <stdint.h>

void pa1_adc_init(void);
uint32_t adc_read(void);
void start_conversion(void);

#define SR_EOC			(1U<<1)


#endif /* ADC_H_ */
