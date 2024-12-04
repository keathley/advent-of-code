Terminals '(' ')' ',' '->' ident int.
Nonterminals program weight names name.
Rootsymbol program.

program -> name weight            : {'$1', '$2', []}.
program -> name weight '->' names : {'$1', '$2', '$4'}.

names -> name           : ['$1'].
names -> name ',' names : ['$1' | '$3'].

name -> ident : extract_token('$1').

weight -> '(' int ')' : extract_token('$2').

Erlang code.

extract_token({_Token, _Line, Value}) -> Value.
