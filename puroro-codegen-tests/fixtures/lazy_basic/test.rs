//! Eager and lazy aliases, follow-parent children, and a field-level lazy pin.

use crate::lazy_basic::{Child, ChildLazy, Note, NoteLazy};
use ::allocator_api2::alloc::Global;
use puroro::{Message, RepeatedStringMut, StringMut};

#[test]
fn eager_round_trip_and_lazy_pin() {
    let mut note = Note::new();
    *note.score_mut() = 7;
    note.title_mut().push_str("hi");
    *note.child_mut().n_mut() = 1;
    note.set_pinned({
        let mut child = Child::new();
        *child.n_mut() = 2;
        child
    });
    note.labels_mut().push().push_str("a");
    note.labels_mut().push().push_str("b");

    assert_eq!(note.score(), 7);
    assert_eq!(note.title(), "hi");
    assert_eq!(note.child().unwrap().n(), 1);
    assert_eq!(note.pinned().unwrap().n().unwrap(), 2);
    assert_eq!(note.labels().len(), 2);

    let decoded = Note::decode(note.encode_to_vec().as_slice()).unwrap();
    assert_eq!(decoded, note);
}

#[test]
fn lazy_getters_match_into_eager() {
    let mut note = Note::new();
    *note.score_mut() = 7;
    note.title_mut().push_str("hi");
    *note.child_mut().n_mut() = 1;
    note.set_pinned({
        let mut child = Child::new();
        *child.n_mut() = 2;
        child
    });
    note.labels_mut().push().push_str("a");

    let lazy = NoteLazy::decode(note.encode_to_vec().as_slice()).unwrap();
    assert_eq!(lazy.score().unwrap(), 7);
    assert_eq!(lazy.title().unwrap().get(), "hi");
    assert_eq!(lazy.child().unwrap().unwrap().n().unwrap(), 1);
    assert_eq!(lazy.pinned().unwrap().unwrap().n().unwrap(), 2);
    let labels = lazy.labels().unwrap();
    assert_eq!(labels.len(), 1);
    assert_eq!(&*labels[0], "a");
    assert_eq!(lazy.into_eager().unwrap(), note);
}

fn _pinned_is_lazy(slot: &mut ChildLazy<Global>) {
    let _ = slot;
}

fn _child_is_eager(slot: &mut Child<Global>) {
    let _ = slot;
}

#[test]
fn eager_parent_stores_pinned_lazy_child() {
    let mut note = Note::new();
    _child_is_eager(note.child_mut());
    _pinned_is_lazy(note.pinned_mut());
}
