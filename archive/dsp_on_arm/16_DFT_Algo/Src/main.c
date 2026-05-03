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

float32_t output_signal_arr[KHZ1_15_SIG_LEN];

float REX[KHZ1_15_SIG_LEN/2];
float IMX[KHZ1_15_SIG_LEN/2];

float g_in_sig_sample;
float g_imp_rsp_sample;

static void plot_input_signal(void);
static void serial_plot_input_sig(void);
static void plot_impulse_response(void);
static void serial_plot_impulse_response(void);
static void serial_plot_output_sig(void);
void serial_plot_all(void);



void calc_sig_dft(float32_t *sig_src_arr,
				  float32_t *sig_dest_rex_arr,
				  float32_t *sig_dest_imx_arr,
				  uint32_t sig_length);

static void pseudo_dly(int dly);
static void fpu_enable(void);

void serial_plot_rex(void);
void get_dft_output_mag(void);


int main()
{


	/*Enable fpu*/
	 fpu_enable();

	/*Initialize the uart*/
	 uart2_tx_init();

	 /*Initialize systick counter*/
	 systick_counter_init();


	  calc_sig_dft((float32_t*)input_signal_f32_1kHz_15kHz,
	 				  (float32_t *)REX,
	 				  (float32_t *)IMX,
	 				  (uint32_t)KHZ1_15_SIG_LEN);


	  get_dft_output_mag();

	  serial_plot_rex();

	while(1)
	{


	}
}




void serial_plot_rex(void)
{

	for( int j = 0; j < (KHZ1_15_SIG_LEN/2); j++)
	{
		printf("%f\r\n",REX[j]);
		pseudo_dly(9000);

	}
}


void get_dft_output_mag(void)
{
	for( int k = 0; k < (KHZ1_15_SIG_LEN/2); k++)
	{
		REX[k] = fabs(REX[k]);
	}
}
void calc_sig_dft(float32_t *sig_src_arr,
				  float32_t *sig_dest_rex_arr,
				  float32_t *sig_dest_imx_arr,
				  uint32_t sig_length)
{
	int i,k,j;

	/*Clear destination buffers*/
	for( j = 0; j < (sig_length/2); j++)
	{
		sig_dest_rex_arr[j] = 0;
		sig_dest_imx_arr[j] = 0;
	}

	/*Compute DFT*/
	for( k = 0; k < (sig_length/2); k++ )
	{
	    for( i = 0; i < sig_length; i++)
	    {
	    	sig_dest_rex_arr[k] = sig_dest_rex_arr[k] +sig_src_arr[i]*cos(2*PI*k*i/sig_length);
	    	sig_dest_imx_arr[k] = sig_dest_imx_arr[k] +sig_src_arr[i]*sin(2*PI*k*i/sig_length);

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

   for( int k = 0; k < KHZ1_15_SIG_LEN ; k++ )
   {


	   printf("%f,",OFFSET1 + input_signal_f32_1kHz_15kHz[k]);
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
