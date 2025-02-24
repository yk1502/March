### Update 1
A Random mover

```
--------------------------------------------------
Results of new vs old (8+0.08, 1t, 1MB, UHO_2024_8mvs_big_+080_+099.pgn):
Elo: -0.30 +/- 1.78, nElo: -0.78 +/- 4.60
LOS: 37.02 %, DrawRatio: 73.57 %, PairsRatio: 0.99
Games: 21930, Wins: 1619, Losses: 1638, Draws: 18673, Points: 10955.5 (49.96 %)
Ptnml(0-2): [71, 1382, 8067, 1385, 60]
LLR: -2.98 (-2.94, 2.94) [0.00, 5.00]
--------------------------------------------------
```


### Update 2
Basic chess engine:
- Negamax
- Iterative Deepening
- Basic time management 
- Material counting
- Checkmate and stalemate detection

```
info depth 1 nodes 20 time 0 score cp 0
info depth 2 nodes 440 time 0 score cp 0
info depth 3 nodes 9762 time 0 score cp 0
info depth 4 nodes 216365 time 8 score cp 0
info depth 5 nodes 5288577 time 106 score cp 100
bestmove b2b3
```

```
--------------------------------------------------
Results of new vs old (8+0.08, 1t, 1MB, UHO_2024_8mvs_big_+080_+099.pgn):
Elo: 865.92 +/- 51.84, nElo: 2917.31 +/- 11.58
LOS: 100.00 %, DrawRatio: 0.06 %, PairsRatio: inf
Games: 3458, Wins: 3411, Losses: 0, Draws: 47, Points: 3434.5 (99.32 %)
Ptnml(0-2): [0, 0, 1, 45, 1683]
LLR: 2.94 (-2.94, 2.94) [0.00, 5.00]
--------------------------------------------------
```


### Update 3
- Alpha beta pruning

```
info depth 1 nodes 20 time 0 score cp 0
info depth 2 nodes 79 time 0 score cp 0
info depth 3 nodes 664 time 0 score cp 0
info depth 4 nodes 2102 time 0 score cp 0
info depth 5 nodes 41260 time 4 score cp 100
info depth 6 nodes 857431 time 31 score cp -100
info depth 7 nodes 1999725 time 62 score cp 100
bestmove a2a3
```

```
--------------------------------------------------
Results of new vs old (8+0.08, 1t, 1MB, UHO_2024_8mvs_big_+080_+099.pgn):
Elo: 287.61 +/- 25.44, nElo: 449.77 +/- 26.00
LOS: 100.00 %, DrawRatio: 9.91 %, PairsRatio: 43.14
Games: 686, Wins: 486, Losses: 20, Draws: 180, Points: 576.0 (83.97 %)
Ptnml(0-2): [0, 7, 34, 131, 171]
LLR: 2.95 (-2.94, 2.94) [0.00, 5.00]
--------------------------------------------------
```



### Update 4
- MVV-LVA

```
info depth 1 nodes 20 time 0 score cp 0
info depth 2 nodes 79 time 0 score cp 0
info depth 3 nodes 664 time 0 score cp 0
info depth 4 nodes 2098 time 0 score cp 0
info depth 5 nodes 26010 time 5 score cp 100
info depth 6 nodes 359287 time 29 score cp -100
info depth 7 nodes 991023 time 68 score cp 100
bestmove a2a3
```

```
--------------------------------------------------
Results of new vs old (8+0.08, 1t, 1MB, UHO_2024_8mvs_big_+080_+099.pgn):
Elo: 123.00 +/- 19.95, nElo: 185.39 +/- 27.66
LOS: 100.00 %, DrawRatio: 32.01 %, PairsRatio: 6.10
Games: 606, Wins: 267, Losses: 61, Draws: 278, Points: 406.0 (67.00 %)
Ptnml(0-2): [1, 28, 97, 118, 59]
LLR: 2.95 (-2.94, 2.94) [0.00, 5.00]
--------------------------------------------------
```



### Update 5
- QSearch

```
info depth 1 nodes 20 time 0 score cp 0
info depth 2 nodes 83 time 0 score cp 0
info depth 3 nodes 610 time 0 score cp 0
info depth 4 nodes 2330 time 1 score cp 0
info depth 5 nodes 15833 time 4 score cp 0
info depth 6 nodes 65918 time 18 score cp 0
info depth 7 nodes 480478 time 52 score cp 0
```

```
--------------------------------------------------
Results of new vs old (8+0.08, 1t, 1MB, UHO_2024_8mvs_big_+080_+099.pgn):
Elo: 55.58 +/- 15.27, nElo: 78.47 +/- 21.18
LOS: 100.00 %, DrawRatio: 37.14 %, PairsRatio: 2.22
Games: 1034, Wins: 347, Losses: 183, Draws: 504, Points: 599.0 (57.93 %)
Ptnml(0-2): [19, 82, 192, 164, 60]
LLR: 2.97 (-2.94, 2.94) [0.00, 5.00]
--------------------------------------------------
```



### Update 6
- MVV-LVA in QSearch

```
info depth 1 nodes 20 time 0 score cp 0
info depth 2 nodes 83 time 0 score cp 0
info depth 3 nodes 610 time 0 score cp 0
info depth 4 nodes 2321 time 1 score cp 0
info depth 5 nodes 15808 time 5 score cp 0
info depth 6 nodes 65631 time 17 score cp 0
info depth 7 nodes 478047 time 51 score cp 0
info depth 8 nodes 2258730 time 353 score cp 0
bestmove a2a3
```

```
--------------------------------------------------
Results of new vs old (8+0.08, 1t, 1MB, UHO_2024_8mvs_big_+080_+099.pgn):
Elo: 26.78 +/- 8.94, nElo: 53.69 +/- 17.85
LOS: 100.00 %, DrawRatio: 63.32 %, PairsRatio: 1.93
Games: 1456, Wins: 315, Losses: 203, Draws: 938, Points: 784.0 (53.85 %)
Ptnml(0-2): [5, 86, 461, 144, 32]
LLR: 2.96 (-2.94, 2.94) [0.00, 5.00]
--------------------------------------------------
```



### Update 7
- Repetition detection

```
info depth 1 nodes 20 time 0 score cp 0
info depth 2 nodes 83 time 0 score cp 0
info depth 3 nodes 610 time 0 score cp 0
info depth 4 nodes 2321 time 2 score cp 0
info depth 5 nodes 15808 time 10 score cp 0
info depth 6 nodes 65631 time 30 score cp 0
info depth 7 nodes 478047 time 66 score cp 0
info depth 8 nodes 2258730 time 371 score cp 0
bestmove a2a3
```

```
--------------------------------------------------
Results of new vs old (8+0.08, 1t, 1MB, UHO_2024_8mvs_big_+080_+099.pgn):
Elo: 18.66 +/- 7.59, nElo: 36.07 +/- 14.65
LOS: 100.00 %, DrawRatio: 59.85 %, PairsRatio: 1.84
Games: 2162, Wins: 653, Losses: 537, Draws: 972, Points: 1139.0 (52.68 %)
Ptnml(0-2): [32, 121, 647, 261, 20]
LLR: 2.95 (-2.94, 2.94) [0.00, 5.00]
--------------------------------------------------
```