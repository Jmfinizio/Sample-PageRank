# Sample-PageRank
The PageRank algorithm reads a .txt file in the format (Node, neighbor), representing all  neighboring edges in the graph. 100 random walks
are computed at each starting node. If a node has any neighbors, there is a 90% chance that the next step in the walk will be on a random neighbor.
There is also a 10% chance that the next step in the walk will be to any node in the graph. If a node does not have any neighbors, then the next step will
be to a random node. The amount of times a walk terminates at a certain node will determine its pagerank value.
