

mod tdudec;

fn main() {
	test(false);	
	test(true);	
}

fn test(save:bool) {
	let path = if save{
		"playersave"
	}else{
		"AIConfig.xml"
	};

	let encrypted_content = match std::fs::read(path){
		Ok(f) => f,
		Err(e) => panic!("cannot read file, {}", e),
	};

	let d = if save{
		let decrypted_content = tdudec::decrypt_save(&encrypted_content);
		let encrypted_content = tdudec::encrypt_save(&decrypted_content);
		tdudec::decrypt_save(&encrypted_content)
	}else{
		let decrypted_content = tdudec::decrypt_others(&encrypted_content);
		let encrypted_content = tdudec::encrypt_others(&decrypted_content);
		tdudec::decrypt_others(&encrypted_content)
	};

	match std::fs::write(format!("{}.plain", path), d){
		Ok(_) => (),
		Err(e) => panic!("cannot write plain text, {}", e),
	};
}
