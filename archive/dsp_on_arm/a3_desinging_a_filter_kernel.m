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


%Implement Low-pass filter of cutoff frequency 70Hz, 
%sampling at 2000Hz (2KHz)
%fir1(kernel_length - 1, cutoff/sampling_rate)

fc =  70;
fs =  2000;
h  = fir1(100,fc/fs);
 
%plot(h);
%freqz(h);

filtered_sig =  conv(h,signal);

%plot input signal vs. filtered signal
subplot(2,1,1);
plot(signal,'r.-','LineWidth',1,'MarkerSize',1);
title('Input Signal');

subplot(2,1,2);
plot(filtered_sig,'b.-','LineWidth',1,'MarkerSize',1);
title('Filtered Signal');

%Save filter kernel to csv
csvwrite('lpf_fc_70hz.txt',h);

%Save input signal to csv
csvwrite('_10hz_100hz_500hz_sig.txt',signal);