fn main() {
   let i1:i32 = false.into();
   let i2:i32 = i32::from(false);  
   assert_eq!(i1, i2);
   assert_eq!(i1, 0);
   let i3: u32 = 'a'.into();
   let s: String = 'a'.into();
}