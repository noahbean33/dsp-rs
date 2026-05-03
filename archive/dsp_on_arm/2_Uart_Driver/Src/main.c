#include <stdio.h>
#include "stm32f4xx.h"
#include "signals.h"
#include "uart.h"




extern float _5hz_signal[HZ_5_SIG_LEN];
float g_in_sig_sample;

static void plot_input_signal(void);
static void pseudo_dly(int dly);

int main()
{

	/*Enable Floating Point Unit :  Enable CP10 and CP11 full access*/
	SCB->CPACR |= (1U<<20);
	SCB->CPACR |= (1U<<21);
	SCB->CPACR |= (1U<<22);
	SCB->CPACR |= (1U<<23);

	/*Initialize the uart*/
	uart2_tx_init();


	while(1)
	{
		printf("Hello from STM32....\n\r");

		//plot_input_signal();
	}
}

static void plot_input_signal(void)
{
	int i;
	for( i = 0; i < HZ_5_SIG_LEN; i++)
	{
		g_in_sig_sample = _5hz_signal[i];
		pseudo_dly(9000);
	}
}

static void pseudo_dly(int dly)
{
	for( int i = 0; i < dly; i++ ){}
}
