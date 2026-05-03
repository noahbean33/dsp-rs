#include <stdio.h>
#include "stm32f4xx.h"
#include "signals.h"
#include "uart.h"
#include "arm_math.h"
#include "systick.h"



#define OFFSET1		5
#define OFFSET2		10

extern float _5hz_signal[HZ_5_SIG_LEN];
extern float32_t input_signal_f32_1kHz_15kHz[KHZ1_15_SIG_LEN] ;
extern float32_t  impulse_response[IMP_RSP_LENGTH];

float32_t output_signal_arr[KHZ1_15_SIG_LEN + IMP_RSP_LENGTH -1];

float g_in_sig_sample;
float g_imp_rsp_sample;

static void plot_input_signal(void);
static void serial_plot_input_sig(void);
static void plot_impulse_response(void);
static void serial_plot_impulse_response(void);
static void serial_plot_output_sig(void);
void serial_plot_all(void);


void convolution(float32_t * sig_src_arr,
				 float32_t * sig_dest_arr,
				 float32_t * imp_response_arr,
				 uint32_t    sig_src_length,
				 uint32_t  imp_response_length);


static void pseudo_dly(int dly);
static void fpu_enable(void);


uint32_t g_before,g_after,g_time_taken;
float32_t  g_seconds,g_milliseconds;

const float SINGLE_CYCLE =  0.0000000625; /*62.5 x 10^-9*/
const int SEC_TO_MSEC	 =  1000;

int main()
{


	/*Enable fpu*/
	 fpu_enable();

	/*Initialize the uart*/
	 uart2_tx_init();

	 /*Initialize systick counter*/
	 systick_counter_init();

/*Our Algo*/

	 /*How long it takes to compute the 'arm_conv_f32' function as is presented in this project
	  * => mumber_of_cycles X time_for_a_single_cycle => 477071  * 62.5x10^-9 = 0.0298169367s = 29.8169365ms  */


	 g_before =  SysTick->VAL;

	 convolution((float32_t *) input_signal_f32_1kHz_15kHz,
	 				( float32_t *) output_signal_arr,
	 				 (float32_t *)impulse_response,
	 				 (uint32_t)   KHZ1_15_SIG_LEN,
	 				 (uint32_t) IMP_RSP_LENGTH);


	 g_after =  SysTick->VAL;

	 /*Compute time taken*/
	 g_time_taken = g_before - g_after;

	 g_seconds =  g_time_taken * SINGLE_CYCLE;

	 g_milliseconds = g_seconds * SEC_TO_MSEC;

/*CMSIS-DSP Algo*/


	 /*How long it takes to compute the 'arm_conv_f32' function as is presented in this project
	  * => mumber_of_cycles X time_for_a_single_cycle => 41160  * 62.5x10^-9 = 0.00257249991s = 2.57249999ms  */

	 arm_conv_f32((float32_t *) input_signal_f32_1kHz_15kHz,
			      (uint32_t)   KHZ1_15_SIG_LEN,
				  (float32_t *)impulse_response,
				  (uint32_t) IMP_RSP_LENGTH,
				  ( float32_t *) output_signal_arr
	             );



	 /*Observation
	  * cmsis_dsp convolution  function  : 2.57249999ms
	  * custom convolution function : 29.8169365ms
	  * 29.8169365ms/ 2.57249999ms = 11.59
	  *
	  * cmsis dsp is 115.59 times faster
	  * */

	while(1)
	{
		 serial_plot_all();

	}
}




void convolution(float32_t * sig_src_arr,
				 float32_t * sig_dest_arr,
				 float32_t * imp_response_arr,
				 uint32_t    sig_src_length,
				 uint32_t  imp_response_length)
{
	uint32_t i,j;

	/*Compute output signal length*/
	uint32_t sig_dest_length = sig_src_length + imp_response_length - 1;

	/*Clear ouput signal buffer*/
	for( i = 0; i < sig_dest_length; i++)
	{
		sig_dest_arr[i] = 0;
	}

	/*Perform convolution*/
	for( i = 0; i < sig_src_length; i++ )
	{
		for( j = 0; j < imp_response_length; j++ )
		{
			sig_dest_arr[i+j] =  sig_dest_arr[i+j] + sig_src_arr[i]*imp_response_arr[j];
		}
	}
}

static void plot_impulse_response(void)
{
	for( int i = 0; i < IMP_RSP_LENGTH; i++ )
	{
		g_imp_rsp_sample =  impulse_response[i];
		pseudo_dly(9000);

	}
}

static void serial_plot_impulse_response(void)
{
	for( int i = 0; i < IMP_RSP_LENGTH; i++ )
	{
		printf("%f\r\n",impulse_response[i]);
		pseudo_dly(9000);

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

static void serial_plot_output_sig(void)
{
	for(int i = 0; i < (KHZ1_15_SIG_LEN + IMP_RSP_LENGTH -1); i++)
	{
		printf("%f\r\n",output_signal_arr[i]);
		pseudo_dly(9000);
	}
}


void serial_plot_all(void)
{
   uint32_t i,j,k;
   i = j = 0;

   for( k = 0; k < (KHZ1_15_SIG_LEN + IMP_RSP_LENGTH -1); k++ )
   {
	   i++;
	   j++;
	   if( i == KHZ1_15_SIG_LEN ) {

		   i = 0;
	   }
	   if( j == IMP_RSP_LENGTH ) {

		   j = 0;
		}

	   printf("%f,",OFFSET1 + input_signal_f32_1kHz_15kHz[i]);
	   printf("%f,",OFFSET2 + impulse_response[j]);
	   printf("%f\n\r",output_signal_arr[k]);
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
