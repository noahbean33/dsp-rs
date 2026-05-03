clc; %Clear command window
close all;  %close all figures
clear;% erase all existing variables
workspace; % make sure workspace is showing

%show numbers in a compact form with 5 digits
format long g
format compact

%Set font size
fontSize = 2;

%Make  0.5 seconds sampled every 1/2000 of a second (2KHz sampling rate)
t = 0 : 1/2000 : 0.5;

%Define wave parameters

f1 =  10; %10Hz
T1  = 1/f1;
amp1 = 1;


f2 =  100; %100Hz
T2  = 1/f2;
amp2 = 1;

f3 =  500; %500Hz
T3  = 1/f3;
amp3 = 1;

%Make signal
signal1 =  amp1 * sin(2*pi*t/T1);
signal2 =  amp2 * sin(2*pi*t/T2);
signal3 =  amp3 * sin(2*pi*t/T3);

%Combine signals
signal =  signal1 + signal2 + signal3;

%Compute dft of the original signal
signal_fft =  fft(signal);

%Get seignal length
L = length(signal);

%Compute signal magnitude
sig_mag =  abs(signal_fft);

Fs =  2000;
f  = Fs *(0: (L/2))/L;

P2 =  abs(signal_fft/L);
P1 = P2(1: L/2+1);
P1(2:end -1) = 2*P1(2:end-1); 
plot(f,P1);
title('Single-Sided Amplitude Spectrum of x(t)');
xlabel('f(Hz)')




