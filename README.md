# CARBON programming language

> Status: just an idea

CARBON is an interface-centric programming language. It is an experiment in hyper-dynamic programming. Profile-guided optimization to work runtime.

## Big idea

Programs do not define concrete representations. Instead, the runtime decides which representation to use. A type's representation can change during its lifetime.

Users can prefer to prioritize `time` or `memory` at an instance-specific.

## Tour

Scalar types:

- Atoms: whitespace-delimited
- Numbers: `1`, `2.0`, `-9`,
- Text: `"some words"`

CARBON is an unusual programming language, so it may as well use unusual syntax.

- Lists: `[ 1 2 3 ]`
- Sets: `{ 1 2 3 }`
- Dictionaries: `{ "a" . 1  "b" . 2  "c" . 3 }`

Identifiers can be Unicode, with some restrictions. Characters within the identifier must belong to the same [Unicode script], although it's legal to include characters from the Common . That is, it is impossible to mix Cyrillic and Tamil in the same identifier.


[Unicode group]: https://en.wikipedia.org/wiki/Script_(Unicode)