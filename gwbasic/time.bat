@ECHO OFF
echo %time%
FOR /L %%n IN (1,1,100) DO CALL RunDOSBox.bat
echo %time%
