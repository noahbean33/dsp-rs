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


#define OFFSET		5

extern  float LP_1HZ_2HZ_IMPULSE_RESPONSE[IMP_RSP_LENGTH];

float32_t ouput_signal_arr[KHZ1_15_SIG_LEN];

float g_in_sig_sample;



static void pseudo_dly(int dly);
static void fpu_enable(void);


uint32_t my_rx_data[50];

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
	 tim2_1hz_interrupt_init();

	 /*Start ADC conversion*/
	 start_conversion();

	 /*Initializr fifo*/
	 rx_fifo_init();


	while(1)
	{



	}
}


static void tim2_callback(void)
{
	printf("A second just passed!! \n\r");
	//GPIOA->ODR ^=(1U<<8);
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
