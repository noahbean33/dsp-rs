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



extern float32_t input_signal_f32_1kHz_15kHz[KHZ1_15_SIG_LEN];
extern float32_t _640_points_ecg_[ECG_SIG_LENGTH];


#define NUM_SAMP 		  128
#define SIGNAL_FREQUENCY  100.0
#define SAMPLING_RATE	  1000


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



typedef struct{
	float real; //Real part
	float imag; // Imaginary part
}complex_number;





complex_number samples[NUM_SAMP];
complex_number twiddle_factors[NUM_SAMP];


void fast_fourier_transform(complex_number *signal_samples, int num_samples, complex_number *twiddles)
{

    /*Temp variable to store complex numbers during calculation*/
    complex_number temp1, temp2;

    /*Indices for the butterfly computation*/
    int upper_leg, lower_leg;

    /*Difference between the indices of the butterfly*/
    int leg_diff;

    /*Number of stages in the FFT, determined log2(number of samples)*/
    int num_stages = 0;

    /*Index and step used for accessing twiddle factors*/
    int index, step;

    /*Loop counters*/
    int i, j, k;

    /*Determine number of stages*/
    i =1;

    do{
    	num_stages  +=1;
    	i = i*2;
    }while( i != num_samples);


    /*Initial variables for the butterfly computation*/
    leg_diff =  num_samples /2;
    step =  2;

    /*Main FFT Computation*/
    for( i = 0; i <  num_stages;  i++){

    	index = 0;

    	for( j = 0; j < leg_diff; j++)
    	{
    		for(upper_leg = j; upper_leg < num_samples; upper_leg += (2* leg_diff))
    		{
    			lower_leg = upper_leg + leg_diff;

    			/*Buttefly computations*/
    			temp1.real = signal_samples[upper_leg].real +  signal_samples[lower_leg].real;
    			temp1.imag = signal_samples[upper_leg].imag +  signal_samples[lower_leg].imag;

    			temp2.real = signal_samples[upper_leg].real -  signal_samples[lower_leg].real;
    			temp2.imag = signal_samples[upper_leg].imag -  signal_samples[lower_leg].imag;

    			signal_samples[lower_leg].real =  temp2.real * twiddles[index].real - temp2.imag * twiddles[index].imag;
    			signal_samples[lower_leg].imag =  temp2.real * twiddles[index].imag + temp2.imag * twiddles[index].real;

    			signal_samples[upper_leg].real = temp1.real;
    			signal_samples[upper_leg].imag = temp1.imag;

    		}

    		index += step;
    	}

    	leg_diff = leg_diff/2;
    	step *=2;

    }

    /*Bit bit reversal reordering of the output data for in-place FFT*/
    j = 0;

     for( i = 1; i < (num_samples - 1);  i++)
     {
    	  k  = num_samples /2;

    	  while( k <= j)
    	  {
    		  j = j - k;
    		  k = k /2;
    	  }
    	   j = j + k;

    	   if( i < j)
    	   {
    		   /*Swap elements at indices i and j */
    		   temp1.real =  signal_samples[j].real;
    		   temp1.imag =  signal_samples[j].imag;

    		   signal_samples[j].real = signal_samples[i].real;
    		   signal_samples[j].imag = signal_samples[i].imag;

    		   signal_samples[i].real = temp1.real;
    		   signal_samples[i].imag = temp1.imag;

    	   }
     }

}


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


	 /*Precompute the twiddle factors*/
	 for(int n =0; n < NUM_SAMP; n++)
	 {
		 twiddle_factors[n].real =  cos( PI * n / NUM_SAMP);
		 twiddle_factors[n].imag =  -sin(PI * n / NUM_SAMP);

	 }


    /*Generate*/
	 for(int n = 0;  n < NUM_SAMP; n++ )
	 {
		 samples[n].real = cos(2 * PI* SIGNAL_FREQUENCY * n / SAMPLING_RATE);
		 samples[n].imag = 0.0;
	 }


   /*Perform FFT on samples*/
    fast_fourier_transform(samples,NUM_SAMP,twiddle_factors);


	 /*Plot time domain results*/
//	 for(int j =0; j <  ECG_SIG_LENGTH; j++)
//	 {
//		 printf("%f\r\n", _640_points_ecg_[j]);
//		 pseudo_dly(90000);
//	 }

	 /*Plot results*/
	 for(int j =0; j < (NUM_SAMP/2); j++)
	 {
		 printf("%f\r\n", samples[j].real);
		 pseudo_dly(90000);
	 }

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
