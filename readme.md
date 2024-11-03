This is an implementation of the repo [Markov Junior](https://github.com/mxgmn/MarkovJunior)
It is a 2d implementation of markovian rewrite rules, currently only random replace works.

To use, type
```bash
cargo run "src/programs/hello.mj"
```

What this does is take the rules from the script and then iterate it on a grid.
For example in hello.mj, it contains
```mj
(bw=bb)
(w=b)
```

Starting with a white grid there are no blue tiles (b) so first rule is incompletable 
(bw=bb)

Next b=w is possible because there is a white tile to convert to blue 
(b=w)

After this now it converts a white tile next to a blue tile into both blue and this repeats until the board fills
(bw=bb)

The implementation is dreadfully slow as of right now because it is single threaded and the opimization is broken.