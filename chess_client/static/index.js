Rust.chess_client.then(client => {
    const test = document.querySelector("#test");
    console.log(client.webasm(["Rook", "Pawn"], 1));
});
