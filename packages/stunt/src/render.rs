use crate::component::tree::AttrMap;
use crate::component::state::{self, Identity};
use crate::component::{Component, Context};
use crate::vdom;

use spin::Mutex;

use std::marker::PhantomData;
use std::sync::Arc;


pub struct Renderer<T: Component + Send + Sync + 'static> {
    _component: PhantomData<T>,
}

impl<T: Component + Send + Sync + 'static> Renderer<T> {
    pub fn new() -> Renderer<T> {
        Renderer {
            _component: PhantomData,
        }
    }

    pub fn render(self) {
        state::get_or_insert(&Identity::new(0), || Arc::new(Mutex::new(T::create())));

        render();
    }
}

#[inline]
pub fn render() {
    let identity = Identity::new(0);

    let root = state::get(&identity);
    let lock = root.lock();

    let render = lock.base_view(Context::new(identity), AttrMap::from(Vec::new().into_iter())).render();

    vdom::reconcile(render);
}


