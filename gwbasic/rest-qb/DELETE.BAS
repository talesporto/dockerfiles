DEFINT A-Z

DECLARE SUB PrintHeaders()
DECLARE SUB ExitWithStatus(stat$)
DECLARE SUB ValidateRequestMethod(method$)
DECLARE FUNCTION GetId%()

ON ERROR GOTO ErrorTrap

ValidateRequestMethod("DELETE")
ID = GetId

NAME "TODO.DAT" AS "TODO.OLD"
OPEN "TODO.OLD" FOR INPUT AS #1
OPEN "TODO.DAT" FOR OUTPUT AS #2
WHILE NOT EOF(1)
    LINE INPUT #1, A$
    ID = ID - 1
    IF ID <> 0 THEN PRINT #2, A$
WEND
CLOSE #1
CLOSE #2
KILL "TODO.OLD"

IF ID > 0 THEN
    ExitWithStatus("404 Not Found")
END IF

PrintHeaders
SYSTEM

ErrorTrap:
    PRINT "Status: 500 Internal Server Error"
    PRINT ""
    PRINT ERR
    SYSTEM

SUB PrintHeaders()
    PRINT "Status: 200 Deleted"
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
