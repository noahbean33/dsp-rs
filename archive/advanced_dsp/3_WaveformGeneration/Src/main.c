#include <stdio.h>
#include "stm32f4xx.h"
#include "signals.h"
#include "uart.h"
#include "arm_math.h"
#include "FreeRTOS.h"
#include "task.h"
#include "sine_generator.h"


#define NOISE_FREQ		50
#define SIGNAL_FREQ		10
#define SAMPLING_FREQ	1000



extern float _5hz_signal[HZ_5_SIG_LEN];
extern float32_t input_signal_f32_1kHz_15kHz[KHZ1_15_SIG_LEN] ;

float g_in_sig_sample;

static void plot_input_signal(void);
static void serial_plot_input_sig(void);

static void pseudo_dly(int dly);
static void fpu_enable(void);

uint32_t task1Profiler, task2Profiler;




void Task1(void *pvParameters);
void Task2(void *pvParameters);



sine_generator_q15_t signal_desc;
sine_generator_q15_t noise_signal_desc;

q15_t sine_sig_sample, noise_sig_sample, disturbed_sig_sample;

int main()
{


	/*Enable fpu*/
	 fpu_enable();

	/*Initialize the uart*/
	 uart2_tx_init();

	 xTaskCreate(Task1,
			 	 "Task1",
				 100,
				 NULL,
				 1,
				 NULL);


	 xTaskCreate(Task2,
			 	 "Task2",
				 100,
				 NULL,
				 1,
				 NULL);


    // vTaskStartScheduler();

	 sine_gen_init_q15(&signal_desc,SIGNAL_FREQ,SAMPLING_FREQ);
	 sine_gen_init_q15(&noise_signal_desc,NOISE_FREQ,SAMPLING_FREQ);

	 while(1)
	{
		 sine_sig_sample = sine_calc_sample_q15(&signal_desc)/2;
		 noise_sig_sample = sine_calc_sample_q15(&noise_signal_desc)/2;
		 disturbed_sig_sample =  sine_sig_sample + noise_sig_sample;
		// printf("%d,",(int)sine_sig_sample);

		 printf("%d\n\r",(int)disturbed_sig_sample);
		 pseudo_dly(9000);


		//serial_plot_input_sig();
	}
}

void Task1(void *pvParameters)
{
	while(1)
	{
		//Do somthing...
		task1Profiler++;
	}
}

void Task2(void *pvParameters)
{
	while(1)
	{
		task2Profiler++;
	}
}

static void plot_input_signal(void)
{
	int i;
	for( i = 0; i < KHZ1_15_SIG_LEN; i++)
	{
		g_in_sig_sample = input_signal_f32_1kHz_15kHz[i];
		pseudo_dly(9000);
	}
}

static void serial_plot_input_sig(void)
{
	for(int i = 0; i < KHZ1_15_SIG_LEN; i++)
	{
		printf("%f\r\n",input_signal_f32_1kHz_15kHz[i]);
		pseudo_dly(9000);
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
