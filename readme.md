# Mylalang

Mylalang is a LISP implemented in Rust.

## Usage
Firstly, clone the repo to a local directory
```bash
git clone git@github.com:APiercey/mylalang.git
cd mylalang
```

Mylalang comes with two binaries: an interactive REPL and a interpreter.

### Interactive REPL
The REPL can be started by running:
```bash
cargo run --bin imy
```

### Interpreter
The REPL can be started by running:
```bash
cargo run --bin imy
```

## Arithmetic Operators
Addiiton
```clojure
(+ 1 2)
=> 3
```

Subtraction
```clojure
(- 7 2)
=> 5
```

Multiplication
```clojure
(* 10 2)
=> 20
```

Division
```clojure
(/ 2 10)
=> 20
```

### List Processing
All operators work against their arguments as lists, reducing to a result:

```clojure
(+ 1 2 3 4 5)
=> 15

(- 1 2 3 4 5)
=> -13
```

## Relational Operators

Equality
```clojure
(= 1 1)
=> true

(= 2 1)
=> false
```

Greater than
```clojure
(> nil 1)
=> false

(> 1 nil)
=> true
```

Less than
```clojure
(< 1 nil)
=> false

(< nil 1)
=> true
```

Greater than or equal to
```clojure
(>= 1 1)
=> true

(>= 2 1)
=> true

(>= 1 2)
=> false
```

Less then or equal to
```clojure
(<= 1 1)
=> true

(<= 1 2)
=> true

(<= 2 1)
=> false
```

## Comments
Comments start with `#`
```clojure
# This is a comment!

# This code does not execute
# (def a "123")

a

thread 'main' panicked at '"a" does not exist within this scope

# oops :)
```

## Defining variables
The `def` keyword allows bind a value (and functions but more on that later) to a name:

Variables 
```clojure
(def a 23)
=> <#def "a">

(def b 2)

(+ a b)
=> 23
```

## Functions
In Mylalang, functions are a primitive type. They can be bound to a name and used as a value. Through this, the language supports both anonymous and named functions.

### Named Functions
The `def` keyword can be used to bind a function to a name
```clojure
(def double (fn [a] a * 2))
=> <#def "double">

(double 2)
=> 4
```

Functions can take multiple paramaters
```clojure
(def greet (fn [firstname lastname] (+ "Hello " firstname " " lastname)))
=> <#def "greet">

(greet "John" "Travolta")
=> Hello John Travolta
```

### Anonmymous Functions
Anonymous functions are called without binding it to a name. An example makes this easier to explain

The function below doubles a given number:
```clojure
(fn [a] (* 2))
=> <#anonfunc [[<#def "a">]]>
```

The return value is a function. Like this, it's a bit useless but it provides the foundation of using functions as values. They can be called immediatly or passed as a function:

Executing an anonymous function
```clojure
((fn [fruit] (+ "My favorite fruit is " fruit)) "Banana")
=> My favorite fruit is Banana
```

Comparing to its named counterpart
```clojure
(def favfruit (fn [fruit] (+ "My favorite fruit is " fruit)))
=> <#def "favfruit">

(favfruit "Banana")
=> My favorite fruit is Banana
```

Passing functions as values
```clojure
(def double (fn [a] (* a 2)))
=> <#def "double">

(def applyfunc (fn [f a] (+ "The value of the applied function is: " (f a)))
=> <#def "applyfunc">

(applyfunc double 21)
=> The value of the applied function is: 42
```

