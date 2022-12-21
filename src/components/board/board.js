// Import styling for this component
import "./board.css";
import React, { useState } from "react";
import { ruling } from "../../definitions";

// Accurately calculate the difference between two numbers
function diff(start, end) {
  if (start > end) {
    return start - end;
  } else {
    return end - start;
  }
}

function Board({ state, move, player, data, timer, wMissing, bMissing }) {
  const [highlight, setHighlight] = useState([]);
  const [hits, setHits] = useState([]);
  const [selected, setSelected] = useState(null);

  const letters = ["a", "b", "c", "d", "e", "f", "g", "h"];
  const numbers = [8, 7, 6, 5, 4, 3, 2, 1];

  const processHighlight = async (piece, cell) => {
    if (highlight.length) {
      // Piece move is valid
      if (highlight.includes(cell)) {
        await move(selected, cell);
      }

      setSelected(null);
      setHighlight([]);
      setHits([]);
    } else {
      const c = [cell[0], cell[1]];

      // Initialise available tile lists
      const list = [];
      const hits = [];

      // Prevent same player from moving piece
      if (
        (piece[0] === "b" && player === "white") ||
        (piece[0] === "w" && player === "black")
      ) {
        return;
      }

      // Current piece
      const p = piece.substring(2);

      const ruleSet = ruling[p];

      // Handle positioning logic
      let blocked = false;
      let hardBlock = false;
      let irregular = false;
      let special = false;

      // Loop through rulebook for chosen piece
      ruleSet.forEach((rule, index) => {
        const move = rule.move;
        const white = piece[0] !== "w";
        const current = letters.findIndex((l) => l === c[0]);

        // Literal string value of the tile, with letter + number
        const nextTile = `${letters[current + (white ? -move[0] : +move[0])]}${
          parseInt(c[1]) + (white ? -move[1] : +move[1])
        }`;

        const tileState = state[nextTile];

        /* If movement difference between previous rule and current rule is 
           greater than 1, then unblock highlight for consequent tiles */
        if (
          (ruleSet[index - 1] &&
            (diff(ruleSet[index - 1].move[0], move[0]) > 1 ||
              diff(ruleSet[index - 1].move[1], move[1]) > 1)) ||
          irregular
        ) {
          blocked = false;
          hardBlock = false;

          // L shape is the odd one out, this creates an exception
          if (move[0] !== -1 && move[1] !== 1 && p === "knight") {
            irregular = true;
          }
        }

        if (
          !(tileState && tileState.startsWith(white ? "b" : "w")) &&
          !blocked &&
          !hardBlock
        ) {
          if (
            (p !== "pawn" ||
              !(tileState && tileState.startsWith(white ? "w" : "b"))) &&
            !special
          ) {
            if (tileState && tileState.startsWith(white ? "w" : "b")) {
              if (
                (move[0] === 1 && move[0] === -1) ||
                (move[1] === 1 && move[1] === -1)
              ) {
              } else {
                hardBlock = true;
              }
              hits.push(nextTile);
            }

            list.push(nextTile);
          } else {
            special = true;
          }

          if (p === "pawn" && piece[0] === "w") {
            // Diagonal-left check for pawn
            const topLeft = `${letters[current - 1]}${parseInt(c[1]) + 1}`;
            const topLeftState = state[topLeft];

            if (topLeftState && topLeftState.startsWith(white ? "w" : "b")) {
              hits.push(topLeft);
              list.push(topLeft);
            }

            // Diagonal-right check for pawn
            const topRight = `${letters[current + 1]}${parseInt(c[1]) + 1}`;
            const topRightState = state[topRight];

            if (topRightState && topRightState.startsWith(white ? "w" : "b")) {
              hits.push(topRight);
              list.push(topRight);
            }
          }

          if (p === "pawn" && piece[0] === "b") {
            // Diagonal-right check for pawn
            const bottomRight = `${letters[current + 1]}${parseInt(c[1]) - 1}`;
            const bottomRightState = state[bottomRight];

            if (
              bottomRightState &&
              bottomRightState.startsWith(white ? "w" : "b")
            ) {
              hits.push(bottomRight);
              list.push(bottomRight);
            }

            // Diagonal-right check for pawn
            const bottomLeft = `${letters[current - 1]}${parseInt(c[1]) - 1}`;
            const bottomLeftState = state[bottomLeft];

            if (
              bottomLeftState &&
              bottomLeftState.startsWith(white ? "w" : "b")
            ) {
              hits.push(bottomLeft);
              list.push(bottomLeft);
            }
          }
        } else {
          blocked = true;
        }
      });

      setHighlight(list);
      setHits(hits);
      setSelected(cell);
    }
  };

  return (
    <div className="board">
      {/* Display game end overlay */}
      {(data.mate || timer) && (
        <div className="end">
          <div className="endContainer">
            <span className="endTitle">
              {data.check ? "Check Mate" : "Timer has Ended"}
            </span>
            <img alt={"player"} src={`gameplay/enabled.svg`}></img>
            {/* If game finishes due to timer, prioritise player with most pieces */}
            <span className="playerWon">
              Player{" "}
              {timer
                ? wMissing > bMissing
                  ? 2
                  : 1
                : data.player === "white"
                ? 1
                : 2}{" "}
              has Won!
            </span>
          </div>
        </div>
      )}

      {letters.map((letter, col) => (
        <div key={col} className="column">
          {numbers.map((number, row) => {
            const cell = `${letter}${number}`;
            const mapped = state[cell];
            return (
              <div
                key={row}
                style={{
                  ...(((data.check &&
                    mapped === "w_king" &&
                    data.player === "white") ||
                    (data.check &&
                      mapped === "b_king" &&
                      data.player === "black")) && {
                    backgroundColor: "#d14747",
                  }),
                }}
                className={`cell ${letter}${number} ${
                  highlight.includes(cell) ? "highlight" : ""
                } ${hits.includes(cell) ? "hit" : ""}`}
                onClick={async () => {
                  await processHighlight(mapped, cell);
                }}
              >
                {mapped ? (
                  <img
                    alt={`${letter}${number}`}
                    className="piece"
                    src={`pieces/${state[`${letter}${number}`]}.svg`}
                  ></img>
                ) : null}
              </div>
            );
          })}
        </div>
      ))}
    </div>
  );
}

export default Board;
