#include <stdio.h>
#include "stm32f4xx.h"
#include "signals.h"
#include "uart.h"
#include "arm_math.h"
#include "adc.h"
#include "fir_filter.h"
#include "fifo.h"
#include "tim.h"
#include "clock.h"


#define OFFSET		10

#define INPUT_SIG_LEN 	RXFIFOSIZE

extern  float LP_1HZ_2HZ_IMPULSE_RESPONSE[IMP_RSP_LENGTH];

uint32_t ouput_signal_arr[INPUT_SIG_LEN ];

uint8_t g_fifo_full_flg, g_process_flg;


static void pseudo_dly(int dly);
static void fpu_enable(void);
static void clear_data_buffer(void);


rx_dataType sensor_data_buffer[RXFIFOSIZE];
static uint8_t read_fifo(rx_dataType *data_buff);

uint32_t bk_thread_profiler;


void moving_average(uint32_t * sig_src_arr,
		            uint32_t * sig_out_arr,
					uint32_t signal_length,
					uint32_t filter_pts);
int main()
{

    /*DEBUG*/
	RCC->AHB1ENR |=(1U<<0);

	GPIOA->MODER |=(1U <<16);

	/*Enable fpu*/
	 fpu_enable();

	 /*Configure clock tree*/
	 clock_100MHz_config();

	/*Initialize the uart*/
	 uart2_tx_init();

    /*Initialize ADC*/
	 pa1_adc_init();

	 /*Enable background thread*/
	 tim2_1khz_interrupt_init();

	 /*Start ADC conversion*/
	 start_conversion();

	 /*Initializr fifo*/
	 rx_fifo_init();

	 bk_thread_profiler = 0;

	while(1)
	{

		 /*If g_process_flg is set to 1, do dsp processing*/
		if( g_process_flg )
		{
		   /*1. Reset data buffer*/
			clear_data_buffer();

			/*2. Read fifo content into data buffer*/
			for( int i = 0; i < RXFIFOSIZE; i++)
			{
				/*Wait until entire batch is collected from adc*/
				while(g_fifo_full_flg == 1){}

				/*read data into data_buff*/
				g_fifo_full_flg = read_fifo(sensor_data_buffer +i);

			}

			/*Perform digital signal procession*/

			   moving_average((uint32_t *) sensor_data_buffer,
					            (uint32_t *) ouput_signal_arr,
								(uint32_t) INPUT_SIG_LEN,
								(uint32_t) 11);

			for( int i = 0 ;  i < INPUT_SIG_LEN ; i++)
			{

				   printf("%d,",(int) (OFFSET +sensor_data_buffer[i]));

				 printf("%d\n\r",(int)ouput_signal_arr[i]);
			}

			/*Reset process flag*/
			g_process_flg = 0;
		}


	}
}

static uint8_t read_fifo(rx_dataType *data_buff)
{
	__IO uint8_t rd_flg;

	/*place fifo data into dat_buff*/
	rd_flg = rx_fifo_get(data_buff);

	/*if fifo is empty then reset g_fifo_full_flag*/
	if(rd_flg == 0)
	{
		/*This will start the fifo_put routine again
		 * to collect the next batch of samples*/
		g_fifo_full_flg = 1;
	}
	else{
		/*keep fifo g_fifo_full_flg at fifo FULL*/
		g_fifo_full_flg = 0;
	}

	return g_fifo_full_flg;

}


static void clear_data_buffer(void)
{
  for(int i = 0; i < RXFIFOSIZE; i++ )
  {
	  sensor_data_buffer[i] = 0;
  }
}
static void tim2_callback(void)
{
  /*Check if fifo is not full*/

	bk_thread_profiler++;

	if(g_fifo_full_flg == 1) /*Not full*/
	{
		g_fifo_full_flg = rx_fifo_put(adc_read());
	}
	else /*Full*/
	{
		g_process_flg = 1;
	}

}

void TIM2_IRQHandler(void)
{
	/*Clear update interrupt flag*/
	TIM2->SR &=~SR_UIF;

	/*Do something...*/
	tim2_callback();
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

void moving_average(uint32_t * sig_src_arr,
		            uint32_t * sig_out_arr,
					uint32_t signal_length,
					uint32_t filter_pts)
{

	int i,j;

	for( i =  floor(filter_pts/2) ; i < (signal_length - (filter_pts/2))-1; i++)
	{
		sig_out_arr[i]= 0;

		for( j = - (floor(filter_pts/2)); j < floor(filter_pts/2); j++ )
		{
			sig_out_arr[i] = sig_out_arr[i] + sig_src_arr[i+j];
		}

		sig_out_arr[i]=  sig_out_arr[i]/filter_pts;
	}

}
