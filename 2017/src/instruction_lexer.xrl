Definitions.

INT        = -?[0-9]+
IDENT      = [a-z_]+
WHITESPACE = [\s\t\n\r]
OPERATOR   = [><=!]+
IF         = if
INC        = inc
DEC        = dec

Rules.

{INT}        : {token, {int, TokenLine, list_to_integer(TokenChars)}}.
{INC}        : {token, {operation, TokenLine, TokenChars}}.
{DEC}        : {token, {operation, TokenLine, TokenChars}}.
{OPERATOR}   : {token, {operator, TokenLine, TokenChars}}.
{IF}         : {token, {check, TokenLine, TokenChars}}.
{IDENT}      : {token, {ident, TokenLine, TokenChars}}.
{WHITESPACE} : skip_token.

Erlang code.
