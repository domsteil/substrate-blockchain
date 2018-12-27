// Encoding library
use parity_codec::Encode;

// Enables access to the runtime storage
use srml_support::{StorageValue, dispatch::Result};

// Enables us to do hashing
use runtime_primitives::traits::Hash;

// Enables access to account balances and interacting with signed messages
use {balances, system::{self, ensure_signed}};





decl_module! {
  pub struct Module<T: Trait> for enum Call where origin: T::Origin {
    fn iou(origin) -> Result {
      // Logic for an iou
    }

    fn set_iou_amount(_origin, value: T::Balance) -> Result {
      // Logic for setting the iou amount
    }
  }
}


fn iou(origin) -> Result {
  // Ensure we have a signed message, and derive the sender's account id from the signature
  let sender = ensure_signed(origin)?;
  
  // Here we grab the iou, and put it into a local variable.
  // We are able to use Self::iou() because we defined it in our decl_storage! macro above
  // If there is no iou, exit with an error message
  let iou = Self::iou().ok_or("Must have iou amount set")?;

  // First, we decrease the balance of the sender by the iou amount using the balances module
  <balances::Module<T>>::decrease_free_balance(&sender, iou)?;

  // New IOU Logic goes here ------>


  
  // Then we flip a coin by generating a random seed
  // We pass the seed with our sender's account id into a hash algorithm
  // Then we check if the first byte of the hash is less than 128
  if (<system::Module<T>>::random_seed(), &sender)
  .using_encoded(<T as system::Trait>::Hashing::hash)
  .using_encoded(|e| e[0] < 128)
  {
    // If the sender wins the coin flip, we increase the sender's balance by the pot amount
    // `::take()` will also remove the pot amount from storage, which by default will give it a value of 0
    <balances::Module<T>>::increase_free_balance_creating(&sender, <Pot<T>>::take());
  }

  // No matter the outcome, we will add the original sender's payment back into the pot
  <Obligation<T>>::mutate(|obligation| *obligation += iou);

  Ok(())
}



fn set_iou_amount(_origin, value: T::Balance) -> Result {
  //If the iou has not been set...
  if Self::iou().is_none() {
    // ... we will set it to the value we passed in.
    <Iou<T>>::put(value);
    
    // We will also put that initial value into the pot for someone to win
    <Obligation<T>>::put(value);
  }
  
  Ok(())
}


decl_storage! {
  trait Store for Module<T: Trait> as Demo {
    Iou get(iou) config(): Option<T::Balance>;
    Obligation get(obligation): T::Balance;
  }
}`