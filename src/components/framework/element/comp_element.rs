use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::Arc;

use parking_lot::Mutex;

use crate::components::framework::animation::AnimationRegistry;
use crate::components::framework::element::ui_traits::{random_id, UIHandler, UIIdentifier};
use crate::components::framework::event::{Event, RenderPass};

pub struct CompElement<IterFn, State, Item, Cons> {
    id: u64,
    elements: HashMap<u64, Box<dyn UIHandler>>,
    changed: bool,
    iter_fn: IterFn,
    item_construct: Arc<Mutex<Cons>>,
    _phantom: PhantomData<(State, Item)>,
}

impl<IterFn, State, Item, Cons> CompElement<IterFn, State, Item, Cons>
    where IterFn: FnMut(Box<dyn for<'a> FnMut(&mut State, &'a mut Item)>),
          Cons: FnMut(bool, &mut State, &mut Item) -> Option<Box<dyn UIHandler>> + 'static,
          Item: UIIdentifier,
{
    pub fn new(iter_fn: IterFn, item_construct: Cons) -> Self {
        CompElement {
            id: random_id(),
            elements: HashMap::new(),
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

        let cons = self.item_construct.clone();

        let c_elements = elements.clone();
        let c_new_elements = new_elements.clone();
        let c_changed = changed.clone();
        (self.iter_fn)(Box::new(move |state, item| {
            let id = item.ui_id();
            let exists = c_elements.borrow().contains_key(&id) || c_new_elements.borrow().contains_key(&id);

            let (id, el) = match (cons.lock())(exists, state, item) {
                None => (id, c_elements.borrow_mut().remove(&id)),
                Some(el) => {
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
    }
}

impl<IterFn, State, Item, Cons> UIHandler for CompElement<IterFn, State, Item, Cons>
    where IterFn: FnMut(Box<dyn for<'a> FnMut(&mut State, &'a mut Item)>),
          Cons: FnMut(bool, &mut State, &mut Item) -> Option<Box<dyn UIHandler>> + 'static,
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