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
f1 =  10; %10hz
T1  = 1/f1;
amp1 = 1;


%Make signal
signal1 =  amp1 * sin(2*pi*t/T1);

plot(t,signal1);