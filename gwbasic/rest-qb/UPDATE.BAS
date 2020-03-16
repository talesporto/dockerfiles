DEFINT A-Z

DECLARE SUB PrintHeaders()
DECLARE SUB ExitWithStatus(stat$)
DECLARE SUB ValidateRequestMethod(method$)
DECLARE SUB ValidateContentType()
DECLARE FUNCTION GetId%()

ON ERROR GOTO ErrorTrap

ValidateRequestMethod("POST")
ValidateContentType
ID = GetId

OPEN ENVIRON$("STDIN") FOR INPUT ACCESS READ AS #1
IF EOF(1) THEN
    T$ = ""
ELSE
    LINE INPUT #1, T$
END IF
CLOSE #1

IF LEN(T$) <= 0 THEN
    ExitWithStatus("400 Bad request, give me one todo item")
END IF

NAME "TODO.DAT" AS "TODO.OLD"
OPEN "TODO.OLD" FOR INPUT AS #1
OPEN "TODO.DAT" FOR OUTPUT AS #2
WHILE NOT EOF(1)
    LINE INPUT #1, A$
    ID = ID - 1
    IF ID = 0 THEN Z$ = T$ ELSE Z$ = A$
    PRINT #2, Z$
WEND
CLOSE #1
CLOSE #2
KILL "TODO.OLD"

IF ID > 0 THEN ExitWithStatus("404 Not Found")

PrintHeaders
SYSTEM

ErrorTrap:
    PRINT "Status: 500 Internal Server Error"
    PRINT ""
    PRINT ERR
    SYSTEM

SUB PrintHeaders()
    PRINT "Status: 200 Updated"
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

SUB ValidateContentType()
    CT$ = ENVIRON$("CONTENT_TYPE")
    IF CT$ <> "text/plain" THEN
        ExitWithStatus("415 Unsupported media type, I only speak text/plain")
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
