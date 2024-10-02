// impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut minprice = i32::MAX;
        println!("Min prize is : {}",minprice);
        let mut maxprofit = 0;

        for pay in prices {
            if pay < minprice {
                minprice = pay;
            }
         
            let profit = pay - minprice;
        
            if profit > maxprofit {
                maxprofit = profit;
            }
        }

  
        maxprofit
    }
// }

fn main() {
    let prices = vec![7, 1, 5, 3, 6, 4];
    let result = max_profit(prices);
    println!("Maximum profit: {}", result);
}
