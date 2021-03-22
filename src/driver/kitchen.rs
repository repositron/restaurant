use std::time::Duration;
use actix::prelude::*;
use actix::utils::IntervalFunc;
use std::sync::Mutex;

use crate::models::Order;

pub struct Kitchen {

}

impl Kitchen {

    pub fn add_order(&mut self, order: Order) {

    }

    pub fn cook(&mut self, context: &mut Context<Self>) {


    }
}

impl Actor for Kitchen {
    type Context = Context<Self>;
    fn started(&mut self, context: &mut Context<Self>) {
        IntervalFunc::new(Duration::from_secs(1), Self::cook)
            .finish()
            .spawn(context);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let k = Kitchen {};

    }


}