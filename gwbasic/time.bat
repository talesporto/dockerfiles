@ECHO OFF
set GWBASIC=GWBASIC.EXE
echo %time%
FOR /L %%n IN (1,1,100) DO CALL ruby run-dos-box.rb HELLO.BAS
echo %time%
