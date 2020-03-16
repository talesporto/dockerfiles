DEFINT A-Z

DECLARE SUB PrintHeaders()
DECLARE SUB ExitWithStatus(stat$)
DECLARE SUB ValidateRequestMethod(method$)

ON ERROR GOTO ErrorTrap

ValidateRequestMethod("GET")

PrintHeaders

PRINT "["
OPEN "TODO.DAT" FOR INPUT AS #1
WHILE NOT EOF(1)
    LINE INPUT #1, A$
    PRINT "    " + CHR$(34) + A$ + CHR$(34)
    IF NOT EOF(1) THEN PRINT(",")
WEND
CLOSE #1

CloseArray:
    PRINT "]"

SYSTEM

ErrorTrap:
    IF ERR=53 THEN RESUME CloseArray ' file not found
    PRINT "Status: 500 Internal Server Error"
    PRINT ""
    PRINT ERR
    SYSTEM

SUB PrintHeaders()
    PRINT "Status: 200 OK"
    PRINT "Content-Type: text/plain"
    PRINT "X-Powered-By: QBASIC"
    PRINT ""
END SUB

SUB ExitWithStatus(s$)
    PRINT "Status: " + s$
    PRINT ""
    SYSTEM
END SUB

SUB ValidateRequestMethod(method$)
    M$ = ENVIRON$("REQUEST_METHOD")
    IF M$ <> method$ THEN
        ExitWithStatus("405 Method not allowed, send " + method$)
    END IF
END SUB
