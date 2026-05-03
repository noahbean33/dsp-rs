% Specifications for the Elliptic IIR Filter
rp = 1;     % Passband ripple in dB
rs = 40;    % Stopband attenuation in dB
fs = 100e3; % Sampling frequency
fpass = 5e3; % Passband frequency
fstop = 10e3; % Stopband frequency

% Design the Elliptic IIR filter
[N, Wn] = ellipord(fpass/(fs/2), fstop/(fs/2), rp, rs);
[b, a] = ellip(N, rp, rs, Wn);

% Transform to Second-Order Sections (SOS)
[sos, g] = tf2sos(b, a);
[numSections, ~] = size(sos);

% Open the header file in write mode
fname = 'elliptic.h';
fid = fopen(fname, 'wt');

% Write definitions to the header file
fprintf(fid, '#ifndef ELLIPTIC_H\n#define ELLIPTIC_H\n\n');
fprintf(fid, '#define NUM_SECTIONS %d\n\n', numSections);
fprintf(fid, '/*Generated with MATLAB \n');
fprintf(fid, ' * Order:  %d\n', N);
fprintf(fid, ' * IIR: Elliptic\n');
fprintf(fid, ' * Fs: %dHz\n', fs);
fprintf(fid, ' * Fpass: %d\n', fpass);
fprintf(fid, ' * Apass : %ddB\n', rp);
fprintf(fid, ' * Astop: %ddB*/\n', rs);


% Write coefficients to the header file
writeCoeff(fid, 'b', sos(:, 1:3), g);
writeCoeff(fid, 'a', sos(:, 4:6), 1);

% Close the definitions
fprintf(fid, '\n#endif');

% Close the header file
fclose(fid);


function writeCoeff(fid,coeffName, coeffs,g)
     [numSections, ~] = size(coeffs);
     fprintf(fid, 'float %s[NUM_SECTIONS][3] = {\n', coeffName);
     for i = 1:numSections
        % Scale the coefficients by the gain if writing b coefficients
        coeff = coeffs(i, :) * (i == 1) * g + coeffs(i, :) * (i ~= 1);
        if i == numSections
            fprintf(fid, '{%.8E, %.8E, %.8E} };\n', coeff);
        else
            fprintf(fid, '{%.8E, %.8E, %.8E},\n', coeff);
        end
     end
end



