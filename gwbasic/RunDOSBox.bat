@ECHO OFF
REM ---
REM Runs DOSBox with the batch file RUNGW.BAT, which runs GWBasic PROGRAM.BAS,
REM redirecting stdin from STDIN.TXT and stdout to STDOUT.TXT.
REM
REM This is a very limited batch file, see run-dos-box.rb for a better solution,
REM which requires Ruby.
REM ---
REM Disable SDL graphics for DOSBox
set SDL_VIDEODRIVER=dummy
"C:\Program Files (x86)\DOSBox-0.74\DOSBox.exe" C:\Users\ngeor\DOSBOX\PROGS\GWBASIC\RUNGW.BAT -exit -noautoexec
type C:\Users\ngeor\DOSBOX\PROGS\GWBASIC\STDOUT.TXT
