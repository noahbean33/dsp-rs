% Specifications
frequency1 = 1e3;  % Frequency of the first sine wave: 1 kHz
frequency2 = 20e3; % Frequency of the second sine wave: 20 kHz
amplitude = 750;   % Amplitude of each sine wave: 1500/2 = 750
fs = 100e3;        % Sampling frequency: 100 kHz
duration = 0.01;   % Duration of the sine wave: 0.01 seconds (10 ms)
% Generate time vector
t = 0:1/fs:duration-1/fs; % Time vector from 0 to duration with 1/fs increments

% Generate the 1kHz sine wave
sinewave1 = amplitude * sin(2 * pi * frequency1 * t);

% Generate the 20kHz sine wave
sinewave2 = amplitude * sin(2 * pi * frequency2 * t);

% Mix the two sine waves
mixed_sinewave = sinewave1 + sinewave2;

% Open the file with write permission
fid = fopen('_1kHz_20kHz_sine.h', 'w');

% Check if file is opened successfully

if fid == -1
    error('Cannot open file for writing');
end

% Write header information to the file
fprintf(fid, '#ifndef _1KHZ_20KHZ_SINE_H\n');
fprintf(fid, '#define _1KHZ_20KHZ_SINE_H\n\n');
fprintf(fid, '#define NUM_SAMPLES %d\n', length(mixed_sinewave));
fprintf(fid, 'float _1kHz_20kHz_sine[NUM_SAMPLES] = {');

% Write the mixed sine wave data to the file horizontally
for i = 1:length(mixed_sinewave)
    if i == length(mixed_sinewave)
        fprintf(fid, ' %f ', mixed_sinewave(i)); % Last element, so don't print a comma
    else
        fprintf(fid, ' %f, ', mixed_sinewave(i)); % Print a comma after each element (except the last one)
    end
end

% Write the closing brace and endif to the file
fprintf(fid, '};\n\n');
fprintf(fid, '#endif // _1KHZ_20KHZ_SINE_H\n');
% Close the file
fclose(fid);

% Plotting
figure;
subplot(3, 1, 1);
plot(t, sinewave1);
title('1kHz Sine Wave');
xlabel('Time (s)');
ylabel('Amplitude');


subplot(3, 1, 2);
plot(t, sinewave2);
title('20kHz Sine Wave');
xlabel('Time (s)');
ylabel('Amplitude');

subplot(3, 1, 3);
plot(t, mixed_sinewave);
title('Mixed 1kHz & 20kHz Sine Wave');
xlabel('Time (s)');
ylabel('Amplitude');
