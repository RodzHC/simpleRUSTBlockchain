


struct Blockchain<B,T>{
    chain:Vec<B>,
    transactions:Vec<T>
}
struct Block{
    index:u128,
    timestamp:String
}
struct Transaction{
    sender:String,
    receiver:String,
    amount:u128
}


impl Blockchain<Block,Transaction>{
    fn new() -> Self{
        Self{
            chain:Vec::new(),
            transactions:Vec::new()
        }
    }
    fn create_block(&mut self)-> Block {
        let B = Block {
            index:1,
            timestamp:"teste".to_string()

        };
         self.chain.push(B);  
         B


    }
    fn add_transaction(&mut self)-> Transaction {
        let T = Transaction {
            sender:"Sender1".to_string(),
            receiver:"Receiver2".to_string(),
            amount:1000
        };
         self.transactions.push(T);  
         T
         

    }
} 

fn main() {
    println!("Hello, world!");

    Blockchain::new();

}

