DEFINT A-Z

DECLARE SUB PrintHeaders()
DECLARE SUB ExitWithStatus(stat$)
DECLARE SUB ValidateRequestMethod(method$)
DECLARE FUNCTION GetId%()

ON ERROR GOTO ErrorTrap

ValidateRequestMethod("GET")
ID = GetId

OPEN "TODO.DAT" FOR INPUT AS #1
WHILE NOT EOF(1) AND ID > 0
    LINE INPUT #1, A$
    ID = ID - 1
WEND
CLOSE #1

IF ID <> 0 THEN ExitWithStatus("404 Not Found")
PrintHeaders
PRINT "{" + CHR$(34) + "item" + CHR$(34) + ": " + CHR$(34) + A$ + CHR$(34) + "}"
SYSTEM

ErrorTrap:
    IF ERR=53 THEN ExitWithStatus("404 Not Found") ' file not found
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

FUNCTION GetId()
    QS$ = ENVIRON$("QUERY_STRING")
    IDX = INSTR(QS$, "id=")
    IF IDX <= 0 THEN
        ExitWithStatus("400 Bad request, missing id parameter")
    END IF
    IDX = IDX + 3
    ID = VAL(MID$(QS$, IDX, LEN(QS$) - IDX + 1))
    IF ID <= 0 THEN
        ExitWithStatus("400 Bad request, wrong id parameter")
    END IF
    GetId = ID
END FUNCTION
