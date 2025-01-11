use std::cell::RefCell;
use std::collections::{HashMap, LinkedList};
use std::fmt::Debug;
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::Arc;

use parking_lot::Mutex;

use crate::components::framework::animation::AnimationRegistry;
use crate::components::framework::element::ui_traits::{random_id, UIHandler, UIIdentifier};
use crate::components::framework::event::{Event, RenderPass};

/// An easy way to handle the UI representation of any sort of collections.
///
/// Given the necessary closures, it will handle adding, removing, and event dispatching of any sub-handlers.
///
/// As an example, it can be used to represent a collection of users (such as a HashMap).
///
/// ### [`Item`]
/// [`Item`] is the thing that is being represented, and only needs to implement [`UIIdentifier`]. The value returned by [`ui_id()`]
/// is only used by this single [`CompElement`]. This way there could be many different representations of the same [`Item`] across
/// multiple [`CompElement`].
///
/// In the example of representing a collection of users with [`UIIdentifier`] implemented, [`ui_id()`]
/// could simply return the ID of the user.
///
/// ### [`State`]
/// [`State`] can be anything, but should be used in the process of creating a [`UIHandler`] which is returned from [`New`].
/// Something like a position, index, etc.
///
/// ### [`New`]
/// [`New`] is the closure that provides a new object of type `dyn UIHandler`. This handler will be the UI representation
/// of the [`Item`] provided. [`New`] accepts `(bool, &mut State, &mut Item)`. The bool is if the [`Item`] already has
/// an element to represent it locally in this [`CompElement`]
///
/// ### [`IterFn`]
/// The closure [`IterFn`] provides the functionality of any kind of iterator, without copying or cloning any values,
/// by being structured in a way that the closure provided by the [`IterFn`] type is called on every iteration in an external iteration.
/// The closure handles the calling of [`New`] and updating of the element list.
///
/// Similar to generators.
///
/// # Examples
///
/// ```
/// use std::hash::{DefaultHasher, Hasher};
/// use std::sync::{Arc, Mutex};
/// use RustUI::components::framework::element::comp_element::CompElement;
/// use RustUI::components::framework::element::ElementBuilder;
/// use RustUI::components::framework::element::ui_traits::{UIHandler, UIIdentifier};
/// use RustUI::components::spatial::vec2::Vec2;
///
/// struct User {
///     name: String
/// }
///
/// impl UIIdentifier for User {
///     fn ui_id(&self) -> u64 {
///         let mut hasher = DefaultHasher::new();
///         hasher.write(self.name.as_bytes());
///         hasher.finish()
///     }
/// }
///
/// // Could also be a HashMap, HashSet, wrapped in Arc<Mutex<...>>, etc.
/// let mut users = vec![
///     User { name: "user_1".to_string() },
///     User { name: "user_2".to_string() }
/// ];
///
/// let users_list = CompElement::new(
///     // IterFn
///     move |mut inner| { // 'inner' is the provided closure that must be called
///         let mut i = 0;
///         let mut state = Vec2::new(0.0, 0.0);
///
///         // Any method of iterating can work, as long as inner() is called,
///         // such as situations where an Arc<Mutex<...>> must be locked.
///         for item in &mut users {
///             inner(&mut state, item);
///             i += 1;
///
///             // Because state can be anything, a Vec2 can be used
///             // to arrange the list of usernames in a grid pattern
///             state.set_x((i % 10 * 20) as f32);
///             state.set_y(((i / 10) * 20) as f32);
///         }
///     },
///     // New
///     move |exists, state, item| {
///         let state_c = *state;
///         let username = item.name.to_string();
///         if !exists {
///             Some(
///                 (Box::new(ElementBuilder::new()
///                     .handler(move |el, e| {
///                         // handle the events
///                     })
///                     .build()
///                 ) as Box<dyn UIHandler>, 0)
///             )
///         } else {
///             None
///         }
///     }
/// );
/// ```
///
/// [`ui_id()`]: UIIdentifier::ui_id
pub struct CompElement<IterFn, State, Item, New> {
    id: u64,
    elements: HashMap<u64, Box<dyn UIHandler>>,
    render_order: Vec<u64>,
    changed: bool,
    iter_fn: IterFn,
    item_construct: Arc<Mutex<New>>,
    _phantom: PhantomData<(State, Item)>,
}

