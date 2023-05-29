pub fn print_to_icp(text : String){
    ic_cdk::println!("{}",text);
}
pub  fn icp_time() -> i64{
    return ic_cdk::api::time() as i64;
}