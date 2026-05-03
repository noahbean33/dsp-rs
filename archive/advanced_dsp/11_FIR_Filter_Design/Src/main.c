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
#include "fir_filter.h"


static void pseudo_dly(int dly);
static void fpu_enable(void);

uint32_t task1Profiler, task2Profiler,task3Profiler, task4Profiler;


void acquisition_tsk(void *pvParameters);
void processing_tsk(void *pvParameters);
void display_tsk(void *pvParameters);


SemaphoreHandle_t xBinarySemaphore;

const TickType_t _10ms =  pdMS_TO_TICKS(10);

uint16_t g_sensor_value;
float32_t f32_sensor_value;

fir_filter_type lpf_fir;
float32_t fltr_sensor_value;


extern float LP_1HZ_2HZ_IMPULSE_RESPONSE[IMP_RSP_LENGTH];

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





	 	 lowpass_filter_init();

    vTaskStartScheduler();


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

	fir_filter_init(&lpf_fir, LP_1HZ_2HZ_IMPULSE_RESPONSE, IMP_RSP_LENGTH);
	while(1)
	{
		xSemaphoreTake(xBinarySemaphore,portMAX_DELAY);

		task2Profiler++;

		/*Perform DSP*/
		fltr_sensor_value  = fir_filter_exec(&lpf_fir, g_sensor_value);

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
		printf("%f\n\r",fltr_sensor_value);

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
