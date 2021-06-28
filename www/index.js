// import the generated JS bindings to the WASM library
import { Game } from "tictactoe";

const init = () => {
    // "Move" button click handler: make a move using the AI
    document.getElementById("ai").onclick = () => {
        const difficulty = document.getElementById("ai-difficulty");
        ai_move(parseInt(difficulty.value))
    }

    // clicking any field attempts to make a move in that field
    for(const row of [0, 1, 2]) {
        for(const column of [0, 1, 2]) {
            const id = "r" + (row+1) + "c" + (column+1);
            document.getElementById(id).onclick = () => { move(row, column) };
        }
    }

    // the "Restart" button just restarts the game
    document.getElementById("restart").onclick = restart;
}

// restart the game and redraw the board
const restart = () => {
    game = Game.new();
    render();
}

// try to make a move, if successful, redraw the board
const move = (row, column) => {
    if(game.do_move(row, column)) {
        render();
    }
}

// try to make a move with the AI at a specified difficulty level
const ai_move = (difficulty) => {
    if(game.do_ai_move(difficulty)) {
        render();
    } else {
        alert("Could not get AI move")
    }
}

// redraw the game UI
const render = () => {
    const board = game.get_board();
    const fields = [
        "r1c1",
        "r1c2",
        "r1c3",
        "r2c1",
        "r2c2",
        "r2c3",
        "r3c1",
        "r3c2",
        "r3c3",
    ];

    fields.forEach((val, i) => {
        document.getElementById(val).textContent = String.fromCharCode(board[i]);
    })

    document.getElementById("status").textContent = game.get_state();
    if(game.game_over()) {
        document.getElementById("restart").style.display = "block";
    } else {
        document.getElementById("restart").style.display = "none";
    }
}

let game;
init();
restart();