# reference_counting_benchmark
A simple benchmark to demonstrate the cost of an atomic operation

Here is my results using Intel Core i7 8750H under Windows 10 :
_____________________________________________
TRYING WITH SINGLE ARC SHARED BETWEEN THREADS
240 662 microseconds
_____________________________________________
_____________________________________________
TRYING WITH AN ARC IN A ARC FOR EACH THREADS
17 784 microseconds
_____________________________________________
_____________________________________________
TRYING BY CLONING INTO AN RC FOR EACH THREADS
1 511 microseconds
_____________________________________________
_____________________________________________
TRYING BY CLONING AN ARC INTO AN RC FOR EACH THREADS
1 589 microseconds
_____________________________________________
_____________________________________________
TRYING BY CLONING THE STRING
323 956 microseconds
_____________________________________________
