#include <stdio.h>
#include "stm32f4xx.h"
#include "signals.h"
#include "uart.h"
#include "arm_math.h"
#include "systick.h"
#include "clock.h"


#define OFFSET1		5
#define OFFSET2		10

extern float _5hz_signal[HZ_5_SIG_LEN];
extern float32_t input_signal_f32_1kHz_15kHz[KHZ1_15_SIG_LEN] ;
extern float32_t  impulse_response[IMP_RSP_LENGTH];
extern float32_t _640_points_ecg_[ECG_SIG_LENGTH];
extern float32_t _10Hz_100Hz_500Hz_signal[_10HZ_100HZ_500HZ_SIG_LEN];
extern float32_t lpf_70hz_impulse_response[LPF_70HZ_IMP_RESP_LEN];

float32_t output_signal_arr[_10HZ_100HZ_500HZ_SIG_LEN + LPF_70HZ_IMP_RESP_LEN -1];
float32_t idft_output_signal_arr[ECG_SIG_LENGTH];

float REX[ECG_SIG_LENGTH/2];
float IMX[ECG_SIG_LENGTH/2];

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


void calc_sig_idft(float32_t * idft_out_arr,
		           float32_t *sig_src_rex_arr,
				   float32_t *sig_src_imx_arr,
				   uint32_t idft_length);


void convolution(float32_t * sig_src_arr,
				 float32_t * sig_dest_arr,
				 float32_t * imp_response_arr,
				 uint32_t    sig_src_length,
				 uint32_t  imp_response_length);


static void pseudo_dly(int dly);
static void fpu_enable(void);

void serial_plot_rex(void);
void get_dft_output_mag(void);
void serial_plot_ecg_signal(void);
void serial_plot_orig_sig_and_idft_sig(void);


int main()
{


	/*Enable fpu*/
	 fpu_enable();

	 /*Set clock tree*/
	 clock_100MHz_config();

	/*Initialize the uart*/
	 uart2_tx_init();

	 convolution((float32_t *) _10Hz_100Hz_500Hz_signal,
	 				( float32_t *) output_signal_arr,
	 				 (float32_t *)lpf_70hz_impulse_response,
	 				 (uint32_t)   _10HZ_100HZ_500HZ_SIG_LEN,
	 				 (uint32_t) LPF_70HZ_IMP_RESP_LEN);



	while(1)
	{

		 serial_plot_output_sig();

	}
}



void serial_plot_orig_sig_and_idft_sig(void)
{
	int i;

	for( i = 0; i < ECG_SIG_LENGTH; i++)
	{
		printf("%f,",OFFSET1 +_640_points_ecg_[i] );
		printf("%f\n\r",idft_output_signal_arr[i] );
		pseudo_dly(9000);

	}
}

void serial_plot_ecg_signal(void)
{
	for( int i = 0; i < ECG_SIG_LENGTH; i++ )
	{
		printf("%f\n\r",_640_points_ecg_[i]);
		pseudo_dly(9000);

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


void calc_sig_idft(float32_t * idft_out_arr,
		           float32_t *sig_src_rex_arr,
				   float32_t *sig_src_imx_arr,
				   uint32_t idft_length)
{

	 int i,k;

	 /*Normalize amplitudes*/
	 for( k = 0; k < idft_length/2; k++)
	 {
		 sig_src_rex_arr[k] =  sig_src_rex_arr[k]/(idft_length/2);
		 sig_src_imx_arr[k] =  -sig_src_imx_arr[k]/(idft_length/2);

	 }

	 /*Deal with idx 0*/
	 sig_src_rex_arr[0] =  sig_src_rex_arr[0]/(idft_length);
	 sig_src_imx_arr[0] =  -sig_src_imx_arr[0]/(idft_length);

	 /*Deal with idx n/2*/
	 sig_src_rex_arr[idft_length/2] =  sig_src_rex_arr[idft_length/2]/(idft_length);
	 sig_src_imx_arr[idft_length/2] =  -sig_src_imx_arr[idft_length/2]/(idft_length);

	 /*Clear output buffer*/
	 for( i = 0; i < idft_length; i++)
	 {
		 idft_out_arr[i] = 0;
	 }

	 /*Perform IDFT*/
	 for( k = 0; k < idft_length/2;k++)
	 {
		 for( i = 0; i <idft_length; i++)
		 {
			 idft_out_arr[i] =  idft_out_arr[i] + sig_src_rex_arr[k]*cos(2*PI*k*i/idft_length);
			 idft_out_arr[i] =  idft_out_arr[i] + sig_src_imx_arr[k]*sin(2*PI*k*i/idft_length);

		 }
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
	for(int i = 0; i < (_10HZ_100HZ_500HZ_SIG_LEN + LPF_70HZ_IMP_RESP_LEN -1); i++)
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
