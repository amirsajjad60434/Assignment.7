

mod company {
    pub mod department{
        pub fn employee (){
            println!("I am a IT Manager at IT Department");
            }
    }

}
fn main() {
    crate::company::department::employee();
}
