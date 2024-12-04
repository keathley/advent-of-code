Definitions.

INT = [0-9]+
IDENT = [a-z_]+
WHITESPACE = [\s\t\n\r]

Rules.

{INT}        : {token, {int, TokenLine, list_to_integer(TokenChars)}}.
{IDENT}      : {token, {ident, TokenLine, TokenChars}}.
\(           : {token, {'(', TokenLine}}.
\)           : {token, {')', TokenLine}}.
,            : {token, {',', TokenLine}}.
->           : {token, {'->', TokenLine}}.
{WHITESPACE} : skip_token.

Erlang code.
