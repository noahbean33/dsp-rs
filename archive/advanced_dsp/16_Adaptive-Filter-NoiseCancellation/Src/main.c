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





#define SAMPLE_RATE				1000
#define SIGNAL_FREQUENCY		5.0
#define NOISE_FREQUENCY			50.0
#define LEARNING_RATE			0.01
#define NUM_WEIGHTS				200



float weights[NUM_WEIGHTS] = {0};
float filter_delay_line[NUM_WEIGHTS] = {0};

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


float read_desired_signal(void);
float read_sensor_value(void);


float fltr_value;
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



	 while(1)
	{


		 /*Read sensor value (input signal with noise)*/
		 float noisy_signal = read_sensor_value();

		 /*Read desired signal(clean signal without noise)*/
		 float desired_signal =  read_desired_signal();

		 /*Execute adaptive filter*/

		 /*shift delay line*/
		 for( int i =  NUM_WEIGHTS -1; i > 0; i--)
		 {
			 filter_delay_line[i] =  filter_delay_line[i -1];
		 }

		 filter_delay_line[0] =  noisy_signal;

		 /*Compute filter output*/
		 float filter_output =  0.0;
		 for(int i = 0;  i <  NUM_WEIGHTS; i++)
		 {
			 filter_output +=weights[i] * filter_delay_line[i];
		 }

		 /*Compute error*/
		 float error =  desired_signal -  filter_output;

		 /*Update the weights using LMS algorithm*/
		 for(int i =0; i < NUM_WEIGHTS; i++)
		 {
			 weights[i] += LEARNING_RATE *error *filter_delay_line[i];
		 }

		 /*Plot results*/
		 printf("%f,", noisy_signal);
		 printf("%f,", filter_output);
		 printf("%f\r\n", desired_signal);


		 pseudo_dly(90000);

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


float read_sensor_value(void)
{

	/*NOTE:  Replace implementation of reading a sensor generation
	 * signal of a particular frequency and noise*/

	static float theta =  0.0;

	theta +=2.0 * M_PI *NOISE_FREQUENCY/SAMPLE_RATE;
	if(theta > 2.0 * M_PI)
	{
		theta -= 2.0 * M_PI;
	}

	/*Simulated sensor value with noise*/
	return sin(theta) + ((rand() % 200)/100.0)-1.0;
}


float read_desired_signal(void)
{
	static float theta =  0.0;
	theta +=2.0 * M_PI *SIGNAL_FREQUENCY/SAMPLE_RATE;

	if(theta > 2.0 * M_PI)
	{
		theta -= 2.0 * M_PI;
	}
	return sin(theta);

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
