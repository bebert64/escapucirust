pub(crate) mod hall_face_down;
pub(crate) mod hall_face_up;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) enum Rooms {
    HallFaceUp,
    HallFaceDown,
}
