# Overview

Six different algorithms were implemented: 
Genetic, Greedy, Nearest Neighbour, Simulated Annealing, Random Swapping, and Ant Colony Optimisation. 

They tested in a map of a random number of cities that were randomly positioned.

All implementations were coded in Rust.

# Configuration

5 minutes were given for all the iterative algorithms.

The temperature graph used for simulated annealing was 0.999994^x where x is the number of iterations that have occured.

The genetic algorithm started with a population of of 100,000 random paths. The worse half in terms of performance was killed off and the better half was cloned 2 of the cities were swapped in the clone. This was repeated and the cost top path was used after all iterations.

For all tests of Ant Colony Optimization, the number of ants  was kept at 5,000, the coefficient of distance was kept at 1, and the coefficient of reward was kept at 0.2 .

# Results
<pre>
The mean ratios for distance / one-tree go as follows (closer to a value of 1 is better): 

Genetic                   1.12547

Greedy Heuristic          1.25549

Nearest Neighbour         1.30237

Simulated Annealing       1.33065

Random Swapping           1.44725

Ant colony Optimisation   1.58841
</pre>

![](https://github.com/elementary-luke/TSP-Pathfinding/blob/main/graphed_results.png?raw=true)
  


# Notes
It's cool to see that simulated annealing does much better than random swapping, even though it's annealing is based on swapping. This definitely shows the importance of early exploration and then honing into an optimal solution later on, where random swapping may converge too early onto a local minimum, and therefore no swap leads to an improvement.

The performance Greedy and Nearest Neighbour is very impressive considering that they can be computed very quickly and are definitely very human approaches.

The gradients on the graph show how well the algorithms scale with increasing numbers of cities, and we can see just how impressive the genetic algorithm is, with a very flat line.

On the other hand, while ant colony optimisation works really well when the city count is low, it seems to scale really badly. However, this is more likely my fault than the algorithms as there are many perameters to fine tune, and it may need more time for graphs with more nodes. I just didn't experiment that much.

Like ant colony optimisation, it would've been good if I was able to give more time to the iterative solutions, or maybe it would've been more fair to judge them on the number of iterations rather than time, since some are more complex, but I thought 5 minutes was a reasonable amount of time someone might wait and I didn't want to wait too long to get results.

My sample size also wasn't massive. random swapping and simulated annealing were tested 134 times each. This means that for each map of a certain city size, the result was the average of around 4 tests, and for the rest of the algorithms, it was the average of 12.

One big algorithm left out was the Christofides algorithm, which is one of the best ones. I would have loved to implement it, but I couldn't implement Edmond's Blossom Algorithm for minimum weight perfect matching, so I'll come back to it later.
