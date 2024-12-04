Definitions.

CHARS      = [a-z0-9"]
WHITESPACE = [\s\t\n\r]

Rules.

\{           : {token, {'{', TokenLine}}.
\}           : {token, {'}', TokenLine}}.
,            : {token, {',', TokenLine}}.
<            : {token, {'<', TokenLine}}.
>            : {token, {'>', TokenLine}}.
!            : {token, {'!', TokenLine}}.
{CHARS}      : {token, {ident, TokenLine, TokenChars}}.
{WHITESPACE} : skip_token.

Erlang code.
