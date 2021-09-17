use pgx::*;
use serde::{Deserialize, Serialize};
use std::collections::BinaryHeap;
use std::cmp::Reverse;

pg_module_magic!();

#[derive(Serialize, Deserialize, PostgresType)]
pub struct TopState {
    min_heap:  BinaryHeap<Reverse<i32>>,
    max_heap:  BinaryHeap<i32>,
    n:usize,
}
impl Default for TopState {
    fn default() -> Self {
        Self { min_heap:BinaryHeap::new(),max_heap:BinaryHeap::new(),n:10 }
    }
}
impl TopState {
    fn acc(& mut self, v: i32)  {
        if self.min_heap.len()<self.n{
            self.min_heap.push(Reverse(v));
            return 
        }

        // 取出当前最小堆上的最小值
        let top = self.min_heap.peek().unwrap().0;
        // 如果比最小值还小 , 肯定不会是要求的最大的 10 个数值, 直接丢弃
        if v<=top{
            return 
        }

        // 插入到最小堆中，然后移除堆中的最小值
        self.min_heap.push(Reverse(v));
        self.min_heap.pop().unwrap();
        return 
       
    }
    fn acc_max(& mut self, v: i32)  {
        if self.max_heap.len()<self.n{
            self.max_heap.push(v);
            return 
        }

        // 取出当前最大堆上的最大值
        let top = self.max_heap.peek().unwrap();
        // 如果比最大值还大 , 肯定不会是要求的最小的 10 个数值, 直接丢弃
        if v>=*top{
            return 
        }

        // 插入到最大堆中，然后移除堆中的最大值
        self.max_heap.push(v);
        self.max_heap.pop().unwrap();
        return 
       
    }

}

#[pg_extern]
fn integer_topn_state_func(
   mut internal_state: TopState,
    next_data_value: i32,
) -> TopState {
    internal_state.acc(next_data_value);
    internal_state.acc_max(next_data_value);
    internal_state
}

#[pg_extern]
fn integer_topn_final_func(internal_state: TopState) -> Vec<Vec<i32>> {
    vec![
    internal_state.min_heap.into_sorted_vec().iter().map(|x|x.0).collect(),
    internal_state.max_heap.into_sorted_vec(),
    ]
}

extension_sql!(
    r#"
    CREATE AGGREGATE MYMAXN (integer)
    (
        sfunc = integer_topn_state_func,
        stype = TopState,
        finalfunc = integer_topn_final_func,
        initcond = '{"min_heap":[],"max_heap":[],"n":10}'
    );
    "#
);

#[cfg(any(test, feature = "pg_test"))]
mod tests {
    use pgx::*;

    #[pg_test]
    fn test_hello_my_topn() {
        // assert_eq!("Hello, my_topn", crate::hello_my_topn());
    }

}

#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}





