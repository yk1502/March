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