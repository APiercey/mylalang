# Mylalang

[Mylalang](https://github.com/APiercey/mylalang) is a LISP implemented in Rust.

## Content
- [Mylalang](#mylalang)
  * [Usage](#usage)
    + [Interactive REPL](#interactive-repl)
    + [Interpreter](#interpreter)
  * [Types](#types)
  * [Arithmetic Operators](#arithmetic-operators)
    + [List Processing](#list-processing)
  * [Relational Operators](#relational-operators)
  * [Comments](#comments)
  * [Binding values](#binding-values)
  * [Functions](#functions)
    + [Named Functions](#named-functions)
    + [Anonmymous Functions](#anonmymous-functions)
    + [Local Binding](#local-binding)
    + [Aliasing](#aliasing)
  * [Inspecting](#inspecting)
  * [Do function](#do-function)
  * [Conditionals](#conditionals)
  * [Importing](#importing)
  * [Recursion and Loops](#recursion-and-loops)
  * [Native functions](#native-functions)

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

## Types
Only primitive types exist in Mylalang. Constructing types does not exist.

- `Integer`s written like `1` or `1042`.
- `Float`s written like `89.353`.
- `String`s written like "This is a string!" or "Hello, world."
- `Null` written as `nil`.
- `List`s written as `[]` contain other types, written like `[2 23 88]` or `["mixed list" 823.45 "containing different" 90 "types" nil]`
- `Function`s wrtten as `(fn [arg1 arg2 ...argN])`. More about this under the functions section.

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
Comments start with `;`
```clojure
; This is a comment!

; This code does not execute
; (def a "123")

a

thread 'main' panicked at '"a" does not exist within this scope

; oops :)
```

## Binding values
The `def` keyword allows bind a value (and functions but more on that later) to a name:

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

### Local Binding
It is possible to bind local scoped variables to a function using the `let` keyword. This keyword functions the same as `def` but can only be used within a function.

`let` is a function which takes multiple arguments. The first is `list` of name and value pairs and binds the name in the first position to a value in the second position of each pair in the list. 

The remaining arguments can be function calls with the last function to return a value. This is used when you want to execute side effects or inspect a result.

```clojure
# This example needs to be compressed into a single line to execute correctly in the REPL.

(def shifter (fn [i]
  (let [x (* i i)
        y (+ i i)
        z (if (> x 50) (- x y) (+ x y))]
    (+ "Final value is " z))))
    
=> <#def "shifter">
```

In the example above, `x` equals the value if `i` multiplied by `i`.

```clojure
(shifter 23)
=> Final value is 483
```

## Aliasing
Because all types - including functions - are just values, it is possible to bind a name to _another_ name using `def`. 

```clojure
(def hey (fn [] (inspect "Hey")))
=> <#def "hey">

(def sayhey hey)
=> <#def "sayhey">

(sayhey)
Hey
=> Hey

(def no_i_made_this "A very fancy string")
=> <#def "no_i_made_this">

(inspect no_i_made_this)
=> "A very fancy string"
```

## Inspecting
Mylalang lets you inspect a value and pass it through to calling functions.

Inspecting simple results
```clojure
(inspect "This is an inspection!")

This is an inspection!
=> This is an inspection!
```

The output in a REPL shows a printed result and then shows the value of the executed statement.

Using inspect inside a function
```clojure
(def suspiciousfunction (fn [a]
  (let [r (* a a)]
    (inspect (+ "Result is strange... " r))
    r)))
```

## Do function
The `do` function lets you execute multiple functions without the need of a `let` function. It's used when you want to execute side effects but have no need for `let` local bindings.

```clojure
(def saygarbage (fn [a]
  (do (inspect "Hello world")
      (inspect "foo bar")
      a)))

=> <#def "saygarbage">
(saygarbage "123")
Hello world
foo bar
=> 123
```

## Conditionals
There are two conditional statements:
- `if`, which when given a true statement returns (or executes) the left value. When false, the right value.
- `unless`, which when given a false statement returns (or executes) the left value. When true, the right value.

```clojure
(if (< 1 10) "One is lower than ten" "Something is fishy..")

=> One is lower than ten
```

```clojure
(unless (= 8 8) "We've broken math" "8 always equals 8")

=> 8 always equals 8
```

## Importing
Importing named valued (bound through `def`) in another file is possible using the `import` function, which binds the imported functions to named in the local scope.

```clojure
# math.my
(def double (fn [a] (* a 2)))

# main.my
(import "math.my")

=> <#def "double"> 

(double 99)

=> 198
```

## Recursion and Loops
Recusion is used for looping in Mylang.

Example of a function using recursion create a loop with an exit condition
```clojure
(def start 0)

(def inc (fn [i] 
  (+ i 1)))

(def loop (fn [i] 
  (if (<= i 10) 
    (loop (inc (inspect i)))
    (inspect "finished"))))

(loop start)
```

## Native functions
Most functions covered are implemented or aliased using Mylalang itself. There are additional examples such as:
- `apply`
- `foldl`
- `foldr`
- `min`
- `max`

and more, which exist in the language. They can be found by checking out the native [implementation files](https://github.com/APiercey/mylalang/tree/master/src/native)
