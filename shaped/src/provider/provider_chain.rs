use super::Provider;

pub trait ProviderChain<Other>
where
    Other: Provider,
{
    type Return;
    fn chain(self, other: Other) -> Self::Return;
}

#[allow(non_snake_case)]
#[doc(hidden)]
pub struct Tuple2Provider<A: Provider, B: Provider> {
    A: A,
    B: B,
}

impl<A: Provider, B: Provider> Provider for Tuple2Provider<A, B>
where
    <A as Provider>::Item: Clone,
    <B as Provider>::Item: Clone,
{
    type Item = (<A as Provider>::Item, <B as Provider>::Item);

    #[allow(non_snake_case)]
    fn provide(&self) -> Vec<Self::Item> {
        let mut result = Vec::new();
        for A in self.A.provide() {
            for B in self.B.provide() {
                result.push((A.clone(), B.clone()));
            }
        }
        result
    }
}

impl<T, ConcreteProvider, Other> ProviderChain<Other> for ConcreteProvider
where
    ConcreteProvider: Provider<Item = (T,)>,
    Other: Provider,
    T: Sized + Send,
{
    type Return = Tuple2Provider<Self, Other>;

    #[allow(non_snake_case)]
    fn chain(self, B: Other) -> Self::Return {
        Tuple2Provider { A: self, B }
    }
}

macro_rules! impl_provider_chain {
    ($previous:ident, $name:ident, ($($ident:ident),*), $last:ident) => {
        #[allow(non_snake_case)]
        #[doc(hidden)]
        pub struct $name<$($ident: Provider,)* $last: Provider> {
            $($ident: $ident,)*
            $last: $last,
        }

        impl<$($ident: Provider,)* $last: Provider> Provider for $name<$($ident,)* $last>
        where
            $(<$ident as Provider>::Item: Clone,)*
            <$last as Provider>::Item: Clone,
        {
            type Item = (
                $(<$ident as Provider>::Item,)*
                <$last as Provider>::Item,
            );

            #[allow(non_snake_case)]
            fn provide(&self) -> Vec<Self::Item> {
                let mut result = Vec::new();
                impl_provider_chain!(@create-for self, result, (), $($ident,)* $last);
                result
            }
        }

        impl<$($ident: Provider,)* Other> ProviderChain<Other> for $previous<$($ident),*>
        where
            Other: Provider,
        {
            type Return = $name<$($ident,)* Other>;

            #[allow(non_snake_case)]
            fn chain(self, $last: Other) -> Self::Return {
                let $previous { $($ident),* } = self;
                $name {
                    $($ident,)*
                    $last,
                }
            }
        }
    };
    (@create-for $self: ident, $result:ident, ($($stacked:ident),*), $first:ident, $($ident:ident),*) => {
        for $first in $self.$first.provide() {
            impl_provider_chain!(@create-for $self, $result, ($($stacked,)* $first), $($ident),*)
        }
    };
    (@create-for $self:ident, $result:ident, ($($stacked:ident),*), $last:ident) => {
        for $last in $self.$last.provide() {
            $result.push(($($stacked.clone(),)* $last.clone()));
        }
    };
}

impl_provider_chain!(Tuple2Provider, Tuple3Provider, (A, B), C);
impl_provider_chain!(Tuple3Provider, Tuple4Provider, (A, B, C), D);
impl_provider_chain!(Tuple4Provider, Tuple5Provider, (A, B, C, D), E);
impl_provider_chain!(Tuple5Provider, Tuple6Provider, (A, B, C, D, E), F);
impl_provider_chain!(Tuple6Provider, Tuple7Provider, (A, B, C, D, E, F), G);
impl_provider_chain!(Tuple7Provider, Tuple8Provider, (A, B, C, D, E, F, G), H);
impl_provider_chain!(Tuple8Provider, Tuple9Provider, (A, B, C, D, E, F, G, H), I);
