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