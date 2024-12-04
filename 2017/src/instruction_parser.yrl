Terminals ident int check operator operation.
Nonterminals instruction condition register value.
Rootsymbol instruction.

instruction -> register operation value condition : {extract_token('$2'), '$1', '$3', '$4'}.

condition -> check register operator value : {extract_token('$3'), '$2', '$4'}.

register -> ident : extract_token('$1').

value -> int : extract_token('$1').

Erlang code.

extract_token({_Token, _Line, Value}) -> Value.
