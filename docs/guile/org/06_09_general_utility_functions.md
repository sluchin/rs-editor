### 6.9 General Utility Functions [¶](06_09_general_utility_functions.md#69-general-utility-functions)

This chapter contains information about procedures which are not cleanly tied to a specific data type. Because of their wide range of applications, they are collected in a _utility_ chapter.

*   [Equality](06_09_general_utility_functions.md#691-equality)
*   [Object Properties](06_09_general_utility_functions.md#692-object-properties)
*   [Sorting](06_09_general_utility_functions.md#693-sorting)
*   [Copying Deep Structures](06_09_general_utility_functions.md#694-copying-deep-structures)
*   [General String Conversion](06_09_general_utility_functions.md#695-general-string-conversion)
*   [Hooks](06_09_general_utility_functions.md#696-hooks)

* * *

Next: [Object Properties](06_09_general_utility_functions.md#692-object-properties), Up: [General Utility Functions](06_09_general_utility_functions.md#69-general-utility-functions)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.9.1 Equality [¶](06_09_general_utility_functions.md#691-equality)

There are three kinds of core equality predicates in Scheme, described below. The same kinds of comparisons arise in other functions, like `memq` and friends (see [List Searching](06_06_09_lists.md#6697-list-searching)).

For all three tests, objects of different types are never equal. So for instance a list and a vector are not `equal?`, even if their contents are the same. Exact and inexact numbers are considered different types too, and are hence not equal even if their values are the same.

`eq?` tests just for the same object (essentially a pointer comparison). This is fast, and can be used when searching for a particular object, or when working with symbols or keywords (which are always unique objects).

`eqv?` extends `eq?` to look at the value of numbers and characters. It can for instance be used somewhat like `=` (see [Comparison Predicates](06_06_02_numerical_data_types.md#6628-comparison-predicates)) but without an error if one operand isn’t a number.

`equal?` goes further, it looks (recursively) into the contents of lists, vectors, etc. This is good for instance on lists that have been read or calculated in various places and are the same, just not made up of the same pairs. Such lists look the same (when printed), and `equal?` will consider them the same.

  

Scheme Procedure: **eq?** … [¶](06_09_general_utility_functions.md)

C Function: **scm\_eq\_p** (x, y) [¶](06_09_general_utility_functions.md)

The Scheme procedure returns `#t` if all of its arguments are the same object, except for numbers and characters. The C function does the same but takes exactly two arguments. For example,

(define x (vector 1 2 3))
(define y (vector 1 2 3))

(eq? x x)  ⇒ #t
(eq? x y)  ⇒ #f

Numbers and characters are not equal to any other object, but the problem is they’re not necessarily `eq?` to themselves either. This is even so when the number comes directly from a variable,

(let ((n (+ 2 3)))
  (eq? n n))       ⇒ \*unspecified\*

Generally `eqv?` below should be used when comparing numbers or characters. `=` (see [Comparison Predicates](06_06_02_numerical_data_types.md#6628-comparison-predicates)) or `char=?` (see [Characters](06_06_03_characters.md#663-characters)) can be used too.

It’s worth noting that end-of-list `()`, `#t`, `#f`, a symbol of a given name, and a keyword of a given name, are unique objects. There’s just one of each, so for instance no matter how `()` arises in a program, it’s the same object and can be compared with `eq?`,

(define x (cdr '(123)))
(define y (cdr '(456)))
(eq? x y) ⇒ #t

(define x (string->symbol "foo"))
(eq? x 'foo) ⇒ #t

C Function: `int` **scm\_is\_eq** `(SCM x, SCM y)` [¶](06_09_general_utility_functions.md)

Return `1` when x and y are equal in the sense of `eq?`, otherwise return `0`.

The `==` operator should not be used on `SCM` values, an `SCM` is a C type which cannot necessarily be compared using `==` (see [The SCM Type](06_03_the_scm_type.md#63-the-scm-type)).

  

Scheme Procedure: **eqv?** … [¶](06_09_general_utility_functions.md)

C Function: **scm\_eqv\_p** (x, y) [¶](06_09_general_utility_functions.md)

The Scheme procedure returns `#t` if all of its arguments are the same object, or for characters and numbers the same value. The C function is similar but takes exactly two arguments.

On objects except characters and numbers, `eqv?` is the same as `eq?` above. `(eqv? x y)` is true if x and y are the same object.

If x and y are numbers or characters, `eqv?` compares their type and value. An exact number is not `eqv?` to an inexact number (even if their value is the same).

(eqv? 3 (+ 1 2)) ⇒ #t
(eqv? 1 1.0)     ⇒ #f

  

Scheme Procedure: **equal?** … [¶](06_09_general_utility_functions.md)

C Function: **scm\_equal\_p** (x, y) [¶](06_09_general_utility_functions.md)

The Scheme procedure returns `#t` if all of its arguments are the same type, and their contents or value are equal. The C function is similar, but takes exactly two arguments.

For a pair, string, vector, array or structure, `equal?` compares the contents, and does so using the same `equal?` recursively, so a deep structure can be traversed.

(equal? (list 1 2 3) (list 1 2 3))   ⇒ #t
(equal? (list 1 2 3) (vector 1 2 3)) ⇒ #f

For other objects, `equal?` compares as per `eqv?` above, which means characters and numbers are compared by type and value (and like `eqv?`, exact and inexact numbers are not `equal?`, even if their value is the same).

(equal? 3 (+ 1 2)) ⇒ #t
(equal? 1 1.0)     ⇒ #f

Hash tables are currently only compared as per `eq?`, so two different tables are not `equal?`, even if their contents are the same.

`equal?` does not support circular data structures, it may go into an infinite loop if asked to compare two circular lists or similar.

GOOPS object types (see [GOOPS](08_00_goops.md#8-goops)), including foreign object types (see [Defining New Foreign Object Types](05_programming_in_c.md#55-defining-new-foreign-object-types)), can have an `equal?` implementation specialized on two values of the same type. If `equal?` is called on two GOOPS objects of the same type, `equal?` will dispatch out to a generic function. This lets an application traverse the contents or control what is considered `equal?` for two objects of such a type. If there’s no such handler, the default is to just compare as per `eq?`.

* * *

Next: [Sorting](06_09_general_utility_functions.md#693-sorting), Previous: [Equality](06_09_general_utility_functions.md#691-equality), Up: [General Utility Functions](06_09_general_utility_functions.md#69-general-utility-functions)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.9.2 Object Properties [¶](06_09_general_utility_functions.md#692-object-properties)

It’s often useful to associate a piece of additional information with a Scheme object even though that object does not have a dedicated slot available in which the additional information could be stored. Object properties allow you to do just that.

Guile’s representation of an object property is a procedure-with-setter (see [Procedures with Setters](06_07_procedures.md#678-procedures-with-setters)) that can be used with the generalized form of `set!` to set and retrieve that property for any Scheme object. So, setting a property looks like this:

([set!](07_06_r6rs_support.md) (my-property obj1) value-for-obj1)
([set!](07_06_r6rs_support.md) (my-property obj2) value-for-obj2)

And retrieving values of the same property looks like this:

(my-property obj1)
⇒
value-for-obj1

(my-property obj2)
⇒
value-for-obj2

To create an object property in the first place, use the `make-object-property` procedure:

(define my-property ([make-object-property](06_09_general_utility_functions.md)))

Scheme Procedure: **make-object-property** [¶](06_09_general_utility_functions.md)

Create and return an object property. An object property is a procedure-with-setter that can be called in two ways. `(set! (property obj) val)` sets obj’s property to val. `(property obj)` returns the current setting of obj’s property.

A single object property created by `make-object-property` can associate distinct property values with all Scheme values that are distinguishable by `eq?` (ruling out numeric values).

Internally, object properties are implemented using a weak key hash table. This means that, as long as a Scheme value with property values is protected from garbage collection, its property values are also protected. When the Scheme value is collected, its entry in the property table is removed and so the (ex-) property values are no longer protected by the table.

Guile also implements a more traditional Lispy interface to properties, in which each object has an list of key-value pairs associated with it. Properties in that list are keyed by symbols. This is a legacy interface; you should use weak hash tables or object properties instead.

Scheme Procedure: **object-properties** obj [¶](06_09_general_utility_functions.md)

C Function: **scm\_object\_properties** (obj) [¶](06_09_general_utility_functions.md)

Return obj’s property list.

Scheme Procedure: **set-object-properties!** obj alist [¶](06_09_general_utility_functions.md)

C Function: **scm\_set\_object\_properties\_x** (obj, alist) [¶](06_09_general_utility_functions.md)

Set obj’s property list to alist.

Scheme Procedure: **object-property** obj key [¶](06_09_general_utility_functions.md)

C Function: **scm\_object\_property** (obj, key) [¶](06_09_general_utility_functions.md)

Return the property of obj with name key.

Scheme Procedure: **set-object-property!** obj key value [¶](06_09_general_utility_functions.md)

C Function: **scm\_set\_object\_property\_x** (obj, key, value) [¶](06_09_general_utility_functions.md)

In obj’s property list, set the property named key to value.

* * *

Next: [Copying Deep Structures](06_09_general_utility_functions.md#694-copying-deep-structures), Previous: [Object Properties](06_09_general_utility_functions.md#692-object-properties), Up: [General Utility Functions](06_09_general_utility_functions.md#69-general-utility-functions)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.9.3 Sorting [¶](06_09_general_utility_functions.md#693-sorting)

Sorting is very important in computer programs. Therefore, Guile comes with several sorting procedures built-in. As always, procedures with names ending in `!` are side-effecting, that means that they may modify their parameters in order to produce their results. The predicate less passed as second or third argument to the procedures below is assumed to define a strict weak order on the elements to be merged or sorted.

The first group of procedures can be used to merge two lists (which must be already sorted on their own) and produce sorted lists containing all elements of the input lists.

Scheme Procedure: **merge** alist blist less [¶](06_09_general_utility_functions.md)

C Function: **scm\_merge** (alist, blist, less) [¶](06_09_general_utility_functions.md)

Merge two already sorted lists into one. Given two lists alist and blist, such that `(sorted? alist less?)` and `(sorted? blist less?)`, return a new list in which the elements of alist and blist have been stably interleaved so that `(sorted? (merge alist blist less?) less?)`. Note: this does \_not\_ accept vectors.

Scheme Procedure: **merge!** alist blist less [¶](06_09_general_utility_functions.md)

C Function: **scm\_merge\_x** (alist, blist, less) [¶](06_09_general_utility_functions.md)

Takes two lists alist and blist such that `(sorted? alist less?)` and `(sorted? blist less?)` and returns a new list in which the elements of alist and blist have been stably interleaved so that `(sorted? (merge alist blist less?) less?)`. This is the destructive variant of `merge` Note: this does \_not\_ accept vectors.

The following procedures can operate on sequences which are either vectors or list. According to the given arguments, they return sorted vectors or lists, respectively. The first of the following procedures determines whether a sequence is already sorted, the other sort a given sequence. The variants with names starting with `stable-` are special in that they maintain a special property of the input sequences: If two or more elements are the same according to the comparison predicate, they are left in the same order as they appeared in the input.

Scheme Procedure: **sorted?** items less [¶](06_09_general_utility_functions.md)

C Function: **scm\_sorted\_p** (items, less) [¶](06_09_general_utility_functions.md)

Return `#t` if items is a list or vector such that, for each element x and the next element y of items, `(less y x)` returns `#f`. Otherwise return `#f`.

Scheme Procedure: **sort** items less [¶](06_09_general_utility_functions.md)

C Function: **scm\_sort** (items, less) [¶](06_09_general_utility_functions.md)

Sort the sequence items, which may be a list or a vector. less is used for comparing the sequence elements. This is not a stable sort.

Scheme Procedure: **sort!** items less [¶](06_09_general_utility_functions.md)

C Function: **scm\_sort\_x** (items, less) [¶](06_09_general_utility_functions.md)

Sort the sequence items, which may be a list or a vector. less is used for comparing the sequence elements. The sorting is destructive, that means that the input sequence is modified to produce the sorted result. This is not a stable sort.

Scheme Procedure: **stable-sort** items less [¶](06_09_general_utility_functions.md)

C Function: **scm\_stable\_sort** (items, less) [¶](06_09_general_utility_functions.md)

Sort the sequence items, which may be a list or a vector. less is used for comparing the sequence elements. This is a stable sort.

Scheme Procedure: **stable-sort!** items less [¶](06_09_general_utility_functions.md)

C Function: **scm\_stable\_sort\_x** (items, less) [¶](06_09_general_utility_functions.md)

Sort the sequence items, which may be a list or a vector. less is used for comparing the sequence elements. The sorting is destructive, that means that the input sequence is modified to produce the sorted result. This is a stable sort.

The procedures in the last group only accept lists or vectors as input, as their names indicate.

Scheme Procedure: **sort-list** items less [¶](06_09_general_utility_functions.md)

C Function: **scm\_sort\_list** (items, less) [¶](06_09_general_utility_functions.md)

Sort the list items, using less for comparing the list elements. This is a stable sort.

Scheme Procedure: **sort-list!** items less [¶](06_09_general_utility_functions.md)

C Function: **scm\_sort\_list\_x** (items, less) [¶](06_09_general_utility_functions.md)

Sort the list items, using less for comparing the list elements. The sorting is destructive, that means that the input list is modified to produce the sorted result. This is a stable sort.

Scheme Procedure: **restricted-vector-sort!** vec less startpos endpos [¶](06_09_general_utility_functions.md)

C Function: **scm\_restricted\_vector\_sort\_x** (vec, less, startpos, endpos) [¶](06_09_general_utility_functions.md)

Sort the vector vec, using less for comparing the vector elements. startpos (inclusively) and endpos (exclusively) delimit the range of the vector which gets sorted. The return value is not specified.

* * *

Next: [General String Conversion](06_09_general_utility_functions.md#695-general-string-conversion), Previous: [Sorting](06_09_general_utility_functions.md#693-sorting), Up: [General Utility Functions](06_09_general_utility_functions.md#69-general-utility-functions)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.9.4 Copying Deep Structures [¶](06_09_general_utility_functions.md#694-copying-deep-structures)

The procedures for copying lists (see [Lists](06_06_09_lists.md#669-lists)) only produce a flat copy of the input list, and currently Guile does not even contain procedures for copying vectors. The `(ice-9 copy-tree)` module contains a `copy-tree` function that can be used for this purpose, as it does not only copy the spine of a list, but also copies any pairs in the cars of the input lists.

(use-modules (ice-9 copy-tree))

Scheme Procedure: **copy-tree** obj [¶](06_09_general_utility_functions.md)

C Function: **scm\_copy\_tree** (obj) [¶](06_09_general_utility_functions.md)

Recursively copy the data tree that is bound to obj, and return the new data structure. `copy-tree` recurses down the contents of both pairs and vectors (since both cons cells and vector cells may point to arbitrary objects), and stops recursing when it hits any other object.

* * *

Next: [Hooks](06_09_general_utility_functions.md#696-hooks), Previous: [Copying Deep Structures](06_09_general_utility_functions.md#694-copying-deep-structures), Up: [General Utility Functions](06_09_general_utility_functions.md#69-general-utility-functions)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.9.5 General String Conversion [¶](06_09_general_utility_functions.md#695-general-string-conversion)

When debugging Scheme programs, but also for providing a human-friendly interface, a procedure for converting any Scheme object into string format is very useful. Conversion from/to strings can of course be done with specialized procedures when the data type of the object to convert is known, but with this procedure, it is often more comfortable.

`object->string` converts an object by using a print procedure for writing to a string port, and then returning the resulting string. Converting an object back from the string is only possible if the object type has a read syntax and the read syntax is preserved by the printing procedure.

Scheme Procedure: **object->string** obj \[printer\] [¶](06_09_general_utility_functions.md)

C Function: **scm\_object\_to\_string** (obj, printer) [¶](06_09_general_utility_functions.md)

Return a Scheme string obtained by printing obj. Printing function can be specified by the optional second argument printer (default: `write`).

* * *

Previous: [General String Conversion](06_09_general_utility_functions.md#695-general-string-conversion), Up: [General Utility Functions](06_09_general_utility_functions.md#69-general-utility-functions)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.9.6 Hooks [¶](06_09_general_utility_functions.md#696-hooks)

A hook is a list of procedures to be called at well defined points in time. Typically, an application provides a hook h and promises its users that it will call all of the procedures in h at a defined point in the application’s processing. By adding its own procedure to h, an application user can tap into or even influence the progress of the application.

Guile itself provides several such hooks for debugging and customization purposes: these are listed in a subsection below.

When an application first creates a hook, it needs to know how many arguments will be passed to the hook’s procedures when the hook is run. The chosen number of arguments (which may be none) is declared when the hook is created, and all the procedures that are added to that hook must be capable of accepting that number of arguments.

A hook is created using `make-hook`. A procedure can be added to or removed from a hook using `add-hook!` or `remove-hook!`, and all of a hook’s procedures can be removed together using `reset-hook!`. When an application wants to run a hook, it does so using `run-hook`.

*   [Hook Usage by Example](06_09_general_utility_functions.md#6961-hook-usage-by-example)
*   [Hook Reference](06_09_general_utility_functions.md#6962-hook-reference)
*   [Hooks For C Code.](06_09_general_utility_functions.md#6963-hooks-for-c-code)
*   [Hooks for Garbage Collection](06_09_general_utility_functions.md#6964-hooks-for-garbage-collection)
*   [Hooks into the Guile REPL](06_09_general_utility_functions.md#6965-hooks-into-the-guile-repl)

* * *

Next: [Hook Reference](06_09_general_utility_functions.md#6962-hook-reference), Up: [Hooks](06_09_general_utility_functions.md#696-hooks)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.9.6.1 Hook Usage by Example [¶](06_09_general_utility_functions.md#6961-hook-usage-by-example)

Hook usage is shown by some examples in this section. First, we will define a hook of arity 2 — that is, the procedures stored in the hook will have to accept two arguments.

(define hook ([make-hook](06_09_general_utility_functions.md) 2))
hook
⇒ #<hook 2 40286c90>

Now we are ready to add some procedures to the newly created hook with `add-hook!`. In the following example, two procedures are added, which print different messages and do different things with their arguments.

([add-hook!](06_09_general_utility_functions.md) hook (lambda (x y)
                    ([display](06_16_reading_and_evaluating_scheme_code.md) "Foo: ")
                    ([display](06_16_reading_and_evaluating_scheme_code.md) ([+](06_06_02_numerical_data_types.md) x y))
                    ([newline](06_12_input_and_output.md))))
([add-hook!](06_09_general_utility_functions.md) hook (lambda (x y)
                    ([display](06_16_reading_and_evaluating_scheme_code.md) "Bar: ")
                    ([display](06_16_reading_and_evaluating_scheme_code.md) ([\*](06_06_02_numerical_data_types.md) x y))
                    ([newline](06_12_input_and_output.md))))

Once the procedures have been added, we can invoke the hook using `run-hook`.

([run-hook](06_09_general_utility_functions.md) hook 3 4)
⊣ Bar: 12
⊣ Foo: 7

Note that the procedures are called in the reverse of the order with which they were added. This is because the default behavior of `add-hook!` is to add its procedure to the _front_ of the hook’s procedure list. You can force `add-hook!` to add its procedure to the _end_ of the list instead by providing a third `#t` argument on the second call to `add-hook!`.

([add-hook!](06_09_general_utility_functions.md) hook (lambda (x y)
                    ([display](06_16_reading_and_evaluating_scheme_code.md) "Foo: ")
                    ([display](06_16_reading_and_evaluating_scheme_code.md) ([+](06_06_02_numerical_data_types.md) x y))
                    ([newline](06_12_input_and_output.md))))
([add-hook!](06_09_general_utility_functions.md) hook (lambda (x y)
                    ([display](06_16_reading_and_evaluating_scheme_code.md) "Bar: ")
                    ([display](06_16_reading_and_evaluating_scheme_code.md) ([\*](06_06_02_numerical_data_types.md) x y))
                    ([newline](06_12_input_and_output.md)))
                    #t)             ; <- Change here!
([run-hook](06_09_general_utility_functions.md) hook 3 4)
⊣ Foo: 7
⊣ Bar: 12

* * *

Next: [Hooks For C Code.](06_09_general_utility_functions.md#6963-hooks-for-c-code), Previous: [Hook Usage by Example](06_09_general_utility_functions.md#6961-hook-usage-by-example), Up: [Hooks](06_09_general_utility_functions.md#696-hooks)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.9.6.2 Hook Reference [¶](06_09_general_utility_functions.md#6962-hook-reference)

When you create a hook with `make-hook`, you must specify the arity of the procedures which can be added to the hook. If the arity is not given explicitly as an argument to `make-hook`, it defaults to zero. All procedures of a given hook must have the same arity, and when the procedures are invoked using `run-hook`, the number of arguments passed must match the arity specified at hook creation time.

The order in which procedures are added to a hook matters. If the third parameter to `add-hook!` is omitted or is equal to `#f`, the procedure is added in front of the procedures which might already be on that hook, otherwise the procedure is added at the end. The procedures are always called from the front to the end of the list when they are invoked via `run-hook`.

The ordering of the list of procedures returned by `hook->list` matches the order in which those procedures would be called if the hook was run using `run-hook`.

Note that the C functions in the following entries are for handling _Scheme-level_ hooks in C. There are also _C-level_ hooks which have their own interface (see [Hooks For C Code.](06_09_general_utility_functions.md#6963-hooks-for-c-code)).

Scheme Procedure: **make-hook** \[n\_args\] [¶](06_09_general_utility_functions.md)

C Function: **scm\_make\_hook** (n\_args) [¶](06_09_general_utility_functions.md)

Create a hook for storing procedure of arity n\_args. n\_args defaults to zero. The returned value is a hook object to be used with the other hook procedures.

Scheme Procedure: **hook?** x [¶](06_09_general_utility_functions.md)

C Function: **scm\_hook\_p** (x) [¶](06_09_general_utility_functions.md)

Return `#t` if x is a hook, `#f` otherwise.

Scheme Procedure: **hook-empty?** hook [¶](06_09_general_utility_functions.md)

C Function: **scm\_hook\_empty\_p** (hook) [¶](06_09_general_utility_functions.md)

Return `#t` if hook is an empty hook, `#f` otherwise.

Scheme Procedure: **add-hook!** hook proc \[append\_p\] [¶](06_09_general_utility_functions.md)

C Function: **scm\_add\_hook\_x** (hook, proc, append\_p) [¶](06_09_general_utility_functions.md)

Add the procedure proc to the hook hook. The procedure is added to the end if append\_p is true, otherwise it is added to the front. The return value of this procedure is not specified.

Scheme Procedure: **remove-hook!** hook proc [¶](06_09_general_utility_functions.md)

C Function: **scm\_remove\_hook\_x** (hook, proc) [¶](06_09_general_utility_functions.md)

Remove the procedure proc from the hook hook. The return value of this procedure is not specified.

Scheme Procedure: **reset-hook!** hook [¶](06_09_general_utility_functions.md)

C Function: **scm\_reset\_hook\_x** (hook) [¶](06_09_general_utility_functions.md)

Remove all procedures from the hook hook. The return value of this procedure is not specified.

Scheme Procedure: **hook->list** hook [¶](06_09_general_utility_functions.md)

C Function: **scm\_hook\_to\_list** (hook) [¶](06_09_general_utility_functions.md)

Convert the procedure list of hook to a list.

Scheme Procedure: **run-hook** hook arg … [¶](06_09_general_utility_functions.md)

C Function: **scm\_run\_hook** (hook, args) [¶](06_09_general_utility_functions.md)

Apply all procedures from the hook hook to the arguments arg .... The order of the procedure application is first to last. The return value of this procedure is not specified.

If, in C code, you are certain that you have a hook object and well formed argument list for that hook, you can also use `scm_c_run_hook`, which is identical to `scm_run_hook` but does no type checking.

C Function: `void` **scm\_c\_run\_hook** `(SCM hook, SCM args)` [¶](06_09_general_utility_functions.md)

The same as `scm_run_hook` but without any type checking to confirm that hook is actually a hook object and that args is a well-formed list matching the arity of the hook.

For C code, `SCM_HOOKP` is a faster alternative to `scm_hook_p`:

C Macro: `int` **SCM\_HOOKP** `(x)` [¶](06_09_general_utility_functions.md)

Return 1 if x is a Scheme-level hook, 0 otherwise.

* * *

Next: [Hooks for Garbage Collection](06_09_general_utility_functions.md#6964-hooks-for-garbage-collection), Previous: [Hook Reference](06_09_general_utility_functions.md#6962-hook-reference), Up: [Hooks](06_09_general_utility_functions.md#696-hooks)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.9.6.3 Hooks For C Code. [¶](06_09_general_utility_functions.md#6963-hooks-for-c-code)

The hooks already described are intended to be populated by Scheme-level procedures. In addition to this, the Guile library provides an independent set of interfaces for the creation and manipulation of hooks that are designed to be populated by functions implemented in C.

The original motivation here was to provide a kind of hook that could safely be invoked at various points during garbage collection. Scheme-level hooks are unsuitable for this purpose as running them could itself require memory allocation, which would then invoke garbage collection recursively … However, it is also the case that these hooks are easier to work with than the Scheme-level ones if you only want to register C functions with them. So if that is mainly what your code needs to do, you may prefer to use this interface.

To create a C hook, you should allocate storage for a structure of type `scm_t_c_hook` and then initialize it using `scm_c_hook_init`.

C Type: **scm\_t\_c\_hook** [¶](06_09_general_utility_functions.md)

Data type for a C hook. The internals of this type should be treated as opaque.

C Enum: **scm\_t\_c\_hook\_type** [¶](06_09_general_utility_functions.md)

Enumeration of possible hook types, which are:

`SCM_C_HOOK_NORMAL` [¶](06_09_general_utility_functions.md)

Type of hook for which all the registered functions will always be called.

`SCM_C_HOOK_OR` [¶](06_09_general_utility_functions.md)

Type of hook for which the sequence of registered functions will be called only until one of them returns C true (a non-NULL pointer).

`SCM_C_HOOK_AND` [¶](06_09_general_utility_functions.md)

Type of hook for which the sequence of registered functions will be called only until one of them returns C false (a NULL pointer).

C Function: `void` **scm\_c\_hook\_init** `(scm_t_c_hook *hook, void *hook_data, scm_t_c_hook_type type)` [¶](06_09_general_utility_functions.md)

Initialize the C hook at memory pointed to by hook. type should be one of the values of the `scm_t_c_hook_type` enumeration, and controls how the hook functions will be called. hook\_data is a closure parameter that will be passed to all registered hook functions when they are called.

To add or remove a C function from a C hook, use `scm_c_hook_add` or `scm_c_hook_remove`. A hook function must expect three `void *` parameters which are, respectively:

hook\_data

The hook closure data that was specified at the time the hook was initialized by `scm_c_hook_init`.

func\_data

The function closure data that was specified at the time that that function was registered with the hook by `scm_c_hook_add`.

data

The call closure data specified by the `scm_c_hook_run` call that runs the hook.

C Type: **scm\_t\_c\_hook\_function** [¶](06_09_general_utility_functions.md)

Function type for a C hook function: takes three `void *` parameters and returns a `void *` result.

C Function: `void` **scm\_c\_hook\_add** `(scm_t_c_hook *hook, scm_t_c_hook_function func, void *func_data, int appendp)` [¶](06_09_general_utility_functions.md)

Add function func, with function closure data func\_data, to the C hook hook. The new function is appended to the hook’s list of functions if appendp is non-zero, otherwise prepended.

C Function: `void` **scm\_c\_hook\_remove** `(scm_t_c_hook *hook, scm_t_c_hook_function func, void *func_data)` [¶](06_09_general_utility_functions.md)

Remove function func, with function closure data func\_data, from the C hook hook. `scm_c_hook_remove` checks both func and func\_data so as to allow for the same func being registered multiple times with different closure data.

Finally, to invoke a C hook, call the `scm_c_hook_run` function specifying the hook and the call closure data for this run:

C Function: `void *` **scm\_c\_hook\_run** `(scm_t_c_hook *hook, void *data)` [¶](06_09_general_utility_functions.md)

Run the C hook hook will call closure data data. Subject to the variations for hook types `SCM_C_HOOK_OR` and `SCM_C_HOOK_AND`, `scm_c_hook_run` calls hook’s registered functions in turn, passing them the hook’s closure data, each function’s closure data, and the call closure data.

`scm_c_hook_run`’s return value is the return value of the last function to be called.

* * *

Next: [Hooks into the Guile REPL](06_09_general_utility_functions.md#6965-hooks-into-the-guile-repl), Previous: [Hooks For C Code.](06_09_general_utility_functions.md#6963-hooks-for-c-code), Up: [Hooks](06_09_general_utility_functions.md#696-hooks)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.9.6.4 Hooks for Garbage Collection [¶](06_09_general_utility_functions.md#6964-hooks-for-garbage-collection)

Whenever Guile performs a garbage collection, it calls the following hooks in the order shown.

C Hook: **scm\_before\_gc\_c\_hook** [¶](06_09_general_utility_functions.md)

C hook called at the very start of a garbage collection, after setting `scm_gc_running_p` to 1, but before entering the GC critical section.

If garbage collection is blocked because `scm_block_gc` is non-zero, GC exits early soon after calling this hook, and no further hooks will be called.

C Hook: **scm\_before\_mark\_c\_hook** [¶](06_09_general_utility_functions.md)

C hook called before beginning the mark phase of garbage collection, after the GC thread has entered a critical section.

C Hook: **scm\_before\_sweep\_c\_hook** [¶](06_09_general_utility_functions.md)

C hook called before beginning the sweep phase of garbage collection. This is the same as at the end of the mark phase, since nothing else happens between marking and sweeping.

C Hook: **scm\_after\_sweep\_c\_hook** [¶](06_09_general_utility_functions.md)

C hook called after the end of the sweep phase of garbage collection, but while the GC thread is still inside its critical section.

C Hook: **scm\_after\_gc\_c\_hook** [¶](06_09_general_utility_functions.md)

C hook called at the very end of a garbage collection, after the GC thread has left its critical section.

Scheme Hook: **after-gc-hook** [¶](06_09_general_utility_functions.md)

Scheme hook with arity 0. This hook is run asynchronously (see [Asynchronous Interrupts](06_22_threads_mutexes_asyncs_and_dynamic_roots.md#6223-asynchronous-interrupts)) soon after the GC has completed and any other events that were deferred during garbage collection have been processed. (Also accessible from C with the name `scm_after_gc_hook`.)

All the C hooks listed here have type `SCM_C_HOOK_NORMAL`, are initialized with hook closure data NULL, are invoked by `scm_c_hook_run` with call closure data NULL.

The Scheme hook `after-gc-hook` is particularly useful in conjunction with guardians (see [Guardians](06_17_memory_management_and_garbage_collection.md#6174-guardians)). Typically, if you are using a guardian, you want to call the guardian after garbage collection to see if any of the objects added to the guardian have been collected. By adding a thunk that performs this call to `after-gc-hook`, you can ensure that your guardian is tested after every garbage collection cycle.

* * *

Previous: [Hooks for Garbage Collection](06_09_general_utility_functions.md#6964-hooks-for-garbage-collection), Up: [Hooks](06_09_general_utility_functions.md#696-hooks)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.9.6.5 Hooks into the Guile REPL [¶](06_09_general_utility_functions.md#6965-hooks-into-the-guile-repl)

* * *

Next: [Controlling the Flow of Program Execution](06_11_controlling_the_flow_of_program_execution.md#611-controlling-the-flow-of-program-execution), Previous: [General Utility Functions](06_09_general_utility_functions.md#69-general-utility-functions), Up: [API Reference](06_00_api_reference.md#6-api-reference)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
