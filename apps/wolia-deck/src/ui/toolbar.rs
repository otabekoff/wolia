//! Deck toolbar.

/// Deck toolbar actions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeckAction {
    // Slides
    NewSlide,
    DuplicateSlide,
    DeleteSlide,

    // Insert
    InsertText,
    InsertImage,
    InsertShape,
    InsertTable,
    InsertChart,
    InsertVideo,

    // Format
    Format,

    // Arrange
    BringForward,
    SendBackward,
    AlignObjects,
    DistributeObjects,

    // Presentation
    StartPresentation,
    StartFromCurrent,
    PresenterView,

    // Animations
    AddAnimation,
    AnimationPane,
    TransitionPane,
}
