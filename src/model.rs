use async_graphql::{ComplexObject, SimpleObject};
use async_graphql::{Context, Object};
use std::sync::Arc;

use crate::data::{Category, Item, Product, ProductsContainer};

struct ProductsSchema;

// #[Object]
// impl ProductsSchema {
//     async fn product_by_id<'c>(&self, ctx: &Context<'c>) -> Result<Product, anyhow::Error> {
//         let products = ctx.data_unchecked::<crate::data::Products>();
//     }
// }

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn all_products<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<Vec<&'ctx String>, anyhow::Error> {
        let products = ctx.data_unchecked::<crate::data::ProductsContainer>();
        Ok(products.data.iter().map(|(_, c)| &c.title).collect())
    }

    // async fn product_by_id<'ctx>(
    //     &self,
    //     ctx: &Context<'ctx>,
    //     product_id: u64,
    // ) -> Result<&'ctx Product, anyhow::Error> {
    //     let products = ctx.data_unchecked::<crate::data::Products>();
    //     products
    //         .data
    //         .iter()
    //         .find(|&(_, &p)| p.product_id == product_id)
    //         .map(|m| m.1)
    //         .ok_or(Err("No product found"))
    //         .map_err(|e| e.into())
    // }
}
