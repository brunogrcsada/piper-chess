export const board = {
  a1: "w_rook",
  b1: "w_knight",
  c1: "w_bishop",
  d1: "w_queen",
  e1: "w_king",
  f1: "w_bishop",
  g1: "w_knight",
  h1: "w_rook",
  a2: "w_pawn",
  b2: "w_pawn",
  c2: "w_pawn",
  d2: "w_pawn",
  e2: "w_pawn",
  f2: "w_pawn",
  g2: "w_pawn",
  h2: "w_pawn",
  a3: null,
  b3: null,
  c3: null,
  d3: null,
  e3: null,
  f3: null,
  g3: null,
  h3: null,
  a4: null,
  b4: null,
  c4: null,
  d4: null,
  e4: null,
  f4: null,
  g4: null,
  h4: null,
  a5: null,
  b5: null,
  c5: null,
  d5: null,
  e5: null,
  f5: null,
  g5: null,
  h5: null,
  a6: null,
  b6: null,
  c6: null,
  d6: null,
  e6: null,
  f6: null,
  g6: null,
  h6: null,
  a7: "b_pawn",
  b7: "b_pawn",
  c7: "b_pawn",
  d7: "b_pawn",
  e7: "b_pawn",
  f7: "b_pawn",
  g7: "b_pawn",
  h7: "b_pawn",
  a8: "b_rook",
  b8: "b_knight",
  c8: "b_bishop",
  d8: "b_queen",
  e8: "b_king",
  f8: "b_bishop",
  g8: "b_knight",
  h8: "b_rook",
};

let rules = {
  pawn: [
    {
      move: [0, 1],
    },
    {
      move: [0, 2],
    },
  ],
  king: [
    {
      move: [0, 1],
    },
    {
      move: [0, -1],
    },
    {
      move: [1, 0],
    },
    {
      move: [-1, 0],
    },
    {
      move: [1, 1],
    },
    {
      move: [-1, -1],
    },
    {
      move: [-1, 1],
    },
  ],
  knight: [
    {
      move: [1, 2],
    },
    {
      move: [2, 1],
    },
    {
      move: [-2, -1],
    },
    {
      move: [2, -1],
    },
    {
      move: [-2, 1],
    },
    {
      move: [-1, 2],
    },
    {
      move: [1, -2],
    },
    {
      move: [-1, -2],
    },
  ],
  rook: [
    {
      move: [0, 1],
    },
    {
      move: [0, 2],
    },
    {
      move: [0, 3],
    },
    {
      move: [0, 4],
    },
    {
      move: [0, 5],
    },
    {
      move: [0, 6],
    },
    {
      move: [0, 7],
    },
    {
      move: [0, -1],
    },
    {
      move: [0, -2],
    },
    {
      move: [0, -3],
    },
    {
      move: [0, -4],
    },
    {
      move: [0, -5],
    },
    {
      move: [0, -6],
    },
    {
      move: [0, -7],
    },

    {
      move: [1, 0],
    },
    {
      move: [2, 0],
    },
    {
      move: [3, 0],
    },
    {
      move: [4, 0],
    },
    {
      move: [5, 0],
    },
    {
      move: [6, 0],
    },
    {
      move: [7, 0],
    },
    {
      move: [-1, 0],
    },
    {
      move: [-2, 0],
    },
    {
      move: [-3, 0],
    },
    {
      move: [-4, 0],
    },
    {
      move: [-5, 0],
    },
    {
      move: [-6, 0],
    },
    {
      move: [-7, 0],
    },
  ],
  bishop: [
    {
      move: [1, 1],
    },
    {
      move: [2, 2],
    },
    {
      move: [3, 3],
    },
    {
      move: [4, 4],
    },
    {
      move: [5, 5],
    },
    {
      move: [6, 6],
    },
    {
      move: [1, -1],
    },
    {
      move: [2, -2],
    },
    {
      move: [3, -3],
    },
    {
      move: [4, -4],
    },
    {
      move: [5, -5],
    },
    {
      move: [6, -6],
    },
    {
      move: [-1, -1],
    },
    {
      move: [-2, -2],
    },
    {
      move: [-3, -3],
    },
    {
      move: [-4, -4],
    },
    {
      move: [-5, -5],
    },
    {
      move: [-6, -6],
    },
    {
      move: [-1, 1],
    },
    {
      move: [-2, 2],
    },
    {
      move: [-3, 3],
    },
    {
      move: [-4, 4],
    },
    {
      move: [-5, 5],
    },
    {
      move: [-6, 6],
    },
  ],
};

rules.queen = rules.rook.concat(rules.bishop);

export const ruling = rules;
