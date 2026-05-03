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

%plot(t,signal);

subplot(4,1,1);
plot(t,signal1,'b.-','LineWidth',1,'MarkerSize',1);
title('10Hz Signal');

subplot(4,1,2);
plot(t,signal2,'b.-','LineWidth',1,'MarkerSize',1);
title('100Hz Signal');

subplot(4,1,3);
plot(t,signal3,'b.-','LineWidth',1,'MarkerSize',1);
title('500Hz Signal');

subplot(4,1,4);
plot(t,signal,'g.-','LineWidth',1,'MarkerSize',1);
title('Combined Signal');
