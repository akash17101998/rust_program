use match_concept::Coin;

mod shadow_concept;
mod ownership_concept;
mod reference_concept;
mod struct_concept;
mod enum_concept;
mod match_concept;
fn main() {
   shadow_concept::shadow_func();
   ownership_concept::owner_func();
   reference_concept::reference_func();
   struct_concept::struct_func();
   struct_concept::area_of_rectangle();
   enum_concept::enum_func();
   match_concept::match_func(Coin::Penny);
   match_concept::new_func();

}
