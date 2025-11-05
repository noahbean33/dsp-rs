#include <stdio.h>
#include <stdlib.h>
#include <math.h>


#define SIG_LENGTH 320

extern double _320_pts_ecg_IMX[SIG_LENGTH];
extern double _320_pts_ecg_REX[SIG_LENGTH];

double Output_magnitude[SIG_LENGTH];
double Output_phase[SIG_LENGTH];


void rect_to_polar_conversion(double *sig_src_rex_arr,
                              double *sig_src_imx_arr,
                              double *sig_out_mag_arr,
                              double *sig_out_phase_arr,
                              int sig_length
                              );

int main()
{
   FILE *input_rex_fptr,*input_imx_fptr,*output_mag_fptr,*output_ph_fptr;

     rect_to_polar_conversion((double *)&_320_pts_ecg_REX[0],
                              (double *)&_320_pts_ecg_IMX[0],
                              (double *)&Output_magnitude[0],
                              (double *)&Output_phase[0],
                              (int) SIG_LENGTH
                              );

    input_rex_fptr = fopen("input_rex.dat","w");
    input_imx_fptr = fopen("input_imx.dat","w");
    output_mag_fptr = fopen("output_magnitude.dat","w");
    output_ph_fptr  = fopen("output_phase.dat","w");

    for(int i =0;i<SIG_LENGTH;i++)
    {
        fprintf(input_rex_fptr,"\n%f",_320_pts_ecg_REX[i]);
        fprintf(input_imx_fptr,"\n%f",_320_pts_ecg_IMX[i]);
        fprintf(output_mag_fptr,"\n%f",Output_magnitude[i]);
        fprintf(output_ph_fptr,"\n%f",Output_phase[i]);
    }

    fclose(input_rex_fptr);
    fclose(input_imx_fptr);
    fclose(output_mag_fptr);
    fclose(output_ph_fptr);

    return 0;
}



void rect_to_polar_conversion(double *sig_src_rex_arr,
                              double *sig_src_imx_arr,
                              double *sig_out_mag_arr,
                              double *sig_out_phase_arr,
                              int sig_length
                              )
    {
     double PI = 3.14159265358979f;
     int k;
     for(k=0;k<sig_length;k++)
     {
         sig_out_mag_arr[k] = sqrt(powf(sig_src_rex_arr[k],2)+ powf(sig_src_imx_arr[k],2));
          if(sig_src_rex_arr[k]==0)
          {
            sig_src_rex_arr[k]= pow(10,-20);
            sig_out_phase_arr[k] = atan(sig_src_imx_arr[k]/sig_src_rex_arr[k]);

            }

            if((sig_src_rex_arr[k]<0)&&(sig_src_imx_arr[k]<0))
            {
                sig_out_phase_arr[k] = sig_out_phase_arr[k]-PI;
            }
           if((sig_src_rex_arr[k]<0)&&(sig_src_imx_arr[k]>=0))
           {
               sig_out_phase_arr[k] = sig_out_phase_arr[k]+PI;
           }
     }

    }
