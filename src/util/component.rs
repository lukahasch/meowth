use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use serde::{Deserialize, Serialize};

use crate::{
    state::State,
    util::{Output, args::Arguments},
};

pub trait Component<M, A>
where
    M: Send + 'static,
{
    type Message: 'static + Send;

    fn update(&mut self, message: Self::Message, state: &State, args: Arguments<A>) -> Output<M>;
    fn view<'a>(&'a self, state: &'a State, args: Arguments<A>)
    -> iced::Element<'a, Self::Message>;
    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::Subscription::none()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Hash)]
pub struct Comp<M, C, A>
where
    C: Component<M, A>,
    M: Send + 'static,
{
    pub component: C,
    _phantom: PhantomData<M>,
}

impl<M, C> Comp<M, C>
where
    C: Component<M, A>,
    M: Send + 'static,
{
    pub fn new(component: C) -> Self {
        Comp {
            component,
            _phantom: PhantomData,
        }
    }
}

impl<M, C> Deref for Comp<M, C>
where
    C: Component<M, A>,
    M: Send + 'static,
{
    type Target = C;

    fn deref(&self) -> &Self::Target {
        &self.component
    }
}

impl<M, C> DerefMut for Comp<M, C>
where
    C: Component<M, A>,
    M: Send + 'static,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.component
    }
}

impl<M, C> From<C> for Comp<M, C>
where
    C: Component<M>,
    M: Send + 'static,
{
    fn from(component: C) -> Self {
        Comp::new(component)
    }
}

impl<M, C> Component<M> for Comp<M, C>
where
    C: Component<M>,
    M: Send + 'static,
{
    type Input<'a> = C::Input<'a>;
    type Message = C::Message;

    fn update(
        &mut self,
        message: Self::Message,
        state: &State,
        input: Self::Input<'_>,
    ) -> Output<M> {
        self.component.update(message, state, input)
    }

    fn view<'a>(
        &'a self,
        state: &'a State,
        input: Self::Input<'a>,
    ) -> iced::Element<'a, Self::Message> {
        self.component.view(state, input)
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        self.component.subscription()
    }
}
