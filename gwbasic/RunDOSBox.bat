@ECHO OFF
set SDL_VIDEODRIVER=dummy
"C:\Program Files (x86)\DOSBox-0.74\DOSBox.exe" C:\Users\ngeor\DOSBOX\PROGS\GWBASIC\RUNGW.BAT -exit -noautoexec
type C:\Users\ngeor\DOSBOX\PROGS\GWBASIC\STDOUT.TXT
