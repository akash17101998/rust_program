pub mod shadow_concept;
pub mod ownership_concept;
pub mod reference_concept;
pub mod struct_concept;
fn main() {
   shadow_concept::shadow_func();
   ownership_concept::owner_func();
   reference_concept::reference_func();
   struct_concept::struct_func();
   struct_concept::area_of_rectangle();

}
