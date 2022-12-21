import { render, screen } from "@testing-library/react";
import Board from "./board";
import { board } from "../../definitions";

describe("Testing that chess board is rendered with all pieces", () => {
  it("should display 64 tiles with all relevant pieces", () => {
    render(
      <Board
        data={{ check: false, mate: false, valid: true }}
        state={board}
        timer={false}
        move={() => {}}
        player={"white"}
        wMissing={0}
        bMissing={0}
      />
    );

    // All piece alts should exist in Row (A - H) - 1
    expect(screen.getByAltText("a1")).toBeInTheDocument();
    expect(screen.getByAltText("a1")).toBeVisible();

    expect(screen.getByAltText("b1")).toBeInTheDocument();
    expect(screen.getByAltText("b1")).toBeVisible();

    expect(screen.getByAltText("c1")).toBeInTheDocument();
    expect(screen.getByAltText("c1")).toBeVisible();

    expect(screen.getByAltText("d1")).toBeInTheDocument();
    expect(screen.getByAltText("d1")).toBeVisible();

    expect(screen.getByAltText("e1")).toBeInTheDocument();
    expect(screen.getByAltText("e1")).toBeVisible();

    expect(screen.getByAltText("f1")).toBeInTheDocument();
    expect(screen.getByAltText("f1")).toBeVisible();

    expect(screen.getByAltText("g1")).toBeInTheDocument();
    expect(screen.getByAltText("g1")).toBeVisible();

    expect(screen.getByAltText("h1")).toBeInTheDocument();
    expect(screen.getByAltText("h1")).toBeVisible();

    // All piece alts should exist in Row (A - H) - 2
    expect(screen.getByAltText("a2")).toBeInTheDocument();
    expect(screen.getByAltText("a2")).toBeVisible();

    expect(screen.getByAltText("b2")).toBeInTheDocument();
    expect(screen.getByAltText("b2")).toBeVisible();

    expect(screen.getByAltText("c2")).toBeInTheDocument();
    expect(screen.getByAltText("c2")).toBeVisible();

    expect(screen.getByAltText("d2")).toBeInTheDocument();
    expect(screen.getByAltText("d2")).toBeVisible();

    expect(screen.getByAltText("e2")).toBeInTheDocument();
    expect(screen.getByAltText("e2")).toBeVisible();

    expect(screen.getByAltText("f2")).toBeInTheDocument();
    expect(screen.getByAltText("f2")).toBeVisible();

    expect(screen.getByAltText("g2")).toBeInTheDocument();
    expect(screen.getByAltText("g2")).toBeVisible();

    expect(screen.getByAltText("h2")).toBeInTheDocument();
    expect(screen.getByAltText("h2")).toBeVisible();

    // All piece alts should exist in Row (A - H) - 8
    expect(screen.getByAltText("a8")).toBeInTheDocument();
    expect(screen.getByAltText("a8")).toBeVisible();

    expect(screen.getByAltText("b8")).toBeInTheDocument();
    expect(screen.getByAltText("b8")).toBeVisible();

    expect(screen.getByAltText("c8")).toBeInTheDocument();
    expect(screen.getByAltText("c8")).toBeVisible();

    expect(screen.getByAltText("d8")).toBeInTheDocument();
    expect(screen.getByAltText("d8")).toBeVisible();

    expect(screen.getByAltText("e8")).toBeInTheDocument();
    expect(screen.getByAltText("e8")).toBeVisible();

    expect(screen.getByAltText("f8")).toBeInTheDocument();
    expect(screen.getByAltText("f8")).toBeVisible();

    expect(screen.getByAltText("g8")).toBeInTheDocument();
    expect(screen.getByAltText("g8")).toBeVisible();

    expect(screen.getByAltText("h8")).toBeInTheDocument();
    expect(screen.getByAltText("h8")).toBeVisible();

    // All piece alts should exist in Row (A - H) - 7
    expect(screen.getByAltText("a7")).toBeInTheDocument();
    expect(screen.getByAltText("a7")).toBeVisible();

    expect(screen.getByAltText("b7")).toBeInTheDocument();
    expect(screen.getByAltText("b7")).toBeVisible();

    expect(screen.getByAltText("c7")).toBeInTheDocument();
    expect(screen.getByAltText("c7")).toBeVisible();

    expect(screen.getByAltText("d7")).toBeInTheDocument();
    expect(screen.getByAltText("d7")).toBeVisible();

    expect(screen.getByAltText("e7")).toBeInTheDocument();
    expect(screen.getByAltText("e7")).toBeVisible();

    expect(screen.getByAltText("f7")).toBeInTheDocument();
    expect(screen.getByAltText("f7")).toBeVisible();

    expect(screen.getByAltText("g7")).toBeInTheDocument();
    expect(screen.getByAltText("g7")).toBeVisible();

    expect(screen.getByAltText("h7")).toBeInTheDocument();
    expect(screen.getByAltText("h7")).toBeVisible();
  });
});
