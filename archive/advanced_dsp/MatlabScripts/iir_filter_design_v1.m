% Specifications for the Elliptic IIR Filter
rp = 1;     % Passband ripple in dB
rs = 40;    % Stopband attenuation in dB
fs = 100e3; % Sampling frequency
fpass = 5e3; % Passband frequency
fstop = 10e3; % Stopband frequency

% Design the Elliptic IIR filter
[N, Wn] = ellipord(fpass/(fs/2), fstop/(fs/2), rp, rs);
[b, a] = ellip(N, rp, rs, Wn);

% Load the mixed signal from the generated header file
mixedSignal = loadSignalFromHeader('_1kHz_20kHz_sine.h');

% Filter the mixed signal to remove the 20kHz component
filteredSignal = filter(b, a, mixedSignal);


% Plot the original mixed signal
figure;
subplot(2,1,1);
plot(t, mixedSignal);
title('Original Mixed Signal (1kHz + 20kHz)');
xlabel('Time (s)');
ylabel('Amplitude');


% Plot the filtered signal
subplot(2,1,2);
plot(t, filteredSignal);
title('Filtered Signal (20kHz component removed)');
xlabel('Time (s)');
ylabel('Amplitude');


function signal =  loadSignalFromHeader(filename)
    % Function to load signal from a given C header file
    % Function reads the file line by line and extracts signal data
    
    % Open the file in read mode
    fid = fopen(filename, 'r');
    
    if fid == -1
        error('Cannot open file for reading');
    end 
    signal = [];


    % Read the file line by line
    tline = fgets(fid);
    while ischar(tline)
        % Search for the line with signal data
        if contains(tline, '{')
            % Extract the signal data
            signalStr = extractBetween(tline, '{', '}');
            signal = str2double(strsplit(signalStr{1}, ','));
        end
        tline = fgets(fid);
    end

    %close file
    fclose(fid);
end