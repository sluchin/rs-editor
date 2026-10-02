### 9.3 A Virtual Machine for Guile [¶](09_03_a_virtual_machine_for_guile.md#93-a-virtual-machine-for-guile)

Enough about data—how does Guile run code?

Code is a grammatical production of a language. Sometimes these languages are implemented using interpreters: programs that run along-side the program being interpreted, dynamically translating the high-level code to low-level code. Sometimes these languages are implemented using compilers: programs that translate high-level programs to equivalent low-level code, and pass on that low-level code to some other language implementation. Each of these languages can be thought to be virtual machines: they offer programs an abstract machine on which to run.

Guile implements a number of interpreters and compilers on different language levels. For example, there is an interpreter for the Scheme language that is itself implemented as a Scheme program compiled to a bytecode for a low-level virtual machine shipped with Guile. That virtual machine is implemented by both an interpreter—a C program that interprets the bytecodes—and a compiler—a C program that dynamically translates bytecode programs to native machine code[37](99_footnotes.md).

This section describes the language implemented by Guile’s bytecode virtual machine, as well as some examples of translations of Scheme programs to Guile’s VM.

*   [Why a VM?](09_03_a_virtual_machine_for_guile.md#931-why-a-vm)
*   [VM Concepts](09_03_a_virtual_machine_for_guile.md#932-vm-concepts)
*   [Stack Layout](09_03_a_virtual_machine_for_guile.md#933-stack-layout)
*   [Variables and the VM](09_03_a_virtual_machine_for_guile.md#934-variables-and-the-vm)
*   [Compiled Procedures are VM Programs](09_03_a_virtual_machine_for_guile.md#935-compiled-procedures-are-vm-programs)
*   [Object File Format](09_03_a_virtual_machine_for_guile.md#936-object-file-format)
*   [Instruction Set](09_03_a_virtual_machine_for_guile.md#937-instruction-set)
*   [Just-In-Time Native Code](09_03_a_virtual_machine_for_guile.md#938-just-in-time-native-code)

* * *

Next: [VM Concepts](09_03_a_virtual_machine_for_guile.md#932-vm-concepts), Up: [A Virtual Machine for Guile](09_03_a_virtual_machine_for_guile.md#93-a-virtual-machine-for-guile)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 9.3.1 Why a VM? [¶](09_03_a_virtual_machine_for_guile.md#931-why-a-vm)

For a long time, Guile only had a Scheme interpreter, implemented in C. Guile’s interpreter operated directly on the S-expression representation of Scheme source code.

But while the interpreter was highly optimized and hand-tuned, it still performed many needless computations during the course of evaluating a Scheme expression. For example, application of a function to arguments needlessly consed up the arguments in a list. Evaluation of an expression like `(f x y)` always had to figure out whether f was a procedure, or a special form like `if`, or something else. The interpreter represented the lexical environment as a heap data structure, so every evaluation caused allocation, which was of course slow. Et cetera.

The solution to the slow-interpreter problem was to compile the higher-level language, Scheme, into a lower-level language for which all of the checks and dispatching have already been done—the code is instead stripped to the bare minimum needed to “do the job”.

The question becomes then, what low-level language to choose? There are many options. We could compile to native code directly, but that poses portability problems for Guile, as it is a highly cross-platform project.

So we want the performance gains that compilation provides, but we also want to maintain the portability benefits of a single code path. The obvious solution is to compile to a virtual machine that is present on all Guile installations.

The easiest (and most fun) way to depend on a virtual machine is to implement the virtual machine within Guile itself. Guile contains a bytecode interpreter (written in C) and a Scheme to bytecode compiler (written in Scheme). This way the virtual machine provides what Scheme needs (tail calls, multiple values, `call/cc`) and can provide optimized inline instructions for Guile as well (GC-managed allocations, type checks, etc.).

Guile also includes a just-in-time (JIT) compiler to translate bytecode to native code. Because Guile embeds a portable code generation library ([https://gitlab.com/wingo/lightening](https://gitlab.com/wingo/lightening)), we keep the benefits of portability while also benefitting from fast native code. To avoid too much time spent in the JIT compiler itself, Guile is tuned to only emit machine code for bytecode that is called often.

The rest of this section describes that VM that Guile implements, and the compiled procedures that run on it.

Before moving on, though, we should note that though we spoke of the interpreter in the past tense, Guile still has an interpreter. The difference is that before, it was Guile’s main Scheme implementation, and so was implemented in highly optimized C; now, it is actually implemented in Scheme, and compiled down to VM bytecode, just like any other program. (There is still a C interpreter around, used to bootstrap the compiler, but it is not normally used at runtime.)

The upside of implementing the interpreter in Scheme is that we preserve tail calls and multiple-value handling between interpreted and compiled code, and with advent of the JIT compiler in Guile 3.0 we reach the speed of the old hand-tuned C implementation; it’s the best of both worlds.

Also note that this decision to implement a bytecode compiler does not preclude ahead-of-time native compilation. More possibilities are discussed in [Extending the Compiler](09_04_compiling_to_the_virtual_machine.md#947-extending-the-compiler).

* * *

Next: [Stack Layout](09_03_a_virtual_machine_for_guile.md#933-stack-layout), Previous: [Why a VM?](09_03_a_virtual_machine_for_guile.md#931-why-a-vm), Up: [A Virtual Machine for Guile](09_03_a_virtual_machine_for_guile.md#93-a-virtual-machine-for-guile)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 9.3.2 VM Concepts [¶](09_03_a_virtual_machine_for_guile.md#932-vm-concepts)

The bytecode in a Scheme procedure is interpreted by a virtual machine (VM). Each thread has its own instantiation of the VM. The virtual machine executes the sequence of instructions in a procedure.

Each VM instruction starts by indicating which operation it is, and then follows by encoding its source and destination operands. Each procedure declares that it has some number of local variables, including the function arguments. These local variables form the available operands of the procedure, and are accessed by index.

The local variables for a procedure are stored on a stack. Calling a procedure typically enlarges the stack, and returning from a procedure shrinks it. Stack memory is exclusive to the virtual machine that owns it.

In addition to their stacks, virtual machines also have access to the global memory (modules, global bindings, etc) that is shared among other parts of Guile, including other VMs.

The registers that a VM has are as follows:

*   ip - Instruction pointer
*   sp - Stack pointer
*   fp - Frame pointer

In other architectures, the instruction pointer is sometimes called the “program counter” (pc). This set of registers is pretty typical for virtual machines; their exact meanings in the context of Guile’s VM are described in the next section.

* * *

Next: [Variables and the VM](09_03_a_virtual_machine_for_guile.md#934-variables-and-the-vm), Previous: [VM Concepts](09_03_a_virtual_machine_for_guile.md#932-vm-concepts), Up: [A Virtual Machine for Guile](09_03_a_virtual_machine_for_guile.md#93-a-virtual-machine-for-guile)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 9.3.3 Stack Layout [¶](09_03_a_virtual_machine_for_guile.md#933-stack-layout)

The stack of Guile’s virtual machine is composed of _frames_. Each frame corresponds to the application of one compiled procedure, and contains storage space for arguments, local variables, and some bookkeeping information (such as what to do after the frame is finished).

While the compiler is free to do whatever it wants to, as long as the semantics of a computation are preserved, in practice every time you call a function, a new frame is created. (The notable exception of course is the tail call case, see [Tail calls](03_hello_scheme.md#332-tail-calls).)

The structure of the top stack frame is as follows:

   | ...previous frame locals...  |
   +==============================+ <- fp + 3
   | Dynamic link                 |
   +------------------------------+
   | Virtual return address (vRA) |
   +------------------------------+
   | Machine return address (mRA) |
   +==============================+ <- fp
   | Local 0                      |
   +------------------------------+
   | Local 1                      |
   +------------------------------+
   | ...                          |
   +------------------------------+
   | Local N-1                    |
   \\------------------------------/ <- sp

In the above drawing, the stack grows downward. At the beginning of a function call, the procedure being applied is in local 0, followed by the arguments from local 1. After the procedure checks that it is being passed a compatible set of arguments, the procedure allocates some additional space in the frame to hold variables local to the function.

Note that once a value in a local variable slot is no longer needed, Guile is free to re-use that slot. This applies to the slots that were initially used for the callee and arguments, too. For this reason, backtraces in Guile aren’t always able to show all of the arguments: it could be that the slot corresponding to that argument was re-used by some other variable.

The _virtual return address_ is the `ip` that was in effect before this program was applied. When we return from this activation frame, we will jump back to this `ip`. Likewise, the _dynamic link_ is the offset of the `fp` that was in effect before this program was applied, relative to the current `fp`.

There are two return addresses: the virtual return address (vRA), and the machine return address (mRA). The vRA is always present and indicates a bytecode address. The mRA is only present when a call is made from a function with machine code (e.g. a function that has been JIT-compiled).

To prepare for a non-tail application, Guile’s VM will emit code that shuffles the function to apply and its arguments into appropriate stack slots, with three free slots below them. The call then initializes those free slots to hold the machine return address (or NULL), the virtual return address, and the offset to the previous frame pointer (`fp`). It then gets the `ip` for the function being called and adjusts `fp` to point to the new call frame.

In this way, the dynamic link links the current frame to the previous frame. Computing a stack trace involves traversing these frames.

Each stack local in Guile is 64 bits wide, even on 32-bit architectures. This allows Guile to preserve its uniform treatment of stack locals while allowing for unboxed arithmetic on 64-bit integers and floating-point numbers. See [Instruction Set](09_03_a_virtual_machine_for_guile.md#937-instruction-set), for more on unboxed arithmetic.

As an implementation detail, we actually store the dynamic link as an offset and not an absolute value because the stack can move at runtime as it expands or during partial continuation calls. If it were an absolute value, we would have to walk the frames, relocating frame pointers.

* * *

Next: [Compiled Procedures are VM Programs](09_03_a_virtual_machine_for_guile.md#935-compiled-procedures-are-vm-programs), Previous: [Stack Layout](09_03_a_virtual_machine_for_guile.md#933-stack-layout), Up: [A Virtual Machine for Guile](09_03_a_virtual_machine_for_guile.md#93-a-virtual-machine-for-guile)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 9.3.4 Variables and the VM [¶](09_03_a_virtual_machine_for_guile.md#934-variables-and-the-vm)

Consider the following Scheme code as an example:

  (define (foo a)
    (lambda (b) (vector foo a b)))

Within the lambda expression, `foo` is a top-level variable, `a` is a lexically captured variable, and `b` is a local variable.

Another way to refer to `a` and `b` is to say that `a` is a “free” variable, since it is not defined within the lambda, and `b` is a “bound” variable. These are the terms used in the _lambda calculus_, a mathematical notation for describing functions. The lambda calculus is useful because it is a language in which to reason precisely about functions and variables. It is especially good at describing scope relations, and it is for that reason that we mention it here.

Guile allocates all variables on the stack. When a lexically enclosed procedure with free variables—a _closure_—is created, it copies those variables into its free variable vector. References to free variables are then redirected through the free variable vector.

If a variable is ever `set!`, however, it will need to be heap-allocated instead of stack-allocated, so that different closures that capture the same variable can see the same value. Also, this allows continuations to capture a reference to the variable, instead of to its value at one point in time. For these reasons, `set!` variables are allocated in “boxes”—actually, in variable cells. See [Variables](06_18_modules.md#6187-variables), for more information. References to `set!` variables are indirected through the boxes.

Thus perhaps counterintuitively, what would seem “closer to the metal”, viz `set!`, actually forces an extra memory allocation and indirection. Sometimes Guile’s optimizer can remove this allocation, but not always.

Going back to our example, `b` may be allocated on the stack, as it is never mutated.

`a` may also be allocated on the stack, as it too is never mutated. Within the enclosed lambda, its value will be copied into (and referenced from) the free variables vector.

`foo` is a top-level variable, because `foo` is not lexically bound in this example.

* * *

Next: [Object File Format](09_03_a_virtual_machine_for_guile.md#936-object-file-format), Previous: [Variables and the VM](09_03_a_virtual_machine_for_guile.md#934-variables-and-the-vm), Up: [A Virtual Machine for Guile](09_03_a_virtual_machine_for_guile.md#93-a-virtual-machine-for-guile)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 9.3.5 Compiled Procedures are VM Programs [¶](09_03_a_virtual_machine_for_guile.md#935-compiled-procedures-are-vm-programs)

By default, when you enter in expressions at Guile’s REPL, they are first compiled to bytecode. Then that bytecode is executed to produce a value. If the expression evaluates to a procedure, the result of this process is a compiled procedure.

A compiled procedure is a compound object consisting of its bytecode and a reference to any captured lexical variables. In addition, when a procedure is compiled, it has associated metadata written to side tables, for instance a line number mapping, or its docstring. You can pick apart these pieces with the accessors in `(system vm program)`. See [Compiled Procedures](06_07_procedures.md#673-compiled-procedures), for a full API reference.

A procedure may reference data that was statically allocated when the procedure was compiled. For example, a pair of immediate objects (see [Immediate Objects](09_02_data_representation.md#9252-immediate-objects)) can be allocated directly in the memory segment that contains the compiled bytecode, and accessed directly by the bytecode.

Another use for statically allocated data is to serve as a cache for a bytecode. Top-level variable lookups are handled in this way; the first time a top-level binding is referenced, the resolved variable will be stored in a cache. Thereafter all access to the variable goes through the cache cell. The variable’s value may change in the future, but the variable itself will not.

We can see how these concepts tie together by disassembling the `foo` function we defined earlier to see what is going on:

scheme@(guile-user)> (define (foo a) (lambda (b) (vector foo a b)))
scheme@(guile-user)> ,x foo
Disassembly of #<procedure foo (a)> at #xf1da30:

   0    (instrument-entry 164)                                at (unknown file):5:0
   2    (assert-nargs-ee/locals 2 1)    ;; 3 slots (1 arg)
   3    (allocate-words/immediate 2 3)                        at (unknown file):5:16
   4    (load-u64 0 0 65605)
   7    (word-set!/immediate 2 0 0)
   8    (load-label 0 7)                ;; anonymous procedure at #xf1da6c
  10    (word-set!/immediate 2 1 0)
  11    (scm-set!/immediate 2 2 1)
  12    (reset-frame 1)                 ;; 1 slot
  13    (handle-interrupts)
  14    (return-values)

----------------------------------------
Disassembly of anonymous procedure at #xf1da6c:

   0    (instrument-entry 183)                                at (unknown file):5:16
   2    (assert-nargs-ee/locals 2 3)    ;; 5 slots (1 arg)
   3    (static-ref 2 152)              ;; #<variable 112e530 value: #<procedure foo (a)>>
   5    (immediate-tag=? 2 7 0)         ;; heap-object?
   7    (je 19)                         ;; -> L2
   8    (static-ref 2 119)              ;; #<directory (guile-user) ca9750>
  10    (static-ref 1 127)              ;; foo
  12    (call-scm<-scm-scm 2 2 1 40)
  14    (immediate-tag=? 2 7 0)         ;; heap-object?
  16    (jne 8)                         ;; -> L1
  17    (scm-ref/immediate 0 2 1)
  18    (immediate-tag=? 0 4095 2308)   ;; undefined?
  20    (je 4)                          ;; -> L1
  21    (static-set! 2 134)             ;; #<variable 112e530 value: #<procedure foo (a)>>
  23    (j 3)                           ;; -> L2
L1:
  24    (throw/value 1 151)             ;; #(unbound-variable #f "Unbound variable: ~S")
L2:
  26    (scm-ref/immediate 2 2 1)
  27    (allocate-words/immediate 1 4)                        at (unknown file):5:28
  28    (load-u64 0 0 781)
  31    (word-set!/immediate 1 0 0)
  32    (scm-set!/immediate 1 1 2)
  33    (scm-ref/immediate 4 4 2)
  34    (scm-set!/immediate 1 2 4)
  35    (scm-set!/immediate 1 3 3)
  36    (mov 4 1)
  37    (reset-frame 1)                 ;; 1 slot
  38    (handle-interrupts)
  39    (return-values)

The first thing to notice is that the bytecode is at a fairly low level. When a program is compiled from Scheme to bytecode, it is expressed in terms of more primitive operations. As such, there can be more instructions than you might expect.

The first chunk of instructions is the outer `foo` procedure. It is followed by the code for the contained closure. The code can look daunting at first glance, but with practice it quickly becomes comprehensible, and indeed being able to read bytecode is an important step to understanding the low-level performance of Guile programs.

The `foo` function begins with a prelude. The `instrument-entry` bytecode increments a counter associated with the function. If the counter reaches a certain threshold, Guile will emit machine code (“JIT-compile”) for `foo`. Emitting machine code is fairly cheap but it does take time, so it’s not something you want to do for every function. Using a per-function counter and a global threshold allows Guile to spend time JIT-compiling only the “hot” functions.

Next in the prelude is an argument-checking instruction, which checks that it was called with only 1 argument (plus the callee function itself makes 2) and then reserves stack space for an additional 1 local.

Then from `ip` 3 to 11, we allocate a new closure by allocating a three-word object, initializing its first word to store a type tag, setting its second word to its code pointer, and finally at `ip` 11, storing local value 1 (the `a` argument) into the third word (the first free variable).

Before returning, `foo` “resets the frame” to hold only one local (the return value), runs any pending interrupts (see [Asynchronous Interrupts](06_22_threads_mutexes_asyncs_and_dynamic_roots.md#6223-asynchronous-interrupts)) and then returns.

Note that local variables in Guile’s virtual machine are usually addressed relative to the stack pointer, which leads to a pleasantly efficient `sp[n]` access. However it can make the disassembly hard to read, because the `sp` can change during the function, and because incoming arguments are relative to the `fp`, not the `sp`.

To know what `fp`\-relative slot corresponds to an `sp`\-relative reference, scan up in the disassembly until you get to a “n slots” annotation; in our case, 3, indicating that the frame has space for 3 slots. Thus a zero-indexed `sp`\-relative slot of 2 corresponds to the `fp`\-relative slot of 0, which initially held the value of the closure being called. This means that Guile doesn’t need the value of the closure to compute its result, and so slot 0 was free for re-use, in this case for the result of making a new closure.

A closure is code with data. As you can see, making the closure involved making an object (`ip` 3), putting a code pointer in it (`ip` 8 and 10), and putting in the closure’s free variable (`ip` 11).

The second stanza disassembles the code for the closure. After the prelude, all of the code between `ip` 5 and 24 is related to loading the toplevel variable `foo` into slot 1. This lookup happens only once, and is associated with a cache; after the first run, the value in the cache will be a bound variable, and the code will jump from `ip` 7 to 26. On the first run, Guile gets the module associated with the function, calls out to a run-time routine to look up the variable, and checks that the variable is bound before initializing the cache. Either way, `ip` 26 dereferences the variable into local 2.

What follows is the allocation and initialization of the vector return value. `Ip` 27 does the allocation, and the following two instructions initialize the type-and-length tag for the object’s first word. `Ip` 32 sets word 1 of the object (the first vector slot) to the value of `foo`; `ip` 33 fetches the closure variable for `a`, then in `ip` 34 stores it in the second vector slot; and finally, in `ip` 35, local `b` is stored to the third vector slot. This is followed by the return sequence.

* * *

Next: [Instruction Set](09_03_a_virtual_machine_for_guile.md#937-instruction-set), Previous: [Compiled Procedures are VM Programs](09_03_a_virtual_machine_for_guile.md#935-compiled-procedures-are-vm-programs), Up: [A Virtual Machine for Guile](09_03_a_virtual_machine_for_guile.md#93-a-virtual-machine-for-guile)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 9.3.6 Object File Format [¶](09_03_a_virtual_machine_for_guile.md#936-object-file-format)

To compile a file to disk, we need a format in which to write the compiled code to disk, and later load it into Guile. A good _object file format_ has a number of characteristics:

*   Above all else, it should be very cheap to load a compiled file.
*   It should be possible to statically allocate constants in the file. For example, a bytevector literal in source code can be emitted directly into the object file.
*   The compiled file should enable maximum code and data sharing between different processes.
*   The compiled file should contain debugging information, such as line numbers, but that information should be separated from the code itself. It should be possible to strip debugging information if space is tight.

These characteristics are not specific to Scheme. Indeed, mainstream languages like C and C++ have solved this issue many times in the past. Guile builds on their work by adopting ELF, the object file format of GNU and other Unix-like systems, as its object file format. Although Guile uses ELF on all platforms, we do not use platform support for ELF. Guile implements its own linker and loader. The advantage of using ELF is not sharing code, but sharing ideas. ELF is simply a well-designed object file format.

An ELF file has two meta-tables describing its contents. The first meta-table is for the loader, and is called the _program table_ or sometimes the _segment table_. The program table divides the file into big chunks that should be treated differently by the loader. Mostly the difference between these _segments_ is their permissions.

Typically all segments of an ELF file are marked as read-only, except that part that represents modifiable static data or static data that needs load-time initialization. Loading an ELF file is as simple as mmapping the thing into memory with read-only permissions, then using the segment table to mark a small sub-region of the file as writable. This writable section is typically added to the root set of the garbage collector as well.

One ELF segment is marked as “dynamic”, meaning that it has data of interest to the loader. Guile uses this segment to record the Guile version corresponding to this file. There is also an entry in the dynamic segment that points to the address of an initialization thunk that is run to perform any needed link-time initialization. (This is like dynamic relocations for normal ELF shared objects, except that we compile the relocations as a procedure instead of having the loader interpret a table of relocations.) Finally, the dynamic segment marks the location of the “entry thunk” of the object file. This thunk is returned to the caller of `load-thunk-from-memory` or `load-thunk-from-file`. When called, it will execute the “body” of the compiled expression.

The other meta-table in an ELF file is the _section table_. Whereas the program table divides an ELF file into big chunks for the loader, the section table specifies small sections for use by introspective tools like debuggers or the like. One segment (program table entry) typically contains many sections. There may be sections outside of any segment, as well.

Typical sections in a Guile `.go` file include:

`.rtl-text`

Bytecode.

`.data`

Data that needs initialization, or which may be modified at runtime.

`.rodata`

Statically allocated data that needs no run-time initialization, and which therefore can be shared between processes.

`.dynamic`

The dynamic section, discussed above.

`.symtab`

`.strtab`

A table mapping addresses in the `.rtl-text` to procedure names. `.strtab` is used by `.symtab`.

`.guile.procprops`

`.guile.arities`

`.guile.arities.strtab`

`.guile.docstrs`

`.guile.docstrs.strtab`

Side tables of procedure properties, arities, and docstrings.

`.guile.docstrs.strtab`

Side table of frame maps, describing the set of live slots for ever return point in the program text, and whether those slots are pointers are not. Used by the garbage collector.

`.debug_info`

`.debug_abbrev`

`.debug_str`

`.debug_loc`

`.debug_line`

Debugging information, in DWARF format. See the DWARF specification, for more information.

`.shstrtab`

Section name string table.

For more information, see [the elf(5) man page](http://linux.die.net/man/5/elf). See [the DWARF specification](http://dwarfstd.org/) for more on the DWARF debugging format. Or if you are an adventurous explorer, try running `readelf` or `objdump` on compiled `.go` files. It’s good times!

* * *

Next: [Just-In-Time Native Code](09_03_a_virtual_machine_for_guile.md#938-just-in-time-native-code), Previous: [Object File Format](09_03_a_virtual_machine_for_guile.md#936-object-file-format), Up: [A Virtual Machine for Guile](09_03_a_virtual_machine_for_guile.md#93-a-virtual-machine-for-guile)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 9.3.7 Instruction Set [¶](09_03_a_virtual_machine_for_guile.md#937-instruction-set)

There are currently about 150 instructions in Guile’s virtual machine. These instructions represent atomic units of a program’s execution. Ideally, they perform one task without conditional branches, then dispatch to the next instruction in the stream.

Instructions themselves are composed of 1 or more 32-bit units. The low 8 bits of the first word indicate the opcode, and the rest of instruction describe the operands. There are a number of different ways operands can be encoded.

`sn`

An unsigned n\-bit integer, indicating the `sp`\-relative index of a local variable.

`fn`

An unsigned n\-bit integer, indicating the `fp`\-relative index of a local variable. Used when a continuation accepts a variable number of values, to shuffle received values into known locations in the frame.

`cn`

An unsigned n\-bit integer, indicating a constant value.

`l24`

An offset from the current `ip`, in 32-bit units, as a signed 24-bit value. Indicates a bytecode address, for a relative jump.

`zi16`

`i16`

`i32`

An immediate Scheme value (see [Immediate Objects](09_02_data_representation.md#9252-immediate-objects)), encoded directly in 16 or 32 bits. `zi16` is sign-extended; the others are zero-extended.

`a32`

`b32`

An immediate Scheme value, encoded as a pair of 32-bit words. `a32` and `b32` values always go together on the same opcode, and indicate the high and low bits, respectively. Normally only used on 64-bit systems.

`n32`

A statically allocated non-immediate. The address of the non-immediate is encoded as a signed 32-bit integer, and indicates a relative offset in 32-bit units. Think of it as `SCM x = ip + offset`.

`r32`

Indirect scheme value, like `n32` but indirected. Think of it as `SCM *x = ip + offset`.

`l32`

`lo32`

An ip-relative address, as a signed 32-bit integer. Could indicate a bytecode address, as in `make-closure`, or a non-immediate address, as with `static-patch!`.

`l32` and `lo32` are the same from the perspective of the virtual machine. The difference is that an assembler might want to allow an `lo32` address to be specified as a label and then some number of words offset from that label, for example when patching a field of a statically allocated object.

`v32:x8-l24`

Almost all VM instructions have a fixed size. The `jtable` instruction used to perform optimized `case` branches is an exception, which uses a `v32` trailing word to indicate the number of additional words in the instruction, which themselves are encoded as `x8-l24` values.

`b1`

A boolean value: 1 for true, otherwise 0.

`xn`

An ignored sequence of n bits.

An instruction is specified by giving its name, then describing its operands. The operands are packed by 32-bit words, with earlier operands occupying the lower bits.

For example, consider the following instruction specification:

Instruction: **call** `f24:proc x8:_ c24:nlocals` [¶](09_03_a_virtual_machine_for_guile.md)

The first word in the instruction will start with the 8-bit value corresponding to the call opcode in the low bits, followed by proc as a 24-bit value. The second word starts with 8 dead bits, followed by the index as a 24-bit immediate value.

For instructions with operands that encode references to the stack, the interpretation of those stack values is up to the instruction itself. Most instructions expect their operands to be tagged SCM values (`scm` representation), but some instructions expect unboxed integers (`u64` and `s64` representations) or floating-point numbers (`f64` representation). It is assumed that the bits for a `u64` value are the same as those for an `s64` value, and that `s64` values are stored in two’s complement.

Instructions have static types: they must receive their operands in the format they expect. It’s up to the compiler to ensure this is the case.

Unless otherwise mentioned, all operands and results are in the `scm` representation.

*   [Call and Return Instructions](09_03_a_virtual_machine_for_guile.md#9371-call-and-return-instructions)
*   [Function Prologue Instructions](09_03_a_virtual_machine_for_guile.md#9372-function-prologue-instructions)
*   [Shuffling Instructions](09_03_a_virtual_machine_for_guile.md#9373-shuffling-instructions)
*   [Trampoline Instructions](09_03_a_virtual_machine_for_guile.md#9374-trampoline-instructions)
*   [Non-Local Control Flow Instructions](09_03_a_virtual_machine_for_guile.md#9375-non-local-control-flow-instructions)
*   [Instrumentation Instructions](09_03_a_virtual_machine_for_guile.md#9376-instrumentation-instructions)
*   [Intrinsic Call Instructions](09_03_a_virtual_machine_for_guile.md#9377-intrinsic-call-instructions)
*   [Constant Instructions](09_03_a_virtual_machine_for_guile.md#9378-constant-instructions)
*   [Memory Access Instructions](09_03_a_virtual_machine_for_guile.md#9379-memory-access-instructions)
*   [Atomic Memory Access Instructions](09_03_a_virtual_machine_for_guile.md#93710-atomic-memory-access-instructions)
*   [Tagging and Untagging Instructions](09_03_a_virtual_machine_for_guile.md#93711-tagging-and-untagging-instructions)
*   [Integer Arithmetic Instructions](09_03_a_virtual_machine_for_guile.md#93712-integer-arithmetic-instructions)
*   [Floating-Point Arithmetic Instructions](09_03_a_virtual_machine_for_guile.md#93713-floating-point-arithmetic-instructions)
*   [Comparison Instructions](09_03_a_virtual_machine_for_guile.md#93714-comparison-instructions)
*   [Branch Instructions](09_03_a_virtual_machine_for_guile.md#93715-branch-instructions)
*   [Raw Memory Access Instructions](09_03_a_virtual_machine_for_guile.md#93716-raw-memory-access-instructions)

* * *

Next: [Function Prologue Instructions](09_03_a_virtual_machine_for_guile.md#9372-function-prologue-instructions), Up: [Instruction Set](09_03_a_virtual_machine_for_guile.md#937-instruction-set)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 9.3.7.1 Call and Return Instructions [¶](09_03_a_virtual_machine_for_guile.md#9371-call-and-return-instructions)

As described earlier (see [Stack Layout](09_03_a_virtual_machine_for_guile.md#933-stack-layout)), Guile’s calling convention is that arguments are passed and values returned on the stack.

For calls, both in tail position and in non-tail position, we require that the procedure and the arguments already be shuffled into place before the call instruction. “Into place” for a tail call means that the procedure should be in slot 0, relative to the `fp`, and the arguments should follow. For a non-tail call, if the procedure is in `fp`\-relative slot n, the arguments should follow from slot n+1, and there should be three free slots between n\-1 and n\-3 in which to save the mRA, vRA, and `fp`.

Returning values is similar. Multiple-value returns should have values already shuffled down to start from `fp`\-relative slot 0 before emitting `return-values`.

In both calls and returns, the `sp` is used to indicate to the callee or caller the number of arguments or return values, respectively. After receiving return values, it is the caller’s responsibility to _restore the frame_ by resetting the `sp` to its former value.

Instruction: **call** `f24:proc x8:_ c24:nlocals` [¶](09_03_a_virtual_machine_for_guile.md)

Call a procedure. proc is the local corresponding to a procedure. The three values below proc will be overwritten by the saved call frame data. The new frame will have space for nlocals locals: one for the procedure, and the rest for the arguments which should already have been pushed on.

When the call returns, execution proceeds with the next instruction. There may be any number of values on the return stack; the precise number can be had by subtracting the address of proc\-1 from the post-call `sp`.

Instruction: **call-label** `f24:proc x8:_ c24:nlocals l32:label` [¶](09_03_a_virtual_machine_for_guile.md)

Call a procedure in the same compilation unit.

This instruction is just like `call`, except that instead of dereferencing proc to find the call target, the call target is known to be at label, a signed 32-bit offset in 32-bit units from the current `ip`. Since proc is not dereferenced, it may be some other representation of the closure.

Instruction: **tail-call** `x24:_` [¶](09_03_a_virtual_machine_for_guile.md)

Tail-call a procedure. Requires that the procedure and all of the arguments have already been shuffled into position, and that the frame has already been reset to the number of arguments to the call.

Instruction: **tail-call-label** `x24:_ l32:label` [¶](09_03_a_virtual_machine_for_guile.md)

Tail-call a known procedure. As `call` is to `call-label`, `tail-call` is to `tail-call-label`.

Instruction: **return-values** `x24:_` [¶](09_03_a_virtual_machine_for_guile.md)

Return a number of values from a call frame. The return values should have already been shuffled down to a contiguous array starting at slot 0, and the frame already reset.

Instruction: **receive** `f12:dst f12:proc x8:_ c24:nlocals` [¶](09_03_a_virtual_machine_for_guile.md)

Receive a single return value from a call whose procedure was in proc, asserting that the call actually returned at least one value. Afterwards, resets the frame to nlocals locals.

Instruction: **receive-values** `f24:proc b1:allow-extra? x7:_ c24:nvalues` [¶](09_03_a_virtual_machine_for_guile.md)

Receive a return of multiple values from a call whose procedure was in proc. If fewer than nvalues values were returned, signal an error. Unless allow-extra? is true, require that the number of return values equals nvalues exactly. After `receive-values` has run, the values can be copied down via `mov`, or used in place.

* * *

Next: [Shuffling Instructions](09_03_a_virtual_machine_for_guile.md#9373-shuffling-instructions), Previous: [Call and Return Instructions](09_03_a_virtual_machine_for_guile.md#9371-call-and-return-instructions), Up: [Instruction Set](09_03_a_virtual_machine_for_guile.md#937-instruction-set)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 9.3.7.2 Function Prologue Instructions [¶](09_03_a_virtual_machine_for_guile.md#9372-function-prologue-instructions)

A function call in Guile is very cheap: the VM simply hands control to the procedure. The procedure itself is responsible for asserting that it has been passed an appropriate number of arguments. This strategy allows arbitrarily complex argument parsing idioms to be developed, without harming the common case.

For example, only calls to keyword-argument procedures “pay” for the cost of parsing keyword arguments. (At the time of this writing, calling procedures with keyword arguments is typically two to four times as costly as calling procedures with a fixed set of arguments.)

Instruction: **assert-nargs-ee** `c24:expected` [¶](09_03_a_virtual_machine_for_guile.md)

Instruction: **assert-nargs-ge** `c24:expected` [¶](09_03_a_virtual_machine_for_guile.md)

Instruction: **assert-nargs-le** `c24:expected` [¶](09_03_a_virtual_machine_for_guile.md)

If the number of actual arguments is not `==`, `>=`, or `<=` expected, respectively, signal an error.

The number of arguments is determined by subtracting the stack pointer from the frame pointer (`fp - sp`). See [Stack Layout](09_03_a_virtual_machine_for_guile.md#933-stack-layout), for more details on stack frames. Note that expected includes the procedure itself.

Instruction: **arguments<=?** `c24:expected` [¶](09_03_a_virtual_machine_for_guile.md)

Set the `LESS_THAN`, `EQUAL`, or `NONE` comparison result values if the number of arguments is respectively less than, equal to, or greater than expected.

Instruction: **positional-arguments<=?** `c24:nreq x8:_ c24:expected` [¶](09_03_a_virtual_machine_for_guile.md)

Set the `LESS_THAN`, `EQUAL`, or `NONE` comparison result values if the number of positional arguments is respectively less than, equal to, or greater than expected. The first nreq arguments are positional arguments, as are the subsequent arguments that are not keywords.

The `arguments<=?` and `positional-arguments<=?` instructions are used to implement multiple arities, as in `case-lambda`. See [Case-lambda](06_07_procedures.md#675-case-lambda), for more information. See [Branch Instructions](09_03_a_virtual_machine_for_guile.md#93715-branch-instructions), for more on comparison results.

Instruction: **bind-kwargs** `c24:nreq c8:flags c24:nreq-and-opt x8:_ c24:ntotal n32:kw-offset` [¶](09_03_a_virtual_machine_for_guile.md)

flags is a bitfield, whose lowest bit is allow-other-keys, second bit is has-rest, and whose following six bits are unused.

Find the last positional argument, and shuffle all the rest above ntotal. Initialize the intervening locals to `SCM_UNDEFINED`. Then load the constant at kw-offset words from the current ip, and use it and the allow-other-keys flag to bind keyword arguments. If has-rest, collect all shuffled arguments into a list, and store it in nreq-and-opt. Finally, clear the arguments that we shuffled up.

The parsing is driven by a keyword arguments association list, looked up using kw-offset. The alist is a list of pairs of the form `(kw . index)`, mapping keyword arguments to their local slot indices. Unless `allow-other-keys` is set, the parser will signal an error if an unknown key is found.

A macro-mega-instruction.

Instruction: **bind-optionals** `f24:nlocals` [¶](09_03_a_virtual_machine_for_guile.md)

Expand the current frame to have at least nlocals locals, filling in any fresh values with `SCM_UNDEFINED`. If the frame has more than nlocals locals, it is left as it is.

Instruction: **bind-rest** `f24:dst` [¶](09_03_a_virtual_machine_for_guile.md)

Collect any arguments at or above dst into a list, and store that list at dst.

Instruction: **alloc-frame** `c24:nlocals` [¶](09_03_a_virtual_machine_for_guile.md)

Ensure that there is space on the stack for nlocals local variables. The value of any new local is undefined.

Instruction: **reset-frame** `c24:nlocals` [¶](09_03_a_virtual_machine_for_guile.md)

Like `alloc-frame`, but doesn’t check that the stack is big enough, and doesn’t initialize values to `SCM_UNDEFINED`. Used to reset the frame size to something less than the size that was previously set via alloc-frame.

Instruction: **assert-nargs-ee/locals** `c12:expected c12:nlocals` [¶](09_03_a_virtual_machine_for_guile.md)

Equivalent to a sequence of `assert-nargs-ee` and `alloc-frame`. The number of locals reserved is expected + nlocals.

* * *

Next: [Trampoline Instructions](09_03_a_virtual_machine_for_guile.md#9374-trampoline-instructions), Previous: [Function Prologue Instructions](09_03_a_virtual_machine_for_guile.md#9372-function-prologue-instructions), Up: [Instruction Set](09_03_a_virtual_machine_for_guile.md#937-instruction-set)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 9.3.7.3 Shuffling Instructions [¶](09_03_a_virtual_machine_for_guile.md#9373-shuffling-instructions)

These instructions are used to move around values on the stack.

Instruction: **mov** `s12:dst s12:src` [¶](09_03_a_virtual_machine_for_guile.md)

Instruction: **long-mov** `s24:dst x8:_ s24:src` [¶](09_03_a_virtual_machine_for_guile.md)

Copy a value from one local slot to another.

As discussed previously, procedure arguments and local variables are allocated to local slots. Guile’s compiler tries to avoid shuffling variables around to different slots, which often makes `mov` instructions redundant. However there are some cases in which shuffling is necessary, and in those cases, `mov` is the thing to use.

Instruction: **long-fmov** `f24:dst x8:_ f24:src` [¶](09_03_a_virtual_machine_for_guile.md)

Copy a value from one local slot to another, but addressing slots relative to the `fp` instead of the `sp`. This is used when shuffling values into place after multiple-value returns.

Instruction: **push** `s24:src` [¶](09_03_a_virtual_machine_for_guile.md)

Bump the stack pointer by one word, and fill it with the value from slot src. The offset to src is calculated before the stack pointer is adjusted.

The `push` instruction is used when another instruction is unable to address an operand because the operand is encoded with fewer than 24 bits. In that case, Guile’s assembler will transparently emit code that temporarily pushes any needed operands onto the stack, emits the original instruction to address those now-near variables, then shuffles the result (if any) back into place.

Instruction: **pop** `s24:dst` [¶](09_03_a_virtual_machine_for_guile.md)

Pop the stack pointer, storing the value that was there in slot dst. The offset to dst is calculated after the stack pointer is adjusted.

Instruction: **drop** `c24:count` [¶](09_03_a_virtual_machine_for_guile.md)

Pop the stack pointer by count words, discarding any values that were stored there.

Instruction: **shuffle-down** `f12:from f12:to` [¶](09_03_a_virtual_machine_for_guile.md)

Shuffle down values from from to to, reducing the frame size by FROM\-TO slots. Part of the internal implementation of `call-with-values`, `values`, and `apply`.

Instruction: **expand-apply-argument** `x24:_` [¶](09_03_a_virtual_machine_for_guile.md)

Take the last local in a frame and expand it out onto the stack, as for the last argument to `apply`.

* * *

Next: [Non-Local Control Flow Instructions](09_03_a_virtual_machine_for_guile.md#9375-non-local-control-flow-instructions), Previous: [Shuffling Instructions](09_03_a_virtual_machine_for_guile.md#9373-shuffling-instructions), Up: [Instruction Set](09_03_a_virtual_machine_for_guile.md#937-instruction-set)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 9.3.7.4 Trampoline Instructions [¶](09_03_a_virtual_machine_for_guile.md#9374-trampoline-instructions)

Though most applicable objects in Guile are procedures implemented in bytecode, not all are. There are primitives, continuations, and other procedure-like objects that have their own calling convention. Instead of adding special cases to the `call` instruction, Guile wraps these other applicable objects in VM trampoline procedures, then provides special support for these objects in bytecode.

Trampoline procedures are typically generated by Guile at runtime, for example in response to a call to `scm_c_make_gsubr`. As such, a compiler probably shouldn’t emit code with these instructions. However, it’s still interesting to know how these things work, so we document these trampoline instructions here.

Instruction: **subr-call** `c24:idx` [¶](09_03_a_virtual_machine_for_guile.md)

Call a subr, passing all locals in this frame as arguments, and storing the results on the stack, ready to be returned.

Instruction: **foreign-call** `c12:cif-idx c12:ptr-idx` [¶](09_03_a_virtual_machine_for_guile.md)

Call a foreign function. Fetch the cif and foreign pointer from cif-idx and ptr-idx closure slots of the callee. Arguments are taken from the stack, and results placed on the stack, ready to be returned.

Instruction: **builtin-ref** `s12:dst c12:idx` [¶](09_03_a_virtual_machine_for_guile.md)

Load a builtin stub by index into dst.

* * *

Next: [Instrumentation Instructions](09_03_a_virtual_machine_for_guile.md#9376-instrumentation-instructions), Previous: [Trampoline Instructions](09_03_a_virtual_machine_for_guile.md#9374-trampoline-instructions), Up: [Instruction Set](09_03_a_virtual_machine_for_guile.md#937-instruction-set)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 9.3.7.5 Non-Local Control Flow Instructions [¶](09_03_a_virtual_machine_for_guile.md#9375-non-local-control-flow-instructions)

Instruction: **capture-continuation** `s24:dst` [¶](09_03_a_virtual_machine_for_guile.md)

Capture the current continuation, and write it to dst. Part of the implementation of `call/cc`.

Instruction: **continuation-call** `c24:contregs` [¶](09_03_a_virtual_machine_for_guile.md)

Return to a continuation, nonlocally. The arguments to the continuation are taken from the stack. contregs is a free variable containing the reified continuation.

Instruction: **abort** `x24:_` [¶](09_03_a_virtual_machine_for_guile.md)

Abort to a prompt handler. The tag is expected in slot 1, and the rest of the values in the frame are returned to the prompt handler. This corresponds to a tail application of `abort-to-prompt`.

If no prompt can be found in the dynamic environment with the given tag, an error is signaled. Otherwise all arguments are passed to the prompt’s handler, along with the captured continuation, if necessary.

If the prompt’s handler can be proven to not reference the captured continuation, no continuation is allocated. This decision happens dynamically, at run-time; the general case is that the continuation may be captured, and thus resumed. A reinstated continuation will have its arguments pushed on the stack from slot 0, as if from a multiple-value return, and control resumes in the caller. Thus to the calling function, a call to `abort-to-prompt` looks like any other function call.

Instruction: **compose-continuation** `c24:cont` [¶](09_03_a_virtual_machine_for_guile.md)

Compose a partial continuation with the current continuation. The arguments to the continuation are taken from the stack. cont is a free variable containing the reified continuation.

Instruction: **prompt** `s24:tag b1:escape-only? x7:_ f24:proc-slot x8:_ l24:handler-offset` [¶](09_03_a_virtual_machine_for_guile.md)

Push a new prompt on the dynamic stack, with a tag from tag and a handler at handler-offset words from the current ip.

If an abort is made to this prompt, control will jump to the handler. The handler will expect a multiple-value return as if from a call with the procedure at proc-slot, with the reified partial continuation as the first argument, followed by the values returned to the handler. If control returns to the handler, the prompt is already popped off by the abort mechanism. (Guile’s `prompt` implements Felleisen’s _–F–_ operator.)

If escape-only? is nonzero, the prompt will be marked as escape-only, which allows an abort to this prompt to avoid reifying the continuation.

See [Prompts](06_11_controlling_the_flow_of_program_execution.md#6115-prompts), for more information on prompts.

Instruction: **throw** `s12:key s12:args` [¶](09_03_a_virtual_machine_for_guile.md)

Raise an error by throwing to key and args. args should be a list.

Instruction: **throw/value** `s24:value n32:key-subr-and-message` [¶](09_03_a_virtual_machine_for_guile.md)

Instruction: **throw/value+data** `s24:value n32:key-subr-and-message` [¶](09_03_a_virtual_machine_for_guile.md)

Raise an error, indicating val as the bad value. key-subr-and-message should be a vector, where the first element is the symbol to which to throw, the second is the procedure in which to signal the error (a string) or `#f`, and the third is a format string for the message, with one template. These instructions do not fall through.

Both of these instructions throw to a key with four arguments: the procedure that indicates the error (or `#f`, the format string, a list with value, and either `#f` or the list with value as the last argument respectively.

Instruction: **unreachable** `x24:_` [¶](09_03_a_virtual_machine_for_guile.md)

Abort the process. This instruction should never be reached and must not continue. You would think this is useless but that’s not the case: it is inserted after a primcall to `raise-exception`, and allows compilers to know that this branch of control flow does not rejoin the graph.

* * *

Next: [Intrinsic Call Instructions](09_03_a_virtual_machine_for_guile.md#9377-intrinsic-call-instructions), Previous: [Non-Local Control Flow Instructions](09_03_a_virtual_machine_for_guile.md#9375-non-local-control-flow-instructions), Up: [Instruction Set](09_03_a_virtual_machine_for_guile.md#937-instruction-set)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 9.3.7.6 Instrumentation Instructions [¶](09_03_a_virtual_machine_for_guile.md#9376-instrumentation-instructions)

Instruction: **instrument-entry** `x24:_ n32:data` [¶](09_03_a_virtual_machine_for_guile.md)

Instruction: **instrument-loop** `x24:_ n32:data` [¶](09_03_a_virtual_machine_for_guile.md)

Increase execution counter for this function and potentially tier up to the next JIT level. data is an offset to a structure recording execution counts and the next-level JIT code corresponding to this function. The increment values are currently 30 for `instrument-entry` and 2 for `instrument-loop`.

`instrument-entry` will also run the apply hook, if VM hooks are enabled.

Instruction: **handle-interrupts** `x24:_` [¶](09_03_a_virtual_machine_for_guile.md)

Handle pending asynchronous interrupts (asyncs). See [Asynchronous Interrupts](06_22_threads_mutexes_asyncs_and_dynamic_roots.md#6223-asynchronous-interrupts). The compiler inserts `handle-interrupts` instructions before any call, return, or loop back-edge.

Instruction: **return-from-interrupt** `x24:_` [¶](09_03_a_virtual_machine_for_guile.md)

A special instruction to return from a call and also pop off the stack frame from the call. Used when returning from asynchronous interrupts.

* * *

Next: [Constant Instructions](09_03_a_virtual_machine_for_guile.md#9378-constant-instructions), Previous: [Instrumentation Instructions](09_03_a_virtual_machine_for_guile.md#9376-instrumentation-instructions), Up: [Instruction Set](09_03_a_virtual_machine_for_guile.md#937-instruction-set)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 9.3.7.7 Intrinsic Call Instructions [¶](09_03_a_virtual_machine_for_guile.md#9377-intrinsic-call-instructions)

Guile’s instruction set is low-level. This is good because the separate components of, say, a `vector-ref` operation might be able to be optimized out, leaving only the operations that need to be performed at run-time.

However some macro-operations may need to perform large amounts of computation at run-time to handle all the edge cases, and whose micro-operation components aren’t amenable to optimization. Residualizing code for the entire macro-operation would lead to code bloat with no benefit.

In this kind of a case, Guile’s VM calls out to _intrinsics_: run-time routines written in the host language (currently C, possibly more in the future if Guile gains more run-time targets like WebAssembly). There is one instruction for each instrinsic prototype; the intrinsic is specified by index in the instruction.

Instruction: **call-thread** `x24:_ c32:idx` [¶](09_03_a_virtual_machine_for_guile.md)

Call the `void`\-returning instrinsic with index idx, passing the current `scm_thread*` as the argument.

Instruction: **call-thread-scm** `s24:a c32:idx` [¶](09_03_a_virtual_machine_for_guile.md)

Call the `void`\-returning instrinsic with index idx, passing the current `scm_thread*` and the `scm` local a as arguments.

Instruction: **call-thread-scm-scm** `s12:a s12:b c32:idx` [¶](09_03_a_virtual_machine_for_guile.md)

Call the `void`\-returning instrinsic with index idx, passing the current `scm_thread*` and the `scm` locals a and b as arguments.

Instruction: **call-scm-sz-u32** `s12:a s12:b c32:idx` [¶](09_03_a_virtual_machine_for_guile.md)

Call the `void`\-returning instrinsic with index idx, passing the locals a, b, and c as arguments. a is a `scm` value, while b and c are raw `u64` values which fit into `size_t` and `uint32_t` types, respectively.

Instruction: **call-scm<-thread** `s24:dst c32:idx` [¶](09_03_a_virtual_machine_for_guile.md)

Call the `SCM`\-returning instrinsic with index idx, passing the current `scm_thread*` as the argument. Place the result in dst.

Instruction: **call-scm<-u64** `s12:dst s12:a c32:idx` [¶](09_03_a_virtual_machine_for_guile.md)

Call the `SCM`\-returning instrinsic with index idx, passing `u64` local a as the argument. Place the result in dst.

Instruction: **call-scm<-s64** `s12:dst s12:a c32:idx` [¶](09_03_a_virtual_machine_for_guile.md)

Call the `SCM`\-returning instrinsic with index idx, passing `s64` local a as the argument. Place the result in dst.

Instruction: **call-scm<-scm** `s12:dst s12:a c32:idx` [¶](09_03_a_virtual_machine_for_guile.md)

Call the `SCM`\-returning instrinsic with index idx, passing `scm` local a as the argument. Place the result in dst.

Instruction: **call-u64<-scm** `s12:dst s12:a c32:idx` [¶](09_03_a_virtual_machine_for_guile.md)

Call the `uint64_t`\-returning instrinsic with index idx, passing `scm` local a as the argument. Place the `u64` result in dst.

Instruction: **call-s64<-scm** `s12:dst s12:a c32:idx` [¶](09_03_a_virtual_machine_for_guile.md)

Call the `int64_t`\-returning instrinsic with index idx, passing `scm` local a as the argument. Place the `s64` result in dst.

Instruction: **call-f64<-scm** `s12:dst s12:a c32:idx` [¶](09_03_a_virtual_machine_for_guile.md)

Call the `double`\-returning instrinsic with index idx, passing `scm` local a as the argument. Place the `f64` result in dst.

Instruction: **call-scm<-scm-scm** `s8:dst s8:a s8:b c32:idx` [¶](09_03_a_virtual_machine_for_guile.md)

Call the `SCM`\-returning instrinsic with index idx, passing `scm` locals a and b as arguments. Place the `scm` result in dst.

Instruction: **call-scm<-scm-uimm** `s8:dst s8:a c8:b c32:idx` [¶](09_03_a_virtual_machine_for_guile.md)

Call the `SCM`\-returning instrinsic with index idx, passing `scm` local a and `uint8_t` immediate b as arguments. Place the `scm` result in dst.

Instruction: **call-scm<-thread-scm** `s12:dst s12:a c32:idx` [¶](09_03_a_virtual_machine_for_guile.md)

Call the `SCM`\-returning instrinsic with index idx, passing the current `scm_thread*` and `scm` local a as arguments. Place the `scm` result in dst.

Instruction: **call-scm<-scm-u64** `s8:dst s8:a s8:b c32:idx` [¶](09_03_a_virtual_machine_for_guile.md)

Call the `SCM`\-returning instrinsic with index idx, passing `scm` local a and `u64` local b as arguments. Place the `scm` result in dst.

Instruction: **call-scm-scm** `s12:a s12:b c32:idx` [¶](09_03_a_virtual_machine_for_guile.md)

Call the `void`\-returning instrinsic with index idx, passing `scm` locals a and b as arguments.

Instruction: **call-scm-scm-scm** `s8:a s8:b s8:c c32:idx` [¶](09_03_a_virtual_machine_for_guile.md)

Call the `void`\-returning instrinsic with index idx, passing `scm` locals a, b, and c as arguments.

Instruction: **call-scm-uimm-scm** `s8:a c8:b s8:c c32:idx` [¶](09_03_a_virtual_machine_for_guile.md)

Call the `void`\-returning instrinsic with index idx, passing `scm` local a, `uint8_t` immediate b, and `scm` local c as arguments.

There are corresponding macro-instructions for specific intrinsics. These are equivalent to `call-instrinsic-kind` instructions with the appropriate intrinsic idx arguments.

Macro Instruction: **add** dst a b [¶](09_03_a_virtual_machine_for_guile.md)

Macro Instruction: **add/immediate** dst a b/imm [¶](09_03_a_virtual_machine_for_guile.md)

Add `SCM` values a and b and place the result in dst.

Macro Instruction: **sub** dst a b [¶](09_03_a_virtual_machine_for_guile.md)

Macro Instruction: **sub/immediate** dst a b/imm [¶](09_03_a_virtual_machine_for_guile.md)

Subtract `SCM` value b from a and place the result in dst.

Macro Instruction: **mul** dst a b [¶](09_03_a_virtual_machine_for_guile.md)

Multiply `SCM` values a and b and place the result in dst.

Macro Instruction: **div** dst a b [¶](09_03_a_virtual_machine_for_guile.md)

Divide `SCM` value a by b and place the result in dst.

Macro Instruction: **quo** dst a b [¶](09_03_a_virtual_machine_for_guile.md)

Compute the quotient of `SCM` values a and b and place the result in dst.

Macro Instruction: **rem** dst a b [¶](09_03_a_virtual_machine_for_guile.md)

Compute the remainder of `SCM` values a and b and place the result in dst.

Macro Instruction: **mod** dst a b [¶](09_03_a_virtual_machine_for_guile.md)

Compute the modulo of `SCM` value a by b and place the result in dst.

Macro Instruction: **logand** dst a b [¶](09_03_a_virtual_machine_for_guile.md)

Compute the bitwise `and` of `SCM` values a and b and place the result in dst.

Macro Instruction: **logior** dst a b [¶](09_03_a_virtual_machine_for_guile.md)

Compute the bitwise inclusive `or` of `SCM` values a and b and place the result in dst.

Macro Instruction: **logxor** dst a b [¶](09_03_a_virtual_machine_for_guile.md)

Compute the bitwise exclusive `or` of `SCM` values a and b and place the result in dst.

Macro Instruction: **logsub** dst a b [¶](09_03_a_virtual_machine_for_guile.md)

Compute the bitwise `and` of `SCM` value a and the bitwise `not` of b and place the result in dst.

Macro Instruction: **lsh** dst a b [¶](09_03_a_virtual_machine_for_guile.md)

Macro Instruction: **lsh/immediate** a b/imm [¶](09_03_a_virtual_machine_for_guile.md)

Shift `SCM` value a left by `u64` value b bits and place the result in dst.

Macro Instruction: **rsh** dst a b [¶](09_03_a_virtual_machine_for_guile.md)

Macro Instruction: **rsh/immediate** dst a b/imm [¶](09_03_a_virtual_machine_for_guile.md)

Shifts `SCM` value a right by `u64` value b bits and place the result in dst.

Macro Instruction: **scm->f64** dst src [¶](09_03_a_virtual_machine_for_guile.md)

Convert src to an unboxed `f64` and place the result in dst, or raises an error if src is not a real number.

Macro Instruction: **scm->u64** dst src [¶](09_03_a_virtual_machine_for_guile.md)

Convert src to an unboxed `u64` and place the result in dst, or raises an error if src is not an integer within range.

Macro Instruction: **scm->u64/truncate** dst src [¶](09_03_a_virtual_machine_for_guile.md)

Convert src to an unboxed `u64` and place the result in dst, truncating to the low 64 bits, or raises an error if src is not an integer.

Macro Instruction: **scm->s64** dst src [¶](09_03_a_virtual_machine_for_guile.md)

Convert src to an unboxed `s64` and place the result in dst, or raises an error if src is not an integer within range.

Macro Instruction: **u64->scm** dst src [¶](09_03_a_virtual_machine_for_guile.md)

Convert u64 value src to a Scheme integer in dst.

Macro Instruction: **s64->scm** scm<-s64 [¶](09_03_a_virtual_machine_for_guile.md)

Convert s64 value src to a Scheme integer in dst.

Macro Instruction: **string-set!** str idx ch [¶](09_03_a_virtual_machine_for_guile.md)

Sets the character idx (a `u64`) of string str to ch (a `u64` that is a valid character value).

Macro Instruction: **string->number** dst src [¶](09_03_a_virtual_machine_for_guile.md)

Call `string->number` on src and place the result in dst.

Macro Instruction: **string->symbol** dst src [¶](09_03_a_virtual_machine_for_guile.md)

Call `string->symbol` on src and place the result in dst.

Macro Instruction: **symbol->keyword** dst src [¶](09_03_a_virtual_machine_for_guile.md)

Call `symbol->keyword` on src and place the result in dst.

Macro Instruction: **class-of** dst src [¶](09_03_a_virtual_machine_for_guile.md)

Set dst to the GOOPS class of `src`.

Macro Instruction: **wind** winder unwinder [¶](09_03_a_virtual_machine_for_guile.md)

Push wind and unwind procedures onto the dynamic stack. Note that neither are actually called; the compiler should emit calls to winder and unwinder for the normal dynamic-wind control flow. Also note that the compiler should have inserted checks that winder and unwinder are thunks, if it could not prove that to be the case. See [Dynamic Wind](06_11_controlling_the_flow_of_program_execution.md#61110-dynamic-wind).

Macro Instruction: **unwind** [¶](09_03_a_virtual_machine_for_guile.md)

Exit from the dynamic extent of an expression, popping the top entry off of the dynamic stack.

Macro Instruction: **push-fluid** fluid value [¶](09_03_a_virtual_machine_for_guile.md)

Dynamically bind value to fluid by creating a with-fluids object, pushing that object on the dynamic stack. See [Fluids and Dynamic States](06_11_controlling_the_flow_of_program_execution.md#61111-fluids-and-dynamic-states).

Macro Instruction: **pop-fluid** [¶](09_03_a_virtual_machine_for_guile.md)

Leave the dynamic extent of a `with-fluid*` expression, restoring the fluid to its previous value. `push-fluid` should always be balanced with `pop-fluid`.

Macro Instruction: **fluid-ref** dst fluid [¶](09_03_a_virtual_machine_for_guile.md)

Place the value associated with the fluid fluid in dst.

Macro Instruction: **fluid-set!** fluid value [¶](09_03_a_virtual_machine_for_guile.md)

Set the value of the fluid fluid to value.

Macro Instruction: **push-dynamic-state** state [¶](09_03_a_virtual_machine_for_guile.md)

Save the current set of fluid bindings on the dynamic stack and instate the bindings from state instead. See [Fluids and Dynamic States](06_11_controlling_the_flow_of_program_execution.md#61111-fluids-and-dynamic-states).

Macro Instruction: **pop-dynamic-state** [¶](09_03_a_virtual_machine_for_guile.md)

Restore a saved set of fluid bindings from the dynamic stack. `push-dynamic-state` should always be balanced with `pop-dynamic-state`.

Macro Instruction: **resolve-module** dst name public? [¶](09_03_a_virtual_machine_for_guile.md)

Look up the module named name, resolve its public interface if the immediate operand public? is true, then place the result in dst.

Macro Instruction: **lookup** dst mod sym [¶](09_03_a_virtual_machine_for_guile.md)

Look up sym in module mod, placing the resulting variable (or `#f` if not found) in dst.

Macro Instruction: **define!** dst mod sym [¶](09_03_a_virtual_machine_for_guile.md)

Look up sym in module mod, placing the resulting variable in dst, creating the variable if needed.

Macro Instruction: **current-module** dst [¶](09_03_a_virtual_machine_for_guile.md)

Set dst to the current module.

Macro Instruction: **$car** dst src [¶](09_03_a_virtual_machine_for_guile.md)

Macro Instruction: **$cdr** dst src [¶](09_03_a_virtual_machine_for_guile.md)

Macro Instruction: **$set-car!** x val [¶](09_03_a_virtual_machine_for_guile.md)

Macro Instruction: **$set-cdr!** x val [¶](09_03_a_virtual_machine_for_guile.md)

Macro Instruction: **$variable-ref** dst src [¶](09_03_a_virtual_machine_for_guile.md)

Macro Instruction: **$variable-set!** x val [¶](09_03_a_virtual_machine_for_guile.md)

Macro Instruction: **$vector-length** dst x [¶](09_03_a_virtual_machine_for_guile.md)

Macro Instruction: **$vector-ref** dst x idx [¶](09_03_a_virtual_machine_for_guile.md)

Macro Instruction: **$vector-ref/immediate** dst x idx/imm [¶](09_03_a_virtual_machine_for_guile.md)

Macro Instruction: **$vector-set!** x idx v [¶](09_03_a_virtual_machine_for_guile.md)

Macro Instruction: **$vector-set!/immediate** x idx/imm v [¶](09_03_a_virtual_machine_for_guile.md)

Macro Instruction: **$allocate-struct** dst vtable nwords [¶](09_03_a_virtual_machine_for_guile.md)

Macro Instruction: **$struct-vtable** dst src [¶](09_03_a_virtual_machine_for_guile.md)

Macro Instruction: **$struct-ref** dst src idx [¶](09_03_a_virtual_machine_for_guile.md)

Macro Instruction: **$struct-ref/immediate** dst src idx/imm [¶](09_03_a_virtual_machine_for_guile.md)

Macro Instruction: **$struct-set!** x idx v [¶](09_03_a_virtual_machine_for_guile.md)

Macro Instruction: **$struct-set!/immediate** x idx/imm v [¶](09_03_a_virtual_machine_for_guile.md)

Intrinsics for use by the baseline compiler. The usual strategy for CPS compilation is to expose the component parts of e.g. `vector-ref` so that the compiler can learn from them and eliminate needless bits. However in the non-optimizing baseline compiler, that’s just overhead, so we have some intrinsics that encapsulate all the usual type checks.

* * *

Next: [Memory Access Instructions](09_03_a_virtual_machine_for_guile.md#9379-memory-access-instructions), Previous: [Intrinsic Call Instructions](09_03_a_virtual_machine_for_guile.md#9377-intrinsic-call-instructions), Up: [Instruction Set](09_03_a_virtual_machine_for_guile.md#937-instruction-set)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 9.3.7.8 Constant Instructions [¶](09_03_a_virtual_machine_for_guile.md#9378-constant-instructions)

The following instructions load literal data into a program. There are two kinds.

The first set of instructions loads immediate values. These instructions encode the immediate directly into the instruction stream.

Instruction: **make-immediate** `s8:dst zi16:low-bits` [¶](09_03_a_virtual_machine_for_guile.md)

Make an immediate whose low bits are low-bits, sign-extended.

Instruction: **make-short-immediate** `s8:dst i16:low-bits` [¶](09_03_a_virtual_machine_for_guile.md)

Make an immediate whose low bits are low-bits, and whose top bits are 0.

Instruction: **make-long-immediate** `s24:dst i32:low-bits` [¶](09_03_a_virtual_machine_for_guile.md)

Make an immediate whose low bits are low-bits, and whose top bits are 0.

Instruction: **make-long-long-immediate** `s24:dst a32:high-bits b32:low-bits` [¶](09_03_a_virtual_machine_for_guile.md)

Make an immediate with high-bits and low-bits.

Non-immediate constant literals are referenced either directly or indirectly. For example, Guile knows at compile-time what the layout of a string will be like, and arranges to embed that object directly in the compiled image. A reference to a string will use `make-non-immediate` to treat a pointer into the compilation unit as a `scm` value directly.

Instruction: **make-non-immediate** `s24:dst n32:offset` [¶](09_03_a_virtual_machine_for_guile.md)

Load a pointer to statically allocated memory into dst. The object’s memory will be found offset 32-bit words away from the current instruction pointer. Whether the object is mutable or immutable depends on where it was allocated by the compiler, and loaded by the loader.

Sometimes you need to load up a code pointer into a register; for this, use `load-label`.

Instruction: **load-label** `s24:dst l32:offset` [¶](09_03_a_virtual_machine_for_guile.md)

Load a label offset words away from the current `ip` and write it to dst. offset is a signed 32-bit integer.

Finally, Guile supports a number of unboxed data types, with their associate constant loaders.

Instruction: **load-f64** `s24:dst au32:high-bits au32:low-bits` [¶](09_03_a_virtual_machine_for_guile.md)

Load a double-precision floating-point value formed by joining high-bits and low-bits, and write it to dst.

Instruction: **load-u64** `s24:dst au32:high-bits au32:low-bits` [¶](09_03_a_virtual_machine_for_guile.md)

Load an unsigned 64-bit integer formed by joining high-bits and low-bits, and write it to dst.

Instruction: **load-s64** `s24:dst au32:high-bits au32:low-bits` [¶](09_03_a_virtual_machine_for_guile.md)

Load a signed 64-bit integer formed by joining high-bits and low-bits, and write it to dst.

Some objects must be unique across the whole system. This is the case for symbols and keywords. For these objects, Guile arranges to initialize them when the compilation unit is loaded, storing them into a slot in the image. References go indirectly through that slot. `static-ref` is used in this case.

Instruction: **static-ref** `s24:dst r32:offset` [¶](09_03_a_virtual_machine_for_guile.md)

Load a scm value into dst. The scm value will be fetched from memory, offset 32-bit words away from the current instruction pointer. offset is a signed value.

Fields of non-immediates may need to be fixed up at load time, because we do not know in advance at what address they will be loaded. This is the case, for example, for a pair containing a non-immediate in one of its fields. `static-set!` and `static-patch!` are used in these situations.

Instruction: **static-set!** `s24:src lo32:offset` [¶](09_03_a_virtual_machine_for_guile.md)

Store a scm value into memory, offset 32-bit words away from the current instruction pointer. offset is a signed value.

Instruction: **static-patch!** `x24:_ lo32:dst-offset l32:src-offset` [¶](09_03_a_virtual_machine_for_guile.md)

Patch a pointer at dst-offset to point to src-offset. Both offsets are signed 32-bit values, indicating a memory address as a number of 32-bit words away from the current instruction pointer.

* * *

Next: [Atomic Memory Access Instructions](09_03_a_virtual_machine_for_guile.md#93710-atomic-memory-access-instructions), Previous: [Constant Instructions](09_03_a_virtual_machine_for_guile.md#9378-constant-instructions), Up: [Instruction Set](09_03_a_virtual_machine_for_guile.md#937-instruction-set)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 9.3.7.9 Memory Access Instructions [¶](09_03_a_virtual_machine_for_guile.md#9379-memory-access-instructions)

In these instructions, the `/immediate` variants represent their indexes or counts as immediates; otherwise these values are unboxed u64 locals.

Instruction: **allocate-words** `s12:dst s12:count` [¶](09_03_a_virtual_machine_for_guile.md)

Instruction: **allocate-words/immediate** `s12:dst c12:count` [¶](09_03_a_virtual_machine_for_guile.md)

Allocate a fresh GC-traced object consisting of count words and store it into dst.

Instruction: **scm-ref** `s8:dst s8:obj s8:idx` [¶](09_03_a_virtual_machine_for_guile.md)

Instruction: **scm-ref/immediate** `s8:dst s8:obj c8:idx` [¶](09_03_a_virtual_machine_for_guile.md)

Load the `SCM` object at word offset idx from local obj, and store it to dst.

Instruction: **scm-set!** `s8:dst s8:idx s8:obj` [¶](09_03_a_virtual_machine_for_guile.md)

Instruction: **scm-set!/immediate** `s8:dst c8:idx s8:obj` [¶](09_03_a_virtual_machine_for_guile.md)

Store the `scm` local val into object obj at word offset idx.

Instruction: **scm-ref/tag** `s8:dst s8:obj c8:tag` [¶](09_03_a_virtual_machine_for_guile.md)

Load the first word of obj, subtract the immediate tag, and store the resulting `SCM` to dst.

Instruction: **scm-set!/tag** `s8:obj c8:tag s8:val` [¶](09_03_a_virtual_machine_for_guile.md)

Set the first word of obj to the unpacked bits of the `scm` value val plus the immediate value tag.

Instruction: **word-ref** `s8:dst s8:obj s8:idx` [¶](09_03_a_virtual_machine_for_guile.md)

Instruction: **word-ref/immediate** `s8:dst s8:obj c8:idx` [¶](09_03_a_virtual_machine_for_guile.md)

Load the word at offset idx from local obj, and store it to the `u64` local dst.

Instruction: **word-set!** `s8:dst s8:idx s8:obj` [¶](09_03_a_virtual_machine_for_guile.md)

Instruction: **word-set!/immediate** `s8:dst c8:idx s8:obj` [¶](09_03_a_virtual_machine_for_guile.md)

Store the `u64` local val into object obj at word offset idx.

Instruction: **pointer-ref/immediate** `s8:dst s8:obj c8:idx` [¶](09_03_a_virtual_machine_for_guile.md)

Load the pointer at offset idx from local obj, and store it to the unboxed pointer local dst.

Instruction: **pointer-set!/immediate** `s8:dst c8:idx s8:obj` [¶](09_03_a_virtual_machine_for_guile.md)

Store the unboxed pointer local val into object obj at word offset idx.

Instruction: **tail-pointer-ref/immediate** `s8:dst s8:obj c8:idx` [¶](09_03_a_virtual_machine_for_guile.md)

Compute the address of word offset idx from local obj, and store it to dst.

* * *

Next: [Tagging and Untagging Instructions](09_03_a_virtual_machine_for_guile.md#93711-tagging-and-untagging-instructions), Previous: [Memory Access Instructions](09_03_a_virtual_machine_for_guile.md#9379-memory-access-instructions), Up: [Instruction Set](09_03_a_virtual_machine_for_guile.md#937-instruction-set)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 9.3.7.10 Atomic Memory Access Instructions [¶](09_03_a_virtual_machine_for_guile.md#93710-atomic-memory-access-instructions)

Instruction: **current-thread** `s24:dst` [¶](09_03_a_virtual_machine_for_guile.md)

Write the current thread into dst.

Instruction: **atomic-scm-ref/immediate** `s8:dst s8:obj c8:idx` [¶](09_03_a_virtual_machine_for_guile.md)

Atomically load the `SCM` object at word offset idx from local obj, using the sequential consistency memory model. Store the result to dst.

Instruction: **atomic-scm-set!/immediate** `s8:obj c8:idx s8:val` [¶](09_03_a_virtual_machine_for_guile.md)

Atomically set the `SCM` object at word offset idx from local obj to val, using the sequential consistency memory model.

Instruction: **atomic-scm-swap!/immediate** `s24:dst x8:_ s24:obj c8:idx s24:val` [¶](09_03_a_virtual_machine_for_guile.md)

Atomically swap the `SCM` value stored in object obj at word offset idx with val, using the sequentially consistent memory model. Store the previous value to dst.

Instruction: **atomic-scm-compare-and-swap!/immediate** `s24:dst x8:_ s24:obj c8:idx s24:expected x8:_ s24:desired` [¶](09_03_a_virtual_machine_for_guile.md)

Atomically swap the `SCM` value stored in object obj at word offset idx with desired, if and only if the value that was there was expected, using the sequentially consistent memory model. Store the value that was previously at idx from obj in dst.

* * *

Next: [Integer Arithmetic Instructions](09_03_a_virtual_machine_for_guile.md#93712-integer-arithmetic-instructions), Previous: [Atomic Memory Access Instructions](09_03_a_virtual_machine_for_guile.md#93710-atomic-memory-access-instructions), Up: [Instruction Set](09_03_a_virtual_machine_for_guile.md#937-instruction-set)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 9.3.7.11 Tagging and Untagging Instructions [¶](09_03_a_virtual_machine_for_guile.md#93711-tagging-and-untagging-instructions)

Instruction: **tag-char** `s12:dst s12:src` [¶](09_03_a_virtual_machine_for_guile.md)

Make a `SCM` character whose integer value is the `u64` in src, and store it in dst.

Instruction: **untag-char** `s12:dst s12:src` [¶](09_03_a_virtual_machine_for_guile.md)

Extract the integer value from the `SCM` character src, and store the resulting `u64` in dst.

Instruction: **tag-fixnum** `s12:dst s12:src` [¶](09_03_a_virtual_machine_for_guile.md)

Make a `SCM` integer whose value is the `s64` in src, and store it in dst.

Instruction: **untag-fixnum** `s12:dst s12:src` [¶](09_03_a_virtual_machine_for_guile.md)

Extract the integer value from the `SCM` integer src, and store the resulting `s64` in dst.

* * *

Next: [Floating-Point Arithmetic Instructions](09_03_a_virtual_machine_for_guile.md#93713-floating-point-arithmetic-instructions), Previous: [Tagging and Untagging Instructions](09_03_a_virtual_machine_for_guile.md#93711-tagging-and-untagging-instructions), Up: [Instruction Set](09_03_a_virtual_machine_for_guile.md#937-instruction-set)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 9.3.7.12 Integer Arithmetic Instructions [¶](09_03_a_virtual_machine_for_guile.md#93712-integer-arithmetic-instructions)

Instruction: **uadd** `s8:dst s8:a s8:b` [¶](09_03_a_virtual_machine_for_guile.md)

Instruction: **uadd/immediate** `s8:dst s8:a c8:b` [¶](09_03_a_virtual_machine_for_guile.md)

Add the `u64` values a and b, and store the `u64` result to dst. Overflow will wrap.

Instruction: **usub** `s8:dst s8:a s8:b` [¶](09_03_a_virtual_machine_for_guile.md)

Instruction: **usub/immediate** `s8:dst s8:a c8:b` [¶](09_03_a_virtual_machine_for_guile.md)

Subtract the `u64` value b from a, and store the `u64` result to dst. Underflow will wrap.

Instruction: **umul** `s8:dst s8:a s8:b` [¶](09_03_a_virtual_machine_for_guile.md)

Instruction: **umul/immediate** `s8:dst s8:a c8:b` [¶](09_03_a_virtual_machine_for_guile.md)

Multiply the `u64` values a and b, and store the `u64` result to dst. Overflow will wrap.

Instruction: **ulogand** `s8:dst s8:a s8:b` [¶](09_03_a_virtual_machine_for_guile.md)

Place the bitwise `and` of the `u64` values a and b into the `u64` local dst.

Instruction: **ulogior** `s8:dst s8:a s8:b` [¶](09_03_a_virtual_machine_for_guile.md)

Place the bitwise inclusive `or` of the `u64` values a and b into the `u64` local dst.

Instruction: **ulogxor** `s8:dst s8:a s8:b` [¶](09_03_a_virtual_machine_for_guile.md)

Place the bitwise exclusive `or` of the `u64` values a and b into the `u64` local dst.

Instruction: **ulogsub** `s8:dst s8:a s8:b` [¶](09_03_a_virtual_machine_for_guile.md)

Place the bitwise `and` of the `u64` values a and the bitwise `not` of b into the `u64` local dst.

Instruction: **ulsh** `s8:dst s8:a s8:b` [¶](09_03_a_virtual_machine_for_guile.md)

Instruction: **ulsh/immediate** `s8:dst s8:a c8:b` [¶](09_03_a_virtual_machine_for_guile.md)

Shift the unboxed unsigned 64-bit integer in a left by b bits, also an unboxed unsigned 64-bit integer. Truncate to 64 bits and write to dst as an unboxed value. Only the lower 6 bits of b are used.

Instruction: **ursh** `s8:dst s8:a s8:b` [¶](09_03_a_virtual_machine_for_guile.md)

Instruction: **ursh/immediate** `s8:dst s8:a c8:b` [¶](09_03_a_virtual_machine_for_guile.md)

Shift the unboxed unsigned 64-bit integer in a right by b bits, also an unboxed unsigned 64-bit integer. Truncate to 64 bits and write to dst as an unboxed value. Only the lower 6 bits of b are used.

Instruction: **srsh** `s8:dst s8:a s8:b` [¶](09_03_a_virtual_machine_for_guile.md)

Instruction: **srsh/immediate** `s8:dst s8:a c8:b` [¶](09_03_a_virtual_machine_for_guile.md)

Shift the unboxed signed 64-bit integer in a right by b bits, also an unboxed signed 64-bit integer. Truncate to 64 bits and write to dst as an unboxed value. Only the lower 6 bits of b are used.

* * *

Next: [Comparison Instructions](09_03_a_virtual_machine_for_guile.md#93714-comparison-instructions), Previous: [Integer Arithmetic Instructions](09_03_a_virtual_machine_for_guile.md#93712-integer-arithmetic-instructions), Up: [Instruction Set](09_03_a_virtual_machine_for_guile.md#937-instruction-set)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 9.3.7.13 Floating-Point Arithmetic Instructions [¶](09_03_a_virtual_machine_for_guile.md#93713-floating-point-arithmetic-instructions)

Instruction: **fadd** `s8:dst s8:a s8:b` [¶](09_03_a_virtual_machine_for_guile.md)

Add the `f64` values a and b, and store the `f64` result to dst.

Instruction: **fsub** `s8:dst s8:a s8:b` [¶](09_03_a_virtual_machine_for_guile.md)

Subtract the `f64` value b from a, and store the `f64` result to dst.

Instruction: **fmul** `s8:dst s8:a s8:b` [¶](09_03_a_virtual_machine_for_guile.md)

Multiply the `f64` values a and b, and store the `f64` result to dst.

Instruction: **fdiv** `s8:dst s8:a s8:b` [¶](09_03_a_virtual_machine_for_guile.md)

Divide the `f64` values a by b, and store the `f64` result to dst.

* * *

Next: [Branch Instructions](09_03_a_virtual_machine_for_guile.md#93715-branch-instructions), Previous: [Floating-Point Arithmetic Instructions](09_03_a_virtual_machine_for_guile.md#93713-floating-point-arithmetic-instructions), Up: [Instruction Set](09_03_a_virtual_machine_for_guile.md#937-instruction-set)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 9.3.7.14 Comparison Instructions [¶](09_03_a_virtual_machine_for_guile.md#93714-comparison-instructions)

Comparison instructions set the comparison result, which may then be utilized by the [Branch Instructions](09_03_a_virtual_machine_for_guile.md#93715-branch-instructions).

Instruction: **u64=?** `s12:a s12:b` [¶](09_03_a_virtual_machine_for_guile.md)

Set the comparison result to EQUAL if the `u64` values a and b are the same, or `NONE` otherwise.

Instruction: **u64<?** `s12:a s12:b` [¶](09_03_a_virtual_machine_for_guile.md)

Set the comparison result to `LESS_THAN` if the `u64` value a is less than the `u64` value b are the same, or `NONE` otherwise.

Instruction: **s64<?** `s12:a s12:b` [¶](09_03_a_virtual_machine_for_guile.md)

Set the comparison result to `LESS_THAN` if the `s64` value a is less than the `s64` value b are the same, or `NONE` otherwise.

Instruction: **s64-imm=?** `s12:a z12:b` [¶](09_03_a_virtual_machine_for_guile.md)

Set the comparison result to EQUAL if the `s64` value a is equal to the immediate `s64` value b, or `NONE` otherwise.

Instruction: **u64-imm<?** `s12:a c12:b` [¶](09_03_a_virtual_machine_for_guile.md)

Set the comparison result to `LESS_THAN` if the `u64` value a is less than the immediate `u64` value b, or `NONE` otherwise.

Instruction: **imm-u64<?** `s12:a s12:b` [¶](09_03_a_virtual_machine_for_guile.md)

Set the comparison result to `LESS_THAN` if the `u64` immediate b is less than the `u64` value a, or `NONE` otherwise.

Instruction: **s64-imm<?** `s12:a z12:b` [¶](09_03_a_virtual_machine_for_guile.md)

Set the comparison result to `LESS_THAN` if the `s64` value a is less than the immediate `s64` value b, or `NONE` otherwise.

Instruction: **imm-s64<?** `s12:a z12:b` [¶](09_03_a_virtual_machine_for_guile.md)

Set the comparison result to `LESS_THAN` if the `s64` immediate b is less than the `s64` value a, or `NONE` otherwise.

Instruction: **f64=?** `s12:a s12:b` [¶](09_03_a_virtual_machine_for_guile.md)

Set the comparison result to EQUAL if the f64 value a is equal to the f64 value b, or `NONE` otherwise.

Instruction: **f64<?** `s12:a s12:b` [¶](09_03_a_virtual_machine_for_guile.md)

Set the comparison result to `LESS_THAN` if the f64 value a is less than the f64 value b, `NONE` if a is greater than or equal to b, or `INVALID` otherwise.

Instruction: **\=?** `s12:a s12:b` [¶](09_03_a_virtual_machine_for_guile.md)

Set the comparison result to EQUAL if the SCM values a and b are numerically equal, in the sense of the Scheme `=` operator. Set to `NONE` otherwise.

Instruction: **heap-numbers-equal?** `s12:a s12:b` [¶](09_03_a_virtual_machine_for_guile.md)

Set the comparison result to EQUAL if the SCM values a and b are numerically equal, in the sense of Scheme `=`. Set to `NONE` otherwise. It is known that both a and b are heap numbers.

Instruction: **<?** `s12:a s12:b` [¶](09_03_a_virtual_machine_for_guile.md)

Set the comparison result to `LESS_THAN` if the SCM value a is less than the SCM value b, `NONE` if a is greater than or equal to b, or `INVALID` otherwise.

Instruction: **immediate-tag=?** `s24:obj c16:mask c16:tag` [¶](09_03_a_virtual_machine_for_guile.md)

Set the comparison result to EQUAL if the result of a bitwise `and` between the bits of `scm` value obj and the immediate mask is tag, or `NONE` otherwise.

Instruction: **heap-tag=?** `s24:obj c16:mask c16:tag` [¶](09_03_a_virtual_machine_for_guile.md)

Set the comparison result to EQUAL if the result of a bitwise `and` between the first word of `scm` value obj and the immediate mask is tag, or `NONE` otherwise.

Instruction: **eq?** `s12:a s12:b` [¶](09_03_a_virtual_machine_for_guile.md)

Set the comparison result to EQUAL if the SCM values a and b are `eq?`, or `NONE` otherwise.

Instruction: **eq-immediate?** `s8:a zi16:b` [¶](09_03_a_virtual_machine_for_guile.md)

Set the comparison result to EQUAL if the SCM value a is equal to the immediate SCM value b (sign-extended), or `NONE` otherwise.

There are a set of macro-instructions for `immediate-tag=?` and `heap-tag=?` as well that abstract away the precise type tag values. See [The SCM Type in Guile](09_02_data_representation.md#925-the-scm-type-in-guile).

Macro Instruction: **fixnum?** x [¶](09_03_a_virtual_machine_for_guile.md)

Macro Instruction: **heap-object?** x [¶](09_03_a_virtual_machine_for_guile.md)

Macro Instruction: **char?** x [¶](09_03_a_virtual_machine_for_guile.md)

Macro Instruction: **eq-false?** x [¶](09_03_a_virtual_machine_for_guile.md)

Macro Instruction: **eq-nil?** x [¶](09_03_a_virtual_machine_for_guile.md)

Macro Instruction: **eq-null?** x [¶](09_03_a_virtual_machine_for_guile.md)

Macro Instruction: **eq-true?** x [¶](09_03_a_virtual_machine_for_guile.md)

Macro Instruction: **unspecified?** x [¶](09_03_a_virtual_machine_for_guile.md)

Macro Instruction: **undefined?** x [¶](09_03_a_virtual_machine_for_guile.md)

Macro Instruction: **eof-object?** x [¶](09_03_a_virtual_machine_for_guile.md)

Macro Instruction: **null?** x [¶](09_03_a_virtual_machine_for_guile.md)

Macro Instruction: **false?** x [¶](09_03_a_virtual_machine_for_guile.md)

Macro Instruction: **nil?** x [¶](09_03_a_virtual_machine_for_guile.md)

Emit a `immediate-tag=?` instruction that will set the comparison result to `EQUAL` if x would pass the corresponding predicate (e.g. `null?`), or `NONE` otherwise.

Macro Instruction: **pair?** x [¶](09_03_a_virtual_machine_for_guile.md)

Macro Instruction: **struct?** x [¶](09_03_a_virtual_machine_for_guile.md)

Macro Instruction: **symbol?** x [¶](09_03_a_virtual_machine_for_guile.md)

Macro Instruction: **variable?** x [¶](09_03_a_virtual_machine_for_guile.md)

Macro Instruction: **vector?** x [¶](09_03_a_virtual_machine_for_guile.md)

Macro Instruction: **immutable-vector?** x [¶](09_03_a_virtual_machine_for_guile.md)

Macro Instruction: **mutable-vector?** x [¶](09_03_a_virtual_machine_for_guile.md)

Macro Instruction: **weak-vector?** x [¶](09_03_a_virtual_machine_for_guile.md)

Macro Instruction: **string?** x [¶](09_03_a_virtual_machine_for_guile.md)

Macro Instruction: **heap-number?** x [¶](09_03_a_virtual_machine_for_guile.md)

Macro Instruction: **hash-table?** x [¶](09_03_a_virtual_machine_for_guile.md)

Macro Instruction: **pointer?** x [¶](09_03_a_virtual_machine_for_guile.md)

Macro Instruction: **fluid?** x [¶](09_03_a_virtual_machine_for_guile.md)

Macro Instruction: **stringbuf?** x [¶](09_03_a_virtual_machine_for_guile.md)

Macro Instruction: **dynamic-state?** x [¶](09_03_a_virtual_machine_for_guile.md)

Macro Instruction: **frame?** x [¶](09_03_a_virtual_machine_for_guile.md)

Macro Instruction: **keyword?** x [¶](09_03_a_virtual_machine_for_guile.md)

Macro Instruction: **atomic-box?** x [¶](09_03_a_virtual_machine_for_guile.md)

Macro Instruction: **syntax?** x [¶](09_03_a_virtual_machine_for_guile.md)

Macro Instruction: **program?** x [¶](09_03_a_virtual_machine_for_guile.md)

Macro Instruction: **vm-continuation?** x [¶](09_03_a_virtual_machine_for_guile.md)

Macro Instruction: **bytevector?** x [¶](09_03_a_virtual_machine_for_guile.md)

Macro Instruction: **weak-set?** x [¶](09_03_a_virtual_machine_for_guile.md)

Macro Instruction: **weak-table?** x [¶](09_03_a_virtual_machine_for_guile.md)

Macro Instruction: **array?** x [¶](09_03_a_virtual_machine_for_guile.md)

Macro Instruction: **bitvector?** x [¶](09_03_a_virtual_machine_for_guile.md)

Macro Instruction: **smob?** x [¶](09_03_a_virtual_machine_for_guile.md)

Macro Instruction: **port?** x [¶](09_03_a_virtual_machine_for_guile.md)

Macro Instruction: **bignum?** x [¶](09_03_a_virtual_machine_for_guile.md)

Macro Instruction: **flonum?** x [¶](09_03_a_virtual_machine_for_guile.md)

Macro Instruction: **compnum?** x [¶](09_03_a_virtual_machine_for_guile.md)

Macro Instruction: **fracnum?** x [¶](09_03_a_virtual_machine_for_guile.md)

Emit a `heap-tag=?` instruction that will set the comparison result to `EQUAL` if x would pass the corresponding predicate (e.g. `null?`), or `NONE` otherwise.

* * *

Next: [Raw Memory Access Instructions](09_03_a_virtual_machine_for_guile.md#93716-raw-memory-access-instructions), Previous: [Comparison Instructions](09_03_a_virtual_machine_for_guile.md#93714-comparison-instructions), Up: [Instruction Set](09_03_a_virtual_machine_for_guile.md#937-instruction-set)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 9.3.7.15 Branch Instructions [¶](09_03_a_virtual_machine_for_guile.md#93715-branch-instructions)

All offsets to branch instructions are 24-bit signed numbers, which count 32-bit units. This gives Guile effectively a 26-bit address range for relative jumps.

Instruction: **j** `l24:offset` [¶](09_03_a_virtual_machine_for_guile.md)

Add offset to the current instruction pointer.

Instruction: **jl** `l24:offset` [¶](09_03_a_virtual_machine_for_guile.md)

If the last comparison result is `LESS_THAN`, add offset, a signed 24-bit number, to the current instruction pointer.

Instruction: **je** `l24:offset` [¶](09_03_a_virtual_machine_for_guile.md)

If the last comparison result is `EQUAL`, add offset, a signed 24-bit number, to the current instruction pointer.

Instruction: **jnl** `l24:offset` [¶](09_03_a_virtual_machine_for_guile.md)

If the last comparison result is not `LESS_THAN`, add offset, a signed 24-bit number, to the current instruction pointer.

Instruction: **jne** `l24:offset` [¶](09_03_a_virtual_machine_for_guile.md)

If the last comparison result is not `EQUAL`, add offset, a signed 24-bit number, to the current instruction pointer.

Instruction: **jge** `l24:offset` [¶](09_03_a_virtual_machine_for_guile.md)

If the last comparison result is `NONE`, add offset, a signed 24-bit number, to the current instruction pointer.

This is intended for use after a `<?` comparison, and is different from `jnl` in the way it handles not-a-number (NaN) values: `<?` sets `INVALID` instead of `NONE` if either value is a NaN. For exact numbers, `jge` is the same as `jnl`.

Instruction: **jnge** `l24:offset` [¶](09_03_a_virtual_machine_for_guile.md)

If the last comparison result is not `NONE`, add offset, a signed 24-bit number, to the current instruction pointer.

This is intended for use after a `<?` comparison, and is different from `jl` in the way it handles not-a-number (NaN) values: `<?` sets `INVALID` instead of `NONE` if either value is a NaN. For exact numbers, `jnge` is the same as `jl`.

Instruction: **jtable** `s24:idx v32:length [x8:_ l24:offset]...` [¶](09_03_a_virtual_machine_for_guile.md)

Branch to an entry in a table, as in C’s `switch` statement. idx is a `u64` local indicating which entry to branch to. The immediate len indicates the number of entries in the table, and should be greater than or equal to 1. The last entry in the table is the "catch-all" entry. The offset... values are signed 24-bit immediates (`l24` encoding), indicating a memory address as a number of 32-bit words away from the current instruction pointer.

* * *

Previous: [Branch Instructions](09_03_a_virtual_machine_for_guile.md#93715-branch-instructions), Up: [Instruction Set](09_03_a_virtual_machine_for_guile.md#937-instruction-set)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 9.3.7.16 Raw Memory Access Instructions [¶](09_03_a_virtual_machine_for_guile.md#93716-raw-memory-access-instructions)

Bytevector operations correspond closely to what the current hardware can do, so it makes sense to inline them to VM instructions, providing a clear path for eventual native compilation. Without this, Scheme programs would need other primitives for accessing raw bytes – but these primitives are as good as any.

Instruction: **u8-ref** `s8:dst s8:ptr s8:idx` [¶](09_03_a_virtual_machine_for_guile.md)

Instruction: **s8-ref** `s8:dst s8:ptr s8:idx` [¶](09_03_a_virtual_machine_for_guile.md)

Instruction: **u16-ref** `s8:dst s8:ptr s8:idx` [¶](09_03_a_virtual_machine_for_guile.md)

Instruction: **s16-ref** `s8:dst s8:ptr s8:idx` [¶](09_03_a_virtual_machine_for_guile.md)

Instruction: **u32-ref** `s8:dst s8:ptr s8:idx` [¶](09_03_a_virtual_machine_for_guile.md)

Instruction: **s32-ref** `s8:dst s8:ptr s8:idx` [¶](09_03_a_virtual_machine_for_guile.md)

Instruction: **u64-ref** `s8:dst s8:ptr s8:idx` [¶](09_03_a_virtual_machine_for_guile.md)

Instruction: **s64-ref** `s8:dst s8:ptr s8:idx` [¶](09_03_a_virtual_machine_for_guile.md)

Instruction: **f32-ref** `s8:dst s8:ptr s8:idx` [¶](09_03_a_virtual_machine_for_guile.md)

Instruction: **f64-ref** `s8:dst s8:ptr s8:idx` [¶](09_03_a_virtual_machine_for_guile.md)

Fetch the item at byte offset idx from the raw pointer local ptr, and store it in dst. All accesses use native endianness.

The idx value should be an unboxed unsigned 64-bit integer.

The results are all written to the stack as unboxed values, either as signed 64-bit integers, unsigned 64-bit integers, or IEEE double floating point numbers.

Instruction: **u8-set!** `s8:ptr s8:idx s8:val` [¶](09_03_a_virtual_machine_for_guile.md)

Instruction: **s8-set!** `s8:ptr s8:idx s8:val` [¶](09_03_a_virtual_machine_for_guile.md)

Instruction: **u16-set!** `s8:ptr s8:idx s8:val` [¶](09_03_a_virtual_machine_for_guile.md)

Instruction: **s16-set!** `s8:ptr s8:idx s8:val` [¶](09_03_a_virtual_machine_for_guile.md)

Instruction: **u32-set!** `s8:ptr s8:idx s8:val` [¶](09_03_a_virtual_machine_for_guile.md)

Instruction: **s32-set!** `s8:ptr s8:idx s8:val` [¶](09_03_a_virtual_machine_for_guile.md)

Instruction: **u64-set!** `s8:ptr s8:idx s8:val` [¶](09_03_a_virtual_machine_for_guile.md)

Instruction: **s64-set!** `s8:ptr s8:idx s8:val` [¶](09_03_a_virtual_machine_for_guile.md)

Instruction: **f32-set!** `s8:ptr s8:idx s8:val` [¶](09_03_a_virtual_machine_for_guile.md)

Instruction: **f64-set!** `s8:ptr s8:idx s8:val` [¶](09_03_a_virtual_machine_for_guile.md)

Store val into memory pointed to by raw pointer local ptr, at byte offset idx. Multibyte values are written using native endianness.

The idx value should be an unboxed unsigned 64-bit integer.

The val values are all unboxed, either as signed 64-bit integers, unsigned 64-bit integers, or IEEE double floating point numbers.

* * *

Previous: [Instruction Set](09_03_a_virtual_machine_for_guile.md#937-instruction-set), Up: [A Virtual Machine for Guile](09_03_a_virtual_machine_for_guile.md#93-a-virtual-machine-for-guile)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 9.3.8 Just-In-Time Native Code [¶](09_03_a_virtual_machine_for_guile.md#938-just-in-time-native-code)

The final piece of Guile’s virtual machine is a just-in-time (JIT) compiler from bytecode instructions to native code. It is faster to run a function when its bytecode instructions are compiled to native code, compared to having the VM interpret the instructions.

The JIT compiler runs automatically, triggered by counters associated with each function. The counter increments when functions are called and during each loop iteration. Once a function’s counter passes a certain value, the function gets JIT-compiled. See [Instrumentation Instructions](09_03_a_virtual_machine_for_guile.md#9376-instrumentation-instructions), for full details.

Guile’s JIT compiler is what is known as a _template JIT_. This kind of JIT is very simple: for each instruction in a function, the JIT compiler will emit a generic sequence of machine code corresponding to the instruction kind, specializing that generic template to reference the specific operands of the instruction being compiled.

The strength of a template JIT is principally that it is very fast at emitting code. It doesn’t need to do any time-consuming analysis on the bytecode that it is compiling to do its job.

A template JIT is also very predictable: the native code emitted by a template JIT has the same performance characteristics of the corresponding bytecode, only that it runs faster. In theory you could even generate the template-JIT machine code ahead of time, as it doesn’t depend on any value seen at run-time.

This predictability makes it possible to reason about the performance of a system in terms of bytecode, knowing that the conclusions apply to native code emitted by a template JIT.

Because the machine code corresponding to an instruction always performs the same tasks that the interpreter would do for that instruction, bytecode and a template JIT also allows Guile programmers to debug their programs in terms of the bytecode model. When a Guile programmer sets a breakpoint, Guile will disable the JIT for the thread being debugged, falling back to the interpreter (which has the corresponding code to run the hooks). See [VM Hooks](06_26_debugging_infrastructure.md#62651-vm-hooks).

To emit native code, Guile uses a forked version of GNU Lightning. This "Lightening" effort, spun out as a separate project, aims to build on the back-end support from GNU Lightning, but adapting the API and behavior of the library to match Guile’s needs. This code is included in the Guile source distribution. For more information, see [https://gitlab.com/wingo/lightening](https://gitlab.com/wingo/lightening). As of mid-2019, Lightening supports code generation for the x86-64, ia32, ARMv7, and AArch64 architectures.

The weaknesses of a template JIT are two-fold. Firstly, as a simple back-end that has to run fast, a template JIT doesn’t have time to do analysis that could help it generate better code, notably global register allocation and instruction selection.

However this is a minor weakness compared to the inability to perform significant, speculative program transformations. For example, Guile could see that in an expression `(f x)`, that in practice f always refers to the same function. An advanced JIT compiler would speculatively inline f into the call-site, along with a dynamic check to make sure that the assertion still held. But as a template JIT doesn’t pay attention to values only known at run-time, it can’t make this transformation.

This limitation is mitigated in part by Guile’s robust ahead-of-time compiler which can already perform significant optimizations when it can prove they will always be valid, and its low-level bytecode which is able to represent the effect of those optimizations (e.g. elided type-checks). See [Compiling to the Virtual Machine](09_04_compiling_to_the_virtual_machine.md#94-compiling-to-the-virtual-machine), for more on Guile’s compiler.

An ahead-of-time Scheme-to-bytecode strategy, complemented by a template JIT, also particularly suits the somewhat static nature of Scheme. Scheme programmers often write code in a way that makes the identity of free variable references lexically apparent. For example, the `(f x)` expression could appear within a `(let ((f (lambda (x) (1+ x)))) ...)` expression, or we could see that `f` was imported from a particular module where we know its binding. Ahead-of-time compilation techniques can work well for a language like Scheme where there is little polymorphism and much first-order programming. They do not work so well for a language like JavaScript, which is highly mutable at run-time and difficult to analyze due to method calls (which are effectively higher-order calls).

All that said, a template JIT works well for Guile at this point. It’s only a few thousand lines of maintainable code, it speeds up Scheme programs, and it keeps the bulk of the Guile Scheme implementation written in Scheme itself. The next step is probably to add ahead-of-time native code emission to the back-end of the compiler written in Scheme, to take advantage of the opportunity to do global register allocation and instruction selection. Once this is working, it can allow Guile to experiment with speculative optimizations in Scheme as well. See [Extending the Compiler](09_04_compiling_to_the_virtual_machine.md#947-extending-the-compiler), for more on future directions.

Finally, note that there are a few environment variables that can be tweaked to make JIT compilation happen sooner, later, or never. See [Environment Variables](04_programming_in_scheme.md#422-environment-variables), for more.

* * *

Previous: [A Virtual Machine for Guile](09_03_a_virtual_machine_for_guile.md#93-a-virtual-machine-for-guile), Up: [Guile Implementation](09_00_guile_implementation.md#9-guile-implementation)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
