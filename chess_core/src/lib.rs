#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate stdweb;

extern crate serde;

#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum Player {
    White,
    Black
}

js_serializable!(Player);
js_deserializable!(Player);

#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum Figure {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

js_serializable!(Figure);
js_deserializable!(Figure);
