# graph_api

This is a simple graph API backed by an adjacency matrix. In the end I kept it
uncomplicated (but all working!) rather than trying to achieve anything 
complicated. Therefore, its serious weak points are:
- vertex indices that are nothing more than `usize`s and are totally 
invalidated when vertices are deleted
- accessor methods that expect valid edge/vertex indices as input and panic
otherwise

However, I do allow arbitrary data to be stored at the edges and vertices!