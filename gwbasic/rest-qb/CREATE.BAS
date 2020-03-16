DEFINT A-Z

DECLARE SUB PrintHeaders()
DECLARE SUB ExitWithStatus(stat$)
DECLARE SUB ValidateRequestMethod(method$)
DECLARE SUB ValidateContentType()

ON ERROR GOTO ErrorTrap

ValidateRequestMethod("POST")
ValidateContentType

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

OPEN "TODO.DAT" FOR APPEND AS #1
PRINT #1, T$
CLOSE #1

PrintHeaders
PRINT "Processed " + T$
SYSTEM

ErrorTrap:
    PRINT "Status: 500 Internal Server Error"
    PRINT ""
    PRINT ERR
    SYSTEM

SUB PrintHeaders()
    PRINT "Status: 201 Created"
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
