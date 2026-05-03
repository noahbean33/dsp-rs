#include <stdio.h>
#include "stm32f4xx.h"
#include "signals.h"
#include "uart.h"
#include "arm_math.h"
#include "FreeRTOS.h"
#include "task.h"




extern float _5hz_signal[HZ_5_SIG_LEN];
extern float32_t input_signal_f32_1kHz_15kHz[KHZ1_15_SIG_LEN] ;

float g_in_sig_sample;

static void plot_input_signal(void);
static void serial_plot_input_sig(void);

static void pseudo_dly(int dly);
static void fpu_enable(void);

#define BLOCK_SIZE 5

uint32_t task1Profiler, task2Profiler;



void Task1(void *pvParameters);
void Task2(void *pvParameters);


uint32_t src_sensor_data[BLOCK_SIZE] = {2043,1023,990,67,3013};

float32_t f32_sensor_data[BLOCK_SIZE];
q31_t q31_sensor_data[BLOCK_SIZE];
float32_t f32_dest_sensor_data[BLOCK_SIZE];
uint32_t u32_dest_sensor_data[BLOCK_SIZE];


int main()
{


	/*Enable fpu*/
	 fpu_enable();

	/*Initialize the uart*/
	 uart2_tx_init();


	 /*Scale ADC values to  between -1 and +1*/
	 for(int i=0; i< BLOCK_SIZE; i++)
	 {
	      f32_sensor_data[i] = ((float32_t)(src_sensor_data[i] & 0xFFF)/(0xFFF/2))-1;
	 }
	 /*Convert float to q*/
	 arm_float_to_q31(f32_sensor_data,q31_sensor_data,BLOCK_SIZE);

	 /*Perform DSP*/
	 /*Do something..*/

	 /*Convert q to float*/
	 arm_q31_to_float(q31_sensor_data,f32_dest_sensor_data,BLOCK_SIZE);

	 /*Convert float to uint32_t*/
	 float32_t temp_data;

	 for(int i = 0; i <BLOCK_SIZE; i++)
	 {
		 temp_data =(f32_dest_sensor_data[i] + 1)*(0xFFF/2);

		 /*Round to nearest integer*/
		 u32_dest_sensor_data[i] = (uint32_t)(temp_data + 0.5);

	 }

	 while(1)
	{
		serial_plot_input_sig();
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
