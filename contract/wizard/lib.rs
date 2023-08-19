#![cfg_attr(not(feature = "std"), no_std, no_main)]

        
#[openbrush::implementation(PSP34, PSP34Ownable,PSP34Mintable,PSP34Enumerable,PSP34Metadata)]
#[openbrush::contract]
pub mod psp34 {
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct wizard {
    	#[storage_field]
		psp34: psp34::Data<Balances>,
		#[storage_field]
		ownable: ownable::Data,
		#[storage_field]
		metadata: metadata::Data,
    }
    
    impl wizard {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut _instance = Self::default();
			ownable::Internal::_init_with_owner(&mut _instance Self::env().caller());
			psp34::Internal::_mint_to(&mut _instance, Self::env().caller(), Id::U8(1)).expect("Can mint");
			let collection_id = _instance.collection_id();
			metadata::Internal::_set_attribute(&mut _instance, collection_id.clone(), String::from("name"), String::from("PSP34"));
			metadata::Internal::_set_attribute(&mut _instance, collection_id, String::from("symbol"), String::from("MPSP"));
			_instance
        }
    }
}