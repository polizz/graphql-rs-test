use paste::paste;
use slab::Slab;
use std::sync::Arc;

macro_rules! model {
    ($model:ident, $container:ident) => {
        pub struct $model {
            pub item_id: u64,
            pub product_id: u64,
            pub category_id: u64,
            pub title: String,
        }

        paste! {
            pub struct $container {
                pub data: Slab<$model>,
            }
        }

        paste! {
            impl $container {
                pub fn new() -> Arc<Self> {
                  let mut data = Slab::new();

                    let _p1 = data.insert($model {
                        item_id: 1,
                        product_id: 1,
                        category_id: 1,
                        title: "Lemons".into(),
                    });

                    let _p2 = data.insert($model {
                        item_id: 2,
                        product_id: 2,
                        category_id: 2,
                        title: "Apples".into(),
                    });

                    Arc::new($container { data })
                }
              }
        }
    };
}

model! { Category, CategoriesContainer }
model! { Product, ProductsContainer }
model! { Item, ItemsContainer }
