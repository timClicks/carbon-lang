# CARBON programming language

> Status: just an idea

CARBON is an interface-centric programming language. It is an experiment in hyper-dynamic programming. Profile-guided optimization to work runtime.


## Langjam #0002

Langjam's theme is **patterns**. CARBON treats this relatively lightly:

- identifier rules, e.g. allowing them to be arbitrary Unicode characters, but all characters in each identifier must be in the same Unicode script


## Big idea

Programs do not define concrete representations. Instead, the runtime decides which representation to use. A type's representation can change during its lifetime.

Users can prefer to prioritize `time` or `memory` at an instance-specific.


## Tour

CARBON is an unusual programming language, so it may as well use unusual syntax.

Scalar types:

- Atoms: `whitespace-delimited` `façade`
- Numbers: `1`, `2.0`, `-9`
- Text: `" some words "`
- Regular expressions: `/ [abc]+ /`

- Lists: `[ 1 2 3 ]`
- Sets: `{ 1 2 3 }`
- Dictionaries: `{ "a" . 1  "b" . 2  "c" . 3 }`

Identifiers can be Unicode, with some restrictions. Characters within the identifier must belong to the same [Unicode script], although it's legal to include characters from the Common . That is, it is impossible to mix Cyrillic and Tamil in the same identifier.


[Unicode group]: https://en.wikipedia.org/wiki/Script_(Unicode)