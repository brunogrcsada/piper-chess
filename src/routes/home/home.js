// Components
import Board from "../../components/board/board";
import Player from "../../components/player/player";
import Switch from "../../components/switch/switch";
import Timer from "../../components/timer/timer";
import { board } from "../../definitions";

import React, { useEffect, useState } from "react";

// CSS
import "./home.css";

function Home() {
  // Current player
  const [player, setPlayer] = useState("white");

  // Default game state from local definition
  const [state, setState] = useState(board);

  // All backend data returned
  const [data, setData] = useState({ check: false, mate: false, valid: true });

  // Default game state from local definition
  const [isTimerFinished, setTimer] = useState(false);

  // Retrieve initial game state from backend
  useEffect(() => {
    async function fetchData() {
      fetch(`http://localhost:2020/state`)
        .then((res) => res.json())
        .then((res) => {
          console.log("RES: ", res);
          if (res) {
            setData(res);
            setState(res.world);
            setPlayer(res.player);
          }
        });
    }
    fetchData();
  }, []);

  // Update board and return validated game state
  const makeMove = async (from, to) => {
    fetch(`http://localhost:2020/move/${from}/${to}`)
      .then((res) => res.json())
      .then((res) => {
        if (res) {
          setData(res);
          setState(res.world);
          setPlayer(res.player);
        }
      });
  };

  const stateValues = Object.values(state);

  let counts = {
    pawn: 8,
    knight: 2,
    bishop: 2,
    queen: 1,
    king: 1,
    rook: 2,
  };

  let calc = {
    b_pawn: 0,
    b_knight: 0,
    b_bishop: 0,
    b_queen: 0,
    b_king: 0,
    b_rook: 0,
    w_pawn: 0,
    w_knight: 0,
    w_bishop: 0,
    w_queen: 0,
    w_king: 0,
    w_rook: 0,
  };

  const whiteMissing = [];
  const blackMissing = [];

  for (let x = 0; x < stateValues.length; x++) {
    // Increase individual counter
    calc[stateValues[x]] += 1;
  }

  Object.entries(calc).forEach((entry) => {
    // Get piece name only
    const piece = entry[0].substring(2);

    const value = entry[1];

    const original = counts[piece];

    if (value < original && entry[0].startsWith("w")) {
      whiteMissing.push(...new Array(original - value).fill(entry[0]));
    } else if (value <= original) {
      blackMissing.push(...new Array(original - value).fill(entry[0]));
    }
  });

  return (
    <div className="Home">
      <Timer set={setTimer}></Timer>
      <div className="switchWrap">
        <Switch />
      </div>
      <div className="center" data-testid="centerContainer">
        <Player
          enabled={player === "white"}
          name={"Player 1"}
          missing={blackMissing}
        ></Player>

        <Board
          wMissing={whiteMissing.length}
          bMissing={blackMissing.length}
          timer={isTimerFinished}
          state={state}
          move={makeMove}
          player={player}
          data={data}
        ></Board>

        <Player
          enabled={player === "black"}
          name={"Player 2"}
          missing={whiteMissing}
        ></Player>
      </div>
    </div>
  );
}

export default Home;
