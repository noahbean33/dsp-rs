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
#include "tim.h"
#include "fifo.h"




static void pseudo_dly(int dly);
static void fpu_enable(void);

uint32_t task1Profiler, task2Profiler,task3Profiler, task4Profiler,TIM2Profiler;


extern float LP_1HZ_2HZ_IMPULSE_RESPONSE[IMP_RSP_LENGTH];


void acquisition_tsk(void *pvParameters);
void processing_tsk(void *pvParameters);
void display_tsk(void *pvParameters);



#define INPUT_SIG_LEN		RXFIFOSIZE

SemaphoreHandle_t xBinarySemaphore;

const TickType_t _10ms =  pdMS_TO_TICKS(10);

uint16_t g_sensor_value;
float32_t f32_sensor_value;
q15_t q15_sensor_value;
q15_t fltr_sig_sample;


uint8_t g_process_flg;
uint8_t g_fifo_full_flg;

static uint8_t read_fifo(rx_dataType *data_buff);
rx_dataType sensor_data_buffer[RXFIFOSIZE];
float32_t  f32_sensor_data_buffer[RXFIFOSIZE];
float32_t  f32_output_signal_buffer[INPUT_SIG_LEN + IMP_RSP_LENGTH - 1];

static void clear_data_buffer(void);


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


	 /*Init fifo*/
	 rx_fifo_init();

	 /*TIM2 interrupt init*/
	 tim2_1khz_interrupt_init();



	 g_fifo_full_flg = 1;

	 xBinarySemaphore =  xSemaphoreCreateBinary();




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

void acquisition_func(void)
{

      if(g_fifo_full_flg == 1)/*Not full*/
      {
    	  g_fifo_full_flg=   rx_fifo_put(adc_read());
      }
      else /*Full*/
      {
    	  /*Start processing thread*/
    	  g_process_flg = 1;
      }


}

void processing_tsk(void *pvParameters)
{
	while(1)
	{
		//xSemaphoreTake(xBinarySemaphore,portMAX_DELAY);

		task2Profiler++;


		if(g_process_flg)
		{
		/*1. clear buffer */
		clear_data_buffer();

		/*2. Read fifo content into data buffer*/
		for(int i = 0; i < RXFIFOSIZE; i++)
		{
			/*Wait until entire batch us collected*/
			while(g_fifo_full_flg == 1){}

			/*read data into data buff*/
			g_fifo_full_flg = read_fifo(sensor_data_buffer + i);

		}
		 /*3. Scale ADC values to  between -1 and +1*/
		for(int i = 0; i < RXFIFOSIZE; i++ ){

			f32_sensor_data_buffer[i] = ((float32_t)(sensor_data_buffer[i]& 0xFFF)/(0xFFF/2))-1;
		}


		/*4. Perform DSP*/
		arm_conv_f32((float32_t *)f32_sensor_data_buffer,
					(uint32_t) INPUT_SIG_LEN,
					(float32_t *) LP_1HZ_2HZ_IMPULSE_RESPONSE,
					(uint32_t)IMP_RSP_LENGTH,
					(float32_t *)f32_output_signal_buffer);



		 g_process_flg = 0;

		xSemaphoreGive(xBinarySemaphore);

		}
	}
}

void display_tsk(void *pvParameters)
{
	while(1)
	{
		xSemaphoreTake(xBinarySemaphore,portMAX_DELAY);

		task3Profiler++;

	     /*Plot original and filtered signals*/
		for(int i = 0; i <  (INPUT_SIG_LEN + IMP_RSP_LENGTH - 1); i++)
		{
			if(i  < INPUT_SIG_LEN)
			{
				printf("%f,",f32_sensor_data_buffer[i]);
			}

			printf("%f\n\r",f32_output_signal_buffer[i]);
		}

		//vTaskDelay(_10ms);


	}
}


static uint8_t read_fifo(rx_dataType *data_buff)
{
	__IO uint8_t rd_flg;

	/*Place fifo data into the data_buff*/
	rd_flg = rx_fifo_get(data_buff);


	/*if fifo empty teh we reset g_fifo_full_flg*/
	if(rd_flg == 0)
	{
		/*Will start the fifo_put routine to
		 * collect the next batch of samples*/
		g_fifo_full_flg =  1;
	}

	/*Keep fifo full flag at full*/
	else
	{
		g_fifo_full_flg = 0;
	}

	return g_fifo_full_flg;
}


static void clear_data_buffer(void)
{
	for(int i=0; i <  RXFIFOSIZE;i++)
	{
		sensor_data_buffer[i] = 0;
		f32_sensor_data_buffer[i] = 0.0;
	}
}


void TIM2_IRQHandler(void)
{
	acquisition_func();

	TIM2Profiler++;


	/*Clear update interrupt flag*/
	TIM2->SR &=~SR_UIF;

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
