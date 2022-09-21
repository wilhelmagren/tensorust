# Holistic testing of tensorust
This directory is intended to be used for various performance tests, proof-of-concept
experiments, and memory analytics. For example, in-memory vector layout, cache hits,
array access metrics and similarly. The idea is for these experiments and practical
tests to help define a proper layout and structure of the Tensor struct to be
implemented. Striving for efficient memory reading, writing, and moving/copying.

## Results
Here I will list a summary of all the experiment/test that have been performed; what
was the objective, where there any potential hypotheses, how did the results look, and
what can be concluded.

- [Row-or-column-order-looping]

# License
All code is written under an Apache-2.0 styled license, please see [LICENSE](https://github.com/willeagren/tensorust/blob/main/LICENSE)
for more information.
