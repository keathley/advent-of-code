Terminals '{' '}' '<' '>' '!' char.
Nonterminals group garbage skip chars.
Rootsymbol groups.

groups -> group            : ['$1']
groups -> group ',' groups : ['$1' | '$3']

group -> '{' groups '}' : {'$1'}
group -> garbage        : nil
group -> chars          : 

garbage -> '<' garbage '>'
garbage -> skip chars

garbage_chars -> skip chars :
garbage_chars -> '<' garbage_chars : '$2'
garbage_chars -> '<' : nil
garbage_chars -> chars : '$2'

skip -> '!' char : nil

chars -> char : nil

Erlang code.

extract_token({_Token, _Line, Value}) -> Value.
