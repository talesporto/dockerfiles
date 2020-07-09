DEFINT A-Z

DECLARE SUB ExitWithStatus(stat$)
DECLARE SUB ValidateRequestMethod(method$)
DECLARE SUB ValidateContentType()
DECLARE SUB ListTodoItems()
DECLARE SUB CreateTodoItem()
DECLARE SUB ReadTodoItem()
DECLARE SUB UpdateTodoItem()
DECLARE SUB DeleteTodoItem()
DECLARE FUNCTION GetId%()
DECLARE FUNCTION GetAction$()
DECLARE FUNCTION GetQueryString$(key$)

CONST TRUE = -1
CONST FALSE = NOT TRUE

ACTION$ = GetAction$

ON ERROR GOTO ErrorTrap

SELECT CASE ACTION$
    CASE "list"
        ListTodoItems
    CASE "create"
        CreateTodoItem
    CASE "read"
        ReadTodoItem
    CASE "update"
        UpdateTodoItem
    CASE "delete"
        DeleteTodoItem
    CASE ELSE
        ExitWithStatus("400 Bad Request unknown action " + ACTION$)
END SELECT
SYSTEM

ErrorTrap:
    PRINT "Status: 500 Internal Server Error"
    PRINT ""
    PRINT ERR
    SYSTEM

SUB ListTodoItems()
    ValidateRequestMethod("GET")

    PRINT "Status: 200 OK"
    PRINT "Content-Type: text/plain"
    PRINT "X-Powered-By: QBASIC"
    PRINT ""

    PRINT "["
    OPEN "TODO.DAT" FOR INPUT AS #1
    WHILE NOT EOF(1)
        LINE INPUT #1, A$
        PRINT "    " + CHR$(34) + A$ + CHR$(34)
        IF NOT EOF(1) THEN PRINT(",")
    WEND
    CLOSE #1
    PRINT "]"
END SUB

SUB CreateTodoItem()
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

    PRINT "Status: 201 Created"
    PRINT "Content-Type: text/plain"
    PRINT "X-Powered-By: QBASIC"
    PRINT ""
    PRINT "Processed " + T$
END SUB

SUB ReadTodoItem()
    ValidateRequestMethod("GET")
    ID = GetId

    OPEN "TODO.DAT" FOR INPUT AS #1
    WHILE NOT EOF(1) AND ID > 0
        LINE INPUT #1, A$
        ID = ID - 1
    WEND
    CLOSE #1

    IF ID <> 0 THEN ExitWithStatus("404 Not Found")
    PRINT "Status: 200 OK"
    PRINT "Content-Type: text/plain"
    PRINT "X-Powered-By: QBASIC"
    PRINT ""
    PRINT "{" + CHR$(34) + "item" + CHR$(34) + ": " + CHR$(34) + A$ + CHR$(34) + "}"
END SUB

SUB UpdateTodoItem()
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

    PRINT "Status: 204 Updated"
    PRINT "Content-Type: text/plain"
    PRINT "X-Powered-By: QBASIC"
    PRINT ""
END SUB

SUB DeleteTodoItem()
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
    PRINT "Status: 204 Deleted"
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
    A$ = GetQueryString$("id")
    IF LEN(A$) > 0 THEN
        ID = VAL(A$)
    ELSE
        ID = 0
    END IF
    IF ID <= 0 THEN
        ExitWithStatus("400 Bad request, missing or wrong id parameter")
    ELSE
        GetId = ID
    END IF
END FUNCTION

FUNCTION GetAction$()
    A$ = GetQueryString$("action")
    IF LEN(A$) > 0 THEN
        GetAction$ = A$
    ELSE
        ExitWithStatus("400 Bad Request, missing action parameter")
    END IF
END FUNCTION

FUNCTION GetQueryString$(K$)
    Q$ = ENVIRON$("QUERY_STRING")
    N = 1
    NEEDLE$ = K$ + "="
    KeyIdx = 0
    FOUND = FALSE
    WHILE N < LEN(Q$) AND NOT FOUND
        KeyIdx = INSTR(N, Q$, NEEDLE$)
        IF KeyIdx <= 1 THEN
            ' If it is == 1 then it is found.
            ' If it is < 1 then it does not exist, mark it as found
            ' just to exit loop.
            FOUND = TRUE
        ELSE
            ' this is only good if the previous value is '&'
            IF MID$(Q$, KeyIdx - 1, 1) <> "&" THEN
                ' found it but it is a coincidence e.g. id= matched for pid=
                N = N + 1
            ELSE
                FOUND = TRUE
            END IF
        END IF
    WEND

    IF KeyIdx <= 0 THEN
        GetQueryString$ = ""
    ELSE
        ' find last ampersand after key index
        ValueIdx = KeyIdx + LEN(NEEDLE$)
        AmpIdx = INSTR(ValueIdx, Q$, "&")
        IF AmpIdx <= 0 THEN
            GetQueryString$ = MID$(Q$, ValueIdx)
        ELSE
            GetQueryString$ = MID$(Q$, ValueIdx, AmpIdx - ValueIdx)
        END IF
    END IF
END FUNCTION
