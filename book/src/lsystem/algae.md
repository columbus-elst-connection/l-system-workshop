# Algae
Aristid Lindenmayer introduces a system of substitutions that would later bear his name. He did this in a set of two articles; _Mathematical Models for Cellular Interactions in Development_. As the title implies it offers a mathematical theory to model growth in certain type of cells. Stripping away the mathematics his article described an example that modeled algae. A more modern description follows below.

We will study _words_, i.e. a sequence of symbols, over an _alphabet_. In our example we take our alphabet to be the symbols *A*, and *B*. We will examine a series of words. In the algae example we are starting with the word *A*. 
With the current word in the series, we replace each symbol with a sequence of symbols and concatenating the sub-sequences in a new word. In our example we will replace each occurring symbol *A* with the sequence *AB*, and each occurring symbol *B* with the sequence *A*.

Below you find the first few iterations of this process.

1. *A*, our starting word.
2. *AB*, because the single *A* is replaced with *AB*.
3. *ABA*, because the single *A* is replaced with *AB* and the *B* is replaced with *A*.
4. *ABAAB*, because each *A* is replaced with *AB* and each *B* is replaced with *A*.

In the above model the symbol *A* is a model for a mature cell, ready to divide itself. The division however is asymmetric. It allows the original cell to comfortable remain in place, making place for a young cell *B*. In its stead, the young cell *B* first must mature to and become an *A* cell before it can start reproducing itself.

Although this is a simple model. A lot can be learned from this. For an few examples, see the exercises.

## Exercises
1. Extend the example with the a few iterations.
2. Count the number of symbols in each word of the series. Guess what number comes next.
3. Count the number of *A*'s and the number of *B*'s separately. What do you get.
4. We will number the words in our series, \\(W_{0}\\) for our start word, \\(W_{1}\\), for the next, \\(W_{2}\\) for the one after that, etcetera. Pick any number, let's say 4. Notice how \\(W_{4}\\), i.e. *ABAAB* is the concatenation of *ABA*, which is \\(W_{3}\\), and *AB*, which is \\(W_{2}\\). In other words \\(W_{4}=W_{3}W_{2}\\).

1. Check if something similar holds for \\(W_{3}\\) and \\(W_{5}\\).
2. Does this property, i.e. \\(W_{k} = W_{k-1}W_{k-2}\\) always hold?
