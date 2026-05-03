#include "elliptic.h"
#include "iir_df2_sos.h"


float w[NUM_SECTIONS][2] = {0};

float iir_df2_sos_exec(int16_t sample)
{
	int16_t section;
	float input;
	float wn, yn;
	float filtered_value;

	input  = (float)(sample);


	 /*Loop through each section of the filter*/
	for(section = 0;  section < NUM_SECTIONS; section++)
	{
		/*Calculat wn, the intermediat value, using previous w values and input*/
		wn =  input- a[section][1]*w[section][0] - a[section][2]*w[section][1];

		/*Calculate yn, the output value, using the calculated wn and previous w values*/
		yn =  b[section][0]*wn + b[section][1] *w[section][0] + b[section][2]*w[section][1];


		/*shift the values of w for the iteration/section*/
		w[section][1] =  w[section][0];
		w[section][0] = wn;

		/*assign the output of the current section as the input to the next section*/
		input =  yn;
	}

	filtered_value   =  yn;

	/*Return filtered value*/
	return filtered_value;

}
