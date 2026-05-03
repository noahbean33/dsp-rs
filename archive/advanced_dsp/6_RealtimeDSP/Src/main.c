#include <stdio.h>
#include "stm32f4xx.h"
#include "signals.h"
#include "uart.h"
#include "arm_math.h"
#include "FreeRTOS.h"
#include "task.h"
#include "sine_generator.h"
#include "lowpass_fltr.h"
#include "semphr.h"


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

uint32_t task1Profiler, task2Profiler,task3Profiler, task4Profiler;




void acquisition_tsk(void *pvParameters);
void noise_gen_tsk(void *pvParameters);
void combiner_tsk(void *pvParameters);
void filtered_sig_tsk(void *pvParameters);



sine_generator_q15_t signal_desc;
sine_generator_q15_t noise_signal_desc;

q15_t sine_sig_sample, noise_sig_sample, disturbed_sig_sample, fltr_sig_sample;

SemaphoreHandle_t xBinarySemaphore;

const TickType_t _10ms =  pdMS_TO_TICKS(10);
int main()
{


	/*Enable fpu*/
	 fpu_enable();

	/*Initialize the uart*/
	 uart2_tx_init();

	 xBinarySemaphore =  xSemaphoreCreateBinary();

	 xTaskCreate(acquisition_tsk,
			 	 "Sine signal Task",
				 256,
				 NULL,
				 1,
				 NULL);


	 xTaskCreate(noise_gen_tsk,
			 	 "Noise Signal Task",
				 256,
				 NULL,
				 1,
				 NULL);

	 xTaskCreate(combiner_tsk,
			 	 "Disturbed  Signal Task",
				 256,
				 NULL,
				 1,
				 NULL);


	 xTaskCreate(filtered_sig_tsk,
			 	 "Filtered  Signal Task",
				 256,
				 NULL,
				 1,
				 NULL);



	 	 lowpass_filter_init();
	 	 sine_gen_init_q15(&signal_desc,SIGNAL_FREQ,SAMPLING_FREQ);
	 	 sine_gen_init_q15(&noise_signal_desc,NOISE_FREQ,SAMPLING_FREQ);

    vTaskStartScheduler();


	 while(1)
	{
//
//
//		 //printf("%d,", 300 +(int)sine_sig_sample);
//		 //printf("%d,", 200+ (int)noise_sig_sample);
//		 printf("%d,",100+(int)disturbed_sig_sample);
//		 printf("%d\n\r",(int)fltr_sig_sample);
//		 pseudo_dly(9000);
//
//
//		//serial_plot_input_sig();
	}
}

void acquisition_tsk(void *pvParameters)
{

	xSemaphoreGive(xBinarySemaphore);

	while(1)
	{
		xSemaphoreTake(xBinarySemaphore,portMAX_DELAY);

		//Do somthing...

	    sine_sig_sample = sine_calc_sample_q15(&signal_desc)/2;

		task1Profiler++;

		xSemaphoreGive(xBinarySemaphore);
		vTaskDelay(_10ms);

	}
}

void noise_gen_tsk(void *pvParameters)
{
	while(1)
	{
		xSemaphoreTake(xBinarySemaphore,portMAX_DELAY);

		task2Profiler++;
	    noise_sig_sample = sine_calc_sample_q15(&noise_signal_desc)/6;

		xSemaphoreGive(xBinarySemaphore);
		vTaskDelay(_10ms);

	}
}

void combiner_tsk(void *pvParameters)
{
	while(1)
	{
		xSemaphoreTake(xBinarySemaphore,portMAX_DELAY);

		task3Profiler++;
		 disturbed_sig_sample =  sine_sig_sample + noise_sig_sample;

		xSemaphoreGive(xBinarySemaphore);
		vTaskDelay(_10ms);


	}
}

void filtered_sig_tsk(void *pvParameters)
{
	while(1)
	{
		xSemaphoreTake(xBinarySemaphore,portMAX_DELAY);
		task4Profiler++;
		fltr_sig_sample  = lowpass_filter_exec(&disturbed_sig_sample);
		printf("%d\n\r",(int)fltr_sig_sample);
		xSemaphoreGive(xBinarySemaphore);
		vTaskDelay(_10ms);


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
