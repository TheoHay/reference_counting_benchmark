# reference_counting_benchmark
A simple benchmark to demonstrate the cost of an atomic operation

Here is my results using Intel Core i7 8750H under Windows 10 :

Single shared Arc with Small Object took 455,808 μs
Single shared Arc with Medium Object took 455,408 μs
Single shared Arc with Big Object took 451,715 μs

Object cloned into Arc with Small Object took 25,266 μs
Object cloned into Arc with Medium Object took 28,879 μs
Object cloned into Arc with Big Object took 25,353 μs

Arc cloned into Arc with Small Object took 29,735 μs
Arc cloned into Arc with Medium Object took 21,832 μs
Arc cloned into Arc with Big Object took 28,520 μs

Arc cloned into RC with Small Object took 697 μs
Arc cloned into RC with Medium Object took 616 μs
Arc cloned into RC with Big Object took 654 μs

Object cloned into Rc with Small Object took 609 μs
Object cloned into Rc with Medium Object took 587 μs
Object cloned into Rc with Big Object took 600 μs

Full Cloning with Small Object took 45,188 μs
Full Cloning with Medium Object took 56,318 μs
Full Cloning with Big Object took 64,595 μs
