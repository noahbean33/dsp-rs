#include <stdio.h>
#include "stm32f4xx.h"
#include "signals.h"
#include "uart.h"
#include "arm_math.h"
#include "adc.h"
#include "fir_filter.h"


#define OFFSET		5

extern  float LP_1HZ_2HZ_IMPULSE_RESPONSE[IMP_RSP_LENGTH];

float32_t ouput_signal_arr[KHZ1_15_SIG_LEN];

float g_in_sig_sample;

static void plot_input_signal(void);
static void serial_plot_input_sig(void);

static void pseudo_dly(int dly);
static void fpu_enable(void);



uint32_t g_sensor_value;

fir_filter_type lpf_fir;
uint32_t fltr_sensor_val;

int main()
{


	/*Enable fpu*/
	 fpu_enable();

	 /*Configure clock tree*/
	 clock_100MHz_config();

	/*Initialize the uart*/
	 uart2_tx_init();

    /*Initialize ADC*/
	 pa1_adc_init();

	 /*Start ADC conversion*/
	 start_conversion();

	 /*Initialize fir filter*/
	 fir_filter_init(&lpf_fir,LP_1HZ_2HZ_IMPULSE_RESPONSE,IMP_RSP_LENGTH);

	while(1)
	{

		g_sensor_value = adc_read();

		fltr_sensor_val = fir_filter_update(&lpf_fir,g_sensor_value);

		printf("%d,",(int)g_sensor_value);
		printf("%d\n\r",(int)fltr_sensor_val);

		pseudo_dly(90000);
	}
}







static void pseudo_dly(int dly)
{
	for( int i = 0; i < dly; i++ ){}
}

static void fpu_enable(void)
{
	/*Enable Floating Point Unit :  Enable CP10 and CP11 full access*/
	SCB->CPACR |= (1U<<20);
	SCB->CPACR |= (1U<<21);
	SCB->CPACR |= (1U<<22);
	SCB->CPACR |= (1U<<23);

}