impl<IterFn, State, Item, New> CompElement<IterFn, State, Item, New>
    where IterFn: FnMut(Box<dyn for<'a> FnMut(&mut State, &'a mut Item)>),
          New: FnMut(bool, &mut State, &mut Item) -> Option<(Box<dyn UIHandler>, u64)> + 'static,
          Item: UIIdentifier,
{
    pub fn new(iter_fn: IterFn, item_construct: New) -> Self {

        CompElement {
            id: random_id(),
            elements: HashMap::new(),
            render_order: Vec::new(),
            changed: true,
            iter_fn,
            item_construct: Arc::new(Mutex::new(item_construct)),
            _phantom: PhantomData::default(),
        }
    }

    pub fn update_elements(&mut self) {
        let mut new_elements = Rc::new(RefCell::new(HashMap::new()));
        let mut elements = Rc::new(RefCell::new(std::mem::take(&mut self.elements)));
        let changed = Rc::new(RefCell::new(false));
        let render_order = Rc::new(RefCell::new(Vec::new()));

        let cons = self.item_construct.clone();

        let c_elements = elements.clone();
        let c_new_elements = new_elements.clone();
        let c_changed = changed.clone();
        let c_render_order = render_order.clone();

        (self.iter_fn)(Box::new(move |state, item| {
            let id = item.ui_id();
            let exists = c_elements.borrow().contains_key(&id) || c_new_elements.borrow().contains_key(&id);

            let (id, el) = match (cons.lock())(exists, state, item) {
                None => (id, c_elements.borrow_mut().remove(&id)),
                Some((el, order)) => {
                    (*c_changed.borrow_mut()) = true;
                    (id, Some(el))
                },
            };

            match el {
                None => {},
                Some(new) => {
                    c_new_elements.borrow_mut().insert(id, new);
                }
            }
        }));

        self.changed = Rc::into_inner(changed).unwrap().into_inner();
        let old_elements = Rc::into_inner(elements).unwrap().into_inner();
        if old_elements.len() > 0 {
            self.changed = true;
        }

        let new_elements = Rc::into_inner(new_elements).unwrap().into_inner();
        std::mem::replace(&mut self.elements, new_elements);

        let new_ord = Rc::into_inner(render_order).unwrap().into_inner();
        std::mem::replace(&mut self.render_order, new_ord);
    }
}

impl<IterFn, State, Item, New> UIHandler for CompElement<IterFn, State, Item, New>
    where IterFn: FnMut(Box<dyn for<'a> FnMut(&mut State, &'a mut Item)>),
          New: FnMut(bool, &mut State, &mut Item) -> Option<(Box<dyn UIHandler>, u64)> + 'static,
          Item: UIIdentifier,
{
    unsafe fn handle(&mut self, event: &Event) -> bool {
        match event {
            Event::PreRender => {
                self.update_elements();
            }
            _ => {}
        }

        let mut handled = false;
        for el in self.elements.values_mut() {
            handled = el.handle(event);
        }

        match event {
            Event::PostRender => {
                self.changed = false;
            }
            _ => {}
        }

        handled
    }

    unsafe fn should_render(&mut self, render_pass: &RenderPass) -> bool {
        if self.changed {
            return true;
        }
        for el in self.elements.values_mut() {
            if el.should_render(render_pass) {
                return true;
            }
        }
        false
    }

    fn animations(&mut self) -> Option<&mut AnimationRegistry> {
        None // TODO
    }
}