[w 2 2] // white tile at 2,2, (starting config at top of file)
[steps 2] // limits steps to 2, (0, 1, 2, stop), (omit to have run until no matches left)
$CustomColor = 3 3 3 // rgb

(b=w) // black is randomly white
{b=w} // all black is white (greedy)
(bw=rw) // black white is red white
(b=w r=b) // simultanious actions randomly
{b=w r=b} // greedy simultanious actions randomly
(b=*) // black is any tile, (wildcard)
(b=*) // black is any tile, (wildcard)
(b=*!w|b) // black is any tile, except white or black
(b=$CustomColor$) // custom color

// loop on tile sequence existing, (exampe same as {w=b})
w{
(w=b)
}
// can set max iterations, (doubles as if with 1)
1wb{
(wb=rr)
}
