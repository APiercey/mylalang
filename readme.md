# Mylalang

[Mylalang](https://github.com/APiercey/mylalang) is a LISP implemented in Rust.

# TODO: Document head and tail
## Content
- [Content](#content)
- [Usage](#usage)
  * [Interactive REPL](#interactive-repl)
  * [Interpreter](#interpreter)
- [Types](#types)
- [Arithmetic Operators](#arithmetic-operators)
  * [List Processing](#list-processing)
- [Relational Operators](#relational-operators)
- [Comments](#comments)
- [Binding values](#binding-values)
- [Functions](#functions)
  * [Named Functions](#named-functions)
  * [Anonymous Functions](#anonymous-functions)
  * [Local Binding](#local-binding)
  * [Function Overloading](#function-overloading)
- [Aliasing](#aliasing)
- [Inspecting](#inspecting)
- [The `do` Function](#the--do--function)
- [Conditionals](#conditionals)
- [Files](#files)
  * [Reading Files](#reading-files)
  * [Importing](#importing)
- [Recursion and Loops](#recursion-and-loops)
- [The `cons` Function](#the--cons--function)
- [The `list` Function](#the--list--function)
  * [`:` Operator](#----operator)
- [The `&` Operator and the `apply` Function](#the-----operator-and-the--apply--function)
- [Evaluating Code](#evaluating-code)
  * [Complex Example](#complex-example)
- [Native functions](#native-functions)
  * [`min`](#-min-)
  * [`max`](#-max-)
  * [`double`](#-double-)
  * [`reduce`](#-reduce-)
  * [`map`](#-map-)

<small><i><a href='http://ecotrust-canada.github.io/markdown-toc/'>Table of contents generated with markdown-toc</a></i></small>

## Usage
Firstly, clone the repository to a local directory
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
- `Array`s written as `[]` contain other types, written like `[2 23 88]` or `["mixed list" 823.45 "containing different" 90 "types" nil]`
- `List`s written as `()` contain other types and form the syntax of the language. Can be constructed using the `list` function.
- `Function`s written as `(fn [arg1 arg2 ...argN])`. More about this under the functions section.
- `Boolean` values are natural booleans, written as `true` or `false`

## Arithmetic Operators
Addition
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

Functions can take multiple parameters
```clojure
(def greet (fn [firstname lastname] (+ "Hello " firstname " " lastname)))
=> <#def "greet">

(greet "John" "Travolta")
=> Hello John Travolta
```

### Anonymous Functions
Anonymous functions are called without binding it to a name. An example makes this easier to explain

The function below doubles a given number:
```clojure
(fn [a] (* 2))
=> <#anonfunc [[<#def "a">]]>
```

The return value is a function. Like this, it's a bit useless but it provides the foundation of using functions as values. They can be called immediately or passed as a function:

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

### Function Overloading
It's possible to bind multiple functions to the same name if they accept different parameters.

```clojure
(def greeter (fn [name] 
  (let [greeting (+ "Hello " name)]
    (inspect greeting))))
=> <#def "greeter">
  
(def greeter (fn [first_name last_name]
  (let [greeting (+ "Hello " first_name " " last_name)]
    (inspect greeting))))
=> <#def "greeter">
  
(def greeter (fn [salutation first_name last_name]
  (let [greeting (+ "Hello " salutation ". " first_name " " last_name)]
    (inspect greeting))))
=> <#def "greeter">

(greeter "Alan")
=> Hello Alan

(greeter "Alan" "Turing")
=> Hello Alan Turing

(greeter "Dr" "Alan" "Turing")
=> Hello Dr. Alan Turing
```
This allows to build boundaries in a very convenient way, particularly with recursion.

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

## The `do` Function
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

## Files
### Reading Files
The `readfile` function will read the contents of a file. The value returned is a string.

```clojure
; echo "Hello world" > file.txt

(readfile "file.txt")

=> Hello world
```

### Importing
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
Recursion is used for looping in Mylang.

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

## The `cons` Function
The `cons` operator is function which constructs a list from another value. An example of its usage is to append a value to a list.
```clojure
(cons "This" ["is" "my" "list"])
=> [[This, is, my, list]]
```

## The `list` Function
The `()` syntax is literally a list which executes a context. The context being the first item in the list.

For this reason, it is difficult for Mylalang to understand if you want to _construct_ a list or execute a some context, such as a bound function.
The `list` function solves this issue by creating a list for you.

```clojure
(list 1 2 3 5 7 11 13)
=> ([1, 2, 3, 5, 7, 11, 13])
```

Lists can be operated on like any other primitive type.

```clojure
(+
  (list 1 2 3)
  (list 4 5 6))
  
=> ([1, 2, 3, 4, 5, 6])
```

The real power of lists comes out when used with recursion where they are applied as a list of arguments. See the `apply` function.

### `:` Operator
The short form of cons is the `:` operator. It is used the same as above.

```clojure
(: 1 [2 3])
=> [[1, 2, 3]]
```

## The `&` Operator and the `apply` Function
The `apply` function is a powerful function that allows us to expand a list as arguments for a function. It accepts the function as the first argument and a list of values-as-parameters as the second argument. 

An example using the `+` operator.

```clojure
; The operation we want to perform
(+ 1 2 3)
=> 6

(apply + (list 1 2 3))
=> 6
```

In most cases, we want to operate on value individually. The `&` (capture) operator allows us to capture all remaining unnamed arguments as a list.


```clojure
(def cap_example (fn [first & rest]
  (do (inspect "First arg")
      (inspect first)
      (inspect "The rest of the args")
      (inspect rest))))

=> <#def "cap_example">

(cap_example 1 2 3 4 5) ; Calling the function
First arg               ; inspecting first argument
1
The rest of the args    ; inspecting the remaining arguments
([2, 3, 4, 5])          

=> ([2, 3, 4, 5])       ; Final return result of the function
```

With recursion, it becomes even more powerful. The implementation of `max` shows how recursion, cons, apply, and & can be used together.

```clojure
(def max (fn [a] a))
(def max (fn [a & rest]
  (let [b (apply max rest)] 
    (if (>= a b) a b))))
```

## Evaluating Code
The `eval` function accepts a string and executes it as code. The return result is always `nil` but any functions or values bound to a name will be bound to the local scope.

A simple example using `inspect`:

```clojure
(eval "(def value 1)")
=> <#def "value">

(inspect value)
1
=> 1
```

### Complex Example
A more complex example when you would want to eval code, is when the code is in another file or serialized into a string. The `import` function is implemented in Mylalang itself using only `eval` and `readfile`.
```clojure
(def import (fn [file_name] 
  (let [contents (readfile file_name)]
    (eval contents))))
```

## Native functions
Most functions covered are implemented Mylalang itself. While there is no standard library, there are additional functions which come with Mylalang:

### `min`
Returns the smallest value from a list.

```clojure
(min 23 77 99)

=> 23
```

### `max`
Returns the largest value from a list.

```clojure
(min 23 77 99)

=> 99
```

### `double`
Doubles a value! Originally implemented to provide a function to test with.

```clojure
(double 21)

=> 42
```

### `map`
The `map` function maps a function over a list. Accepts a function as its first argument and a list for the remaining arguments.

```clojure
(map double 1 2 3 4 5)

=> ([2, 4, 6, 8, 10])
```

### `reduce`
Reduces a list of values to the right. Accepts a function in it's first position, accumulator in it's second. The rest of the arguments are captured.
```clojure
(reduce + 0 1 2 3 4)

=> 10
```

More functions exist in the language which can be found by checking out the native [implementation files](https://github.com/APiercey/mylalang/tree/master/src/native).
