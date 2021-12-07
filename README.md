# CARBON programming language

> Status: just an idea

CARBON is an interface-centric programming language named after the concept of an allotropy.
It is an experiment in hyper-dynamic programming.
Programs do not define concrete representations. Instead, the runtime decides which representation to use. A type's representation can change during its lifetime.
Users can opt in to prioritizing certain runtime characteristics, such as `speed` or `memory`, per instance. The runtime will run experiments to determine the optimal representation that satisfies the priorities.
In some sense, profile-guided optimization at runtime.

## Langjam #0002

Langjam's theme is **patterns**. CARBON treats this relatively lightly:

- identifier rules, e.g. allowing them to be arbitrary Unicode characters, but all characters in each identifier must be in the same Unicode script


## Tour

CARBON is an unusual programming language, so it may as well use unusual syntax. Its identifiers look a lot like lisp. Words are conventionally delimited by hyphens, e.g. `a-variable`. This style, known informally as kebab case, is also seen in [Dylan](https://opendylan.org/).  

Scalar types:

- Atoms: `whitespace-delimited`, `façade`
- Numbers: `1`, `2.0`, `-9`
- Text: `" some words "`
- Regular expressions: `/ [abc]+ /`

- Lists: `[ 1 2 3 ]`
- Sets: `{ 1 2 3 }`
- Dictionaries: `{ "a" : 1  "b" : 2  "c" : 3 }`

All types accept a hyper-parameter, which is what should be maximized by the representation optimizer.

- ` [ 1 2 3 ] :: { speed }`

Identifiers can be Unicode, with some restrictions. Characters within the identifier must belong to the same [Unicode script], although it's legal to include characters from the Common . That is, it is impossible to mix Cyrillic and Tamil in the same identifier.


[Unicode group]: https://en.wikipedia.org/wiki/Script_(Unicode)