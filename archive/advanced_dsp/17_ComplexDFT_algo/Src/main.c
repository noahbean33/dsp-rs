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
#include "adc.h"
#include "iir_df2_sos.h"
#include "_1kHz_20kHz_sine.h"
#include "lms.h"





#define NUM_SAMP 		  100
#define SIGNAL_FREQUENCY  100.0
#define SAMPLING_RATE	  1000


static void pseudo_dly(int dly);
static void fpu_enable(void);

uint32_t task1Profiler, task2Profiler,task3Profiler, task4Profiler;


void acquisition_tsk(void *pvParameters);
void processing_tsk(void *pvParameters);
void display_tsk(void *pvParameters);


SemaphoreHandle_t xBinarySemaphore;

const TickType_t _10ms =  pdMS_TO_TICKS(10);

uint16_t g_sensor_value;


float32_t f32_fltr_sensor_value;



typedef struct{
	float real; //Real part
	float imag; // Imaginary part
}complex_number;





complex_number samples[NUM_SAMP];

void discrete_fourier_transform(complex_number *x)
{
	complex_number result[NUM_SAMP];

	for(int k = 0; k <  NUM_SAMP; k++)
	{
		result[k].real =  0.0;
		result[k].imag =  0.0;


		/*Compute DFT for each frequency*/
		for(int n =0;  n < NUM_SAMP; n++ )
		{
			/*Update real and imaginary parts*/
			result[k].real += x[n].real * cosf(2 * PI * k * n /NUM_SAMP)
							- x[n].imag * sinf(2 * PI * k * n /NUM_SAMP);

			result[k].imag += x[n].imag * cosf(2 * PI * k * n /NUM_SAMP)
							+ x[n].real * sinf(2 * PI * k * n /NUM_SAMP);


		}



	}

	/*Copy the computed DFT result back to the input array*/
	for(int k =0; k < NUM_SAMP;k++)
	{
		x[k] =  result[k];
	}

}


int main()
{


	/*Enable fpu*/
	 fpu_enable();

	/*Initialize the uart*/
	 uart2_tx_init();


	 /*Initialize adc*/
	 pa1_adc_init();

	 /*Start_conversion*/
	 start_conversion();

	 xBinarySemaphore =  xSemaphoreCreateBinary();

	 xTaskCreate(acquisition_tsk,
			 	 "Acquisition Task",
				 256,
				 NULL,
				 1,
				 NULL);


	 xTaskCreate(processing_tsk,
			 	 "Processing Task",
				 256,
				 NULL,
				 1,
				 NULL);

	 xTaskCreate(display_tsk,
			 	 "Display Task",
				 256,
				 NULL,
				 1,
				 NULL);


    //vTaskStartScheduler();

    /*Generate a test signal 100hz*/
	 for(int n = 0;  n < NUM_SAMP; n++ )
	 {
		 samples[n].real =  cos(2 * PI* SIGNAL_FREQUENCY * n / SAMPLING_RATE);
		 samples[n].imag = 0.0;
	 }


   /*Perform DFT on samples*/
	 discrete_fourier_transform(samples);

	 /*Plot results*/
	 for(int j =0; j < (NUM_SAMP/2); j++)
	 {
		 printf("%f\r\n", samples[j].real);
		 pseudo_dly(90000);
	 }

	 while(1)
	{




	}
}

void acquisition_tsk(void *pvParameters)
{

	xSemaphoreGive(xBinarySemaphore);

	while(1)
	{
		xSemaphoreTake(xBinarySemaphore,portMAX_DELAY);

		//Do somthing...


		g_sensor_value =  adc_read();


		task1Profiler++;

		xSemaphoreGive(xBinarySemaphore);
		vTaskDelay(_10ms);

	}
}

void processing_tsk(void *pvParameters)
{
	while(1)
	{
		xSemaphoreTake(xBinarySemaphore,portMAX_DELAY);

		task2Profiler++;


		f32_fltr_sensor_value  = iir_df2_sos_exec(g_sensor_value);


		xSemaphoreGive(xBinarySemaphore);
		vTaskDelay(_10ms);

	}
}

void display_tsk(void *pvParameters)
{
	while(1)
	{
		xSemaphoreTake(xBinarySemaphore,portMAX_DELAY);

		task3Profiler++;

		printf("%d,",(int)g_sensor_value);
		printf("%0.10f\n\r",f32_fltr_sensor_value);

		xSemaphoreGive(xBinarySemaphore);
		vTaskDelay(_10ms);


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
